use std::{fs::{self, File}, path::Path, result::Result, env, process::ExitCode, str, io::{BufReader, BufWriter}};
use xml::{EventReader, reader::XmlEvent, common::{TextPosition, Position}};

mod model_ep03;
use model_ep03::*;
mod server_ep03;

fn parse_entire_xml_file(file_path: &Path) -> Result<String, ()> {
    let file = File::open(file_path).map_err(|err| {
        eprintln!("ERROR: could not open file {file_path}: {err}", file_path = file_path.display());
    })?;
    let er = EventReader::new(BufReader::new(file));
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

fn save_model_as_json(model: &Model, index_path: &str) -> Result<(), ()> {
    println!("Saving {index_path}...");
    let index_file = File::create(index_path).map_err(|err| {
        eprintln!("ERROR: could not create index file {index_path}: {err}");
    })?;  

    serde_json::to_writer(BufWriter::new(index_file), &model).map_err(|err| {
        eprintln!("ERROR: could not serialize index into file {index_path}: {err}");
    })?;
    Ok(())
}

fn add_folder_to_model(dir_path: &Path, model: &mut Model) -> Result<(), ()> {
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
            add_folder_to_model(&file_path, model)?;
            continue 'next_file;
        }
        println!("Indexing {:?}...", &file_path);
        let content = match parse_entire_xml_file(&file_path) {
            Ok(content) => content.chars().collect::<Vec<_>>(),
            Err(()) => continue 'next_file,
        };

        let mut tf = TermFreq::new();
        
        let mut n = 0;
        for term in Lexer::new(&content){
            if let Some(freq) = tf.get_mut(&term) {
                *freq += 1;
            } else {
                tf.insert(term, 1);
            }
            n += 1;
        }

        for t in tf.keys() {
            if let Some(freq) = model.df.get_mut(t) {
                *freq += 1;
            } else {
                model.df.insert(t.to_string(), 1);
            }
        }
    
        model.tfpd.insert(file_path, (n, tf));
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
            let mut model = Default::default();
            add_folder_to_model(Path::new(&dir_path), &mut model)?;
            save_model_as_json(&model, "index.json")
        },
        "search" => {
            let index_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: no path to index was provided for {subcommand} subcommand");
            })?;
            let prompt = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: no search query was provided {subcommand} subcommand");
            })?.chars().collect::<Vec<_>>();
            let index_file = File::open(&index_path).map_err(|err| {
                eprintln!("ERROR: could not open index file {index_path}: {err}");
            })?; 
            let model:Model = serde_json::from_reader(index_file).map_err(|err| {
                eprintln!("ERROR: could not parse index file {index_path}: {err}");
            })?;
            for (path, rank) in search_query(&model, &prompt).iter().take(20) {
                println!("{path} {rank}", path = path.display())
            }
            Ok(())
        },
        "serve" => {
            let index_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: no path to index was provided for {subcommand} subcommand");
            })?;
            let index_file = File::open(&index_path).map_err(|err| {
                eprintln!("ERROR: could not open index file {index_path}: {err}");
            })?; 
            let model:Model = serde_json::from_reader(index_file).map_err(|err| {
                eprintln!("ERROR: could not parse index file {index_path}: {err}");
            })?;
            let address = args.next().unwrap_or("127.0.0.1:8080".to_string());
            server_ep03::start(&address, &model)
        }
        _ => {
            usage(&program);
            eprintln!("ERROR: unknown subcommand {subcommand}");
            return Err(());
        }
    }
}

fn main() -> ExitCode {
    match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
