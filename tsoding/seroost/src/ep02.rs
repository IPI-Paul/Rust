use std::{fs::{self, File}, path::{Path, PathBuf}, collections::HashMap, result::Result, env, process::ExitCode, str};
use tiny_http::{Server, Response, Header, Request, Method, StatusCode};
use xml::{EventReader, reader::XmlEvent, common::{TextPosition, Position}};

#[derive(Debug)]
struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        Self { content }
    }
    fn trim_left(&mut self) {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }
    fn chop(&mut self, n: usize) -> &'a [char] {
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        token
    }
    fn chop_while<P>(&mut self, mut predicate: P) -> &'a [char] where P: FnMut(&char) -> bool {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }
        self.chop(n)
    }
    fn next_token(&mut self) -> Option<String> {
        self.trim_left();
        if self.content.len() == 0 {
            return None
        }
        if self.content[0].is_numeric() {
            return Some(self.chop_while(|x| x.is_numeric()).iter().collect());
        }
        if self.content[0].is_alphabetic() {
            return Some(self.chop_while(|x| x.is_alphanumeric()).iter().map(|x| x.to_ascii_uppercase()).collect());
        }
        return Some(self.chop(1).iter().collect());
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn parse_entire_xml_file(file_path: &Path) -> Result<String, ()> {
    let file = File::open(file_path).map_err(|err| {
        eprintln!("ERROR: could not open file {file_path}: {err}", file_path = file_path.display());
    })?;
    let er = EventReader::new(file);
    let mut content = String::new();
    for event in er.into_iter() {
        let event = event.map_err(|err| {
            let TextPosition {row, column} = err.position();
            let msg = err.msg();
            eprintln!("{file_path}:{row}:{column}: ERROR: {msg}", file_path = file_path.display());
        })?;
        if let XmlEvent::Characters(text) = event {
            content.push_str(&text);
            content.push_str(" ");
        }
    }
    Ok(content)
}

type TermFreq = HashMap::<String, usize>;
type TermFreqIndex = HashMap<PathBuf, TermFreq>;

fn check_index(index_path: &str) -> Result<(), ()> {
    println!("Reading {index_path} index file...");
    let index_file = File::open(index_path).map_err(|err| {
        eprintln!("ERROR: could not open index file {index_path}: {err}");
    })?; 
    let tf_index:TermFreqIndex = serde_json::from_reader(index_file).map_err(|err| {
        eprintln!("ERROR: could not parse index file {index_path}: {err}");
    })?;
    println!("{index_path} contains {count} files", count = tf_index.len());
    Ok(())
}

fn save_tf_index(tf_index: &TermFreqIndex, index_path: &str) -> Result<(), ()> {
    println!("Saving {index_path}...");
    let index_file = File::create(index_path).map_err(|err| {
        eprintln!("ERROR: could not create index file {index_path}: {err}");
    })?;  

    serde_json::to_writer(index_file, &tf_index).map_err(|err| {
        eprintln!("ERROR: could not serialize index into file {index_path}: {err}");
    })?;
    Ok(())
}

fn tf_index_of_folder(dir_path: &Path, tf_index: &mut TermFreqIndex) -> Result<(), ()> {
    let dir = fs::read_dir(dir_path).map_err(|err| {
        eprintln!("ERROR: could not open directory {dir_path}: {err}", dir_path = dir_path.display());
    })?;
    'next_file: for file in dir {
        let file = file.map_err(|err| {
            eprintln!("ERROR: could not read next file in directory {dir_path} during indexing: {err}", dir_path = dir_path.display());
        })?;
        let file_path = file.path();
        let file_type = file.file_type().map_err(|err| {
            eprintln!("ERROR: could not determine type of file {file_path}: {err}", file_path = file_path.display());
        })?;
        if file_type.is_dir() {
            tf_index_of_folder(&file_path, tf_index)?;
            continue 'next_file;
        }
        println!("Indexing {:?}...", &file_path);
        let content = match parse_entire_xml_file(&file_path) {
            Ok(content) => content.chars().collect::<Vec<_>>(),
            Err(()) => continue 'next_file,
        };

        let mut tf = TermFreq::new();
        
        for term in Lexer::new(&content){
            if let Some(freq) = tf.get_mut(&term) {
                *freq += 1;
            } else {
                tf.insert(term, 1);
            }
        }
    
        tf_index.insert(file_path, tf);
    }
    Ok(())
}

fn usage(program: &str) {
    eprintln!("Usage: {program} [SUBCOMMAND] [OPTIONS]");
    eprintln!("Subcommands:");
    eprintln!("     index <folder>                  index the <folder> and save the index to index.json file");
    eprintln!("     search <index-file>             check how many documents are indexed in the file (searching is not implemented yet)");
    eprintln!("     serve <index-file> [address]    start local HTTP server with Web Interface");
}

fn serve_static_file(request: Request, file_path: &str, content_type: &str) -> Result<(), ()> {
    let content_type_header = Header::from_bytes("Content-Type", content_type)
        .expect("That we didn't put any garbage in the headers");
    let file = File::open(file_path).map_err(|err| {
        eprintln!("ERROR: could not serve file {file_path}: {err}");
    })?;
    let response = Response::from_file(file).with_header(content_type_header);
    request.respond(response).map_err(|err| {
        eprintln!("ERROR: could not server static file {file_path}: {err}");
    })
}

fn serve_404(request: Request) -> Result<(), ()> {
    request.respond(Response::from_string("404").with_status_code(StatusCode(404))).map_err(|err| {
        eprintln!("ERROR: could not serve a request: {err}");
    })
}

fn tf(t: &str, d: &TermFreq) -> f32 {
    let a = *d.get(t).unwrap_or(&0) as f32; 
    let b = d.iter().map(|(_, f)| *f).sum::<usize>() as f32;
    a / b
}

fn idf(t: &str, d: &TermFreqIndex) -> f32 {
    let n = d.len() as f32;
    let m = d.values().filter(|tf| tf.contains_key(t)).count().max(1) as f32;
    return (n / m).log10();
}

fn serve_request(tf_index: &TermFreqIndex, mut request: Request) -> Result<(), ()> {
    println!("INFO: received request! method: {:?}, url: {:?}",
        request.method(), request.url()
    );
    match (request.method(), request.url()) {
        (Method::Post, "/api/search") => {
            let mut buf = Vec::new();
            let _ = request.as_reader().read_to_end(&mut buf);
            let body = str::from_utf8(&buf).map_err(|err| {
                eprintln!("ERROR: could not interpret body as UTF-8 string: {err}");
            })?.chars().collect::<Vec<_>>();
            let mut result = Vec::<(&Path, f32)>::new();
            for (path, tf_table) in tf_index {
                let mut rank = 0.0;
                for token in Lexer::new(&body) {
                    rank += tf(&token, &tf_table) * idf(&token, &tf_index);
                }
                result.push((path, rank))
            }
            result.sort_by(|(_, rank1), (_, rank2)| rank1.partial_cmp(rank2).unwrap());
            result.reverse();
            for (path, rank) in result.iter().take(10) {
                println!("{path} => {rank}", path = path.display());
            }
            request.respond(Response::from_string("ok")).map_err(|err| {
                eprintln!("ERROR: {err}");
            })
        },
        (Method::Get, "/css/style.css") => {
            serve_static_file(request, "css/style.css", "text/css; charset=utf-8")
        },
        (Method::Get, "/scripts/index.js") => {
            serve_static_file(request, "scripts/index.js", "text/javascript; charset=utf-8")
        },
        (Method::Get, "/") => {
            serve_static_file(request, "index.html", "text/html; charset=utf-8")
        },
        _ => {
            serve_404(request)
        },
    }
}

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("path to program is provided");

    let subcommand = args.next().ok_or_else(|| {
        usage(&program);
        eprintln!("ERROR: no subcommand was provided");
    })?;
    match subcommand.as_str() {
        "index" => {
            let dir_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: no directory was provided for {subcommand} subcommand");
            })?;
            let mut tf_index = TermFreqIndex::new();
            tf_index_of_folder(Path::new(&dir_path), &mut tf_index)?;
            save_tf_index(&tf_index, "index.json")?;
        },
        "search" => {
            let index_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: no path to index was provided for {subcommand} subcommand");
            })?;
            check_index(&index_path)?;
        },
        "serve" => {
            let index_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: no path to index was provided for {subcommand} subcommand");
            })?;
            let index_file = File::open(&index_path).map_err(|err| {
                eprintln!("ERROR: could not open index file {index_path}: {err}");
            })?; 
            let tf_index:TermFreqIndex = serde_json::from_reader(index_file).map_err(|err| {
                eprintln!("ERROR: could not parse index file {index_path}: {err}");
            })?;
            let address = args.next().unwrap_or("127.0.0.1:8080".to_string());
            let server = Server::http(&address).map_err(|err| {
                eprintln!("ERROR: could not start HTTP server at {address}: {err}");
            })?;
            println!("INFO: listening at http://{address}/");
            for request in server.incoming_requests() {
                let _ = serve_request(&tf_index, request);
            }
        }
        _ => {
            usage(&program);
            eprintln!("ERROR: unknown subcommand {subcommand}");
            return Err(());
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
