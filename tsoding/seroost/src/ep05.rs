use std::{fs::{self, File}, path::{Path, PathBuf}, result::Result, env, process::ExitCode, str, io::{BufReader, BufWriter}, sync::{Mutex, Arc}, thread};
use xml::{EventReader, reader::XmlEvent, common::{TextPosition, Position}};

mod model_ep05;
use model_ep05::*;
mod server_ep05;
mod lexer_ep05;
mod snowball;

fn parse_entire_txt_file(file_path: &Path) -> Result<String, ()> {
    fs::read_to_string(file_path).map_err(|err| {
        eprintln!("ERROR: could not open file {file_path}: {err}", file_path = file_path.display());
    })
} 

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

fn parse_entire_file_by_extension(file_path: &Path) -> Result<String, ()> {
    let extension = file_path.extension().ok_or_else(|| {
        eprintln!("ERROR: can't detect file type of {file_path} without extension", file_path = file_path.display());
    })?.to_string_lossy();
    match extension.as_ref() {
        "xhtml" | "xml" => parse_entire_xml_file(file_path),
        "txt" | "md" => parse_entire_txt_file(file_path),
        _ => {
            eprintln!("ERROR: can't detect file type of {file_path}: unsupported extension {extension}",
                file_path = file_path.display()
            );
            Err(())
        }
    }
}

fn save_model_as_json(model: &InMemoryModel, index_path: &str) -> Result<(), ()> {
    println!("Saving {index_path}...");
    let index_file = File::create(index_path).map_err(|err| {
        eprintln!("ERROR: could not create index file {index_path}: {err}");
    })?;  

    serde_json::to_writer(BufWriter::new(index_file), &model).map_err(|err| {
        eprintln!("ERROR: could not serialize index into file {index_path}: {err}");
    })?;
    Ok(())
}

fn add_folder_to_model(dir_path: &Path, model: Arc<Mutex<InMemoryModel>>, skipped: &mut usize) -> Result<(), ()> {
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
        let last_modified = file.metadata().map_err(|err| {
            eprintln!("ERROR: could not get the metadata of file {file_path}: {err}", file_path = file_path.display());
        })?.modified().map_err(|err| {
            eprintln!("ERROR: could not get last modification date of file {file_path}: {err}", file_path = file_path.display());
        })?;
        if file_type.is_dir() {
            add_folder_to_model(&file_path, Arc::clone(&model), skipped)?;
            continue 'next_file;
        }
        let mut model = model.lock().unwrap();
        if model.requires_reindexing(&file_path, last_modified)? {
            println!("Indexing {:?}...", &file_path);
            let content = match parse_entire_file_by_extension(&file_path) {
                Ok(content) => content.chars().collect::<Vec<_>>(),
                Err(()) => {
                    *skipped += 1;
                    continue 'next_file;
                },
            };
    
            model.add_document(file_path, last_modified, &content)?;
        } else {
            println!("Ignoring {file_path} as it has not been modified", file_path = file_path.display());
            *skipped += 1;
        }
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


    let mut subcommand = None;
    let mut use_sqlite_mode = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "sqlite" => use_sqlite_mode = true,
            _ => {
                subcommand = Some(arg);
                break
            }
        }
    }

    let subcommand = subcommand.ok_or_else(|| {
        usage(&program);
        eprintln!("ERROR: no subcommand was provided");
    })?;
    match subcommand.as_str() {
        /*
        "reindex" => {
            assert!(!use_sqlite_mode, "The sqlite model is depracated");
            let dir_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: no directory was provided for {subcommand} subcommand");
            })?;
            let index_path = "index.json";
            let index_file = File::open(&index_path).map_err(|err| {
                eprintln!("ERROR: could not open index file {index_path}: {err}");
            })?; 
            let mut model: InMemoryModel = serde_json::from_reader(index_file).map_err(|err| {
                eprintln!("ERROR: could not parse index file {index_path}: {err}");
            })?;
            let mut skipped = 0;
            add_folder_to_model(Path::new(&dir_path), &mut model, &mut skipped)?;
            save_model_as_json(&model, index_path)?;
            println!("Skipped {skipped} files.");
            Ok(())
        },
        "index" => {
            let dir_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: no directory was provided for {subcommand} subcommand");
            })?;
            let mut skipped = 0;

            if use_sqlite_mode {
                let index_path = "index.db";
                if let Err(err) = fs::remove_file(index_path) {
                    if err.kind() != std::io::ErrorKind::NotFound {
                        eprintln!("ERROR: could not delete file {index_path}: {err}");
                        return Err(());
                    }
                }

                let mut model = SqliteModel::open(Path::new(index_path))?;
                model.begin()?;
                add_folder_to_model(Path::new(&dir_path), &mut model, &mut skipped)?;
                model.commit()?;
            } else {
                let index_path = "index.json";
                let mut model = Default::default();
                add_folder_to_model(Path::new(&dir_path), &mut model, &mut skipped)?;
                save_model_as_json(&model, index_path)?;
            }
            println!("Skipped {skipped} files.");
            Ok(())
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
            if use_sqlite_mode {
                let model = SqliteModel::open(Path::new(&index_path))?;
                for (path, rank) in model.search_query(&prompt)?.iter().take(20) {
                    println!("{path} {rank}", path = path.display());
                }
            } else {
                let index_file = File::open(&index_path).map_err(|err| {
                    eprintln!("ERROR: could not open index file {index_path}: {err}");
                })?; 
                let model = serde_json::from_reader::<_, InMemoryModel>(index_file).map_err(|err| {
                    eprintln!("ERROR: could not parse index file {index_path}: {err}");
                })?;
                for (path, rank) in model.search_query(&prompt)?.iter().take(20) {
                    println!("{path} {rank}", path = path.display())
                }
            }
            Ok(())
        },
        */
        "serve" => {
            assert!(!use_sqlite_mode);
            let dir_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: no directory was provided for {subcommand} subcommand");
            })?;
            
            let mut index_path_base = PathBuf::new();
            index_path_base.push(dir_path.clone() + ".tmp");
            index_path_base.set_extension("index.json");
            let mut index_path = PathBuf::new();
            index_path.push(index_path_base.display().to_string().replace("/", ".").as_str());

            let address = args.next().unwrap_or("127.0.0.1:8080".to_string());
            let exists = Path::new(&index_path).try_exists().map_err(|err| {
                eprintln!("ERROR: could not check the existence of file {index_path}: {err}", index_path = index_path.display());
            })?;
            let model: Arc<Mutex<InMemoryModel>>;
            if exists {
                let index_file = File::open(&index_path).map_err(|err| {
                    eprintln!("ERROR: could not open index file {index_path}: {err}", index_path = index_path.display());
                })?; 
                model = Arc::new(Mutex::new(serde_json::from_reader(index_file).map_err(|err| {
                    eprintln!("ERROR: could not parse index file {index_path}: {err}", index_path = index_path.display());
                })?));
            } else {
                model = Arc::new(Mutex::new(Default::default()));
            }

            {
                let model = Arc::clone(&model);
                thread::spawn(move || {
                    let mut skipped = 0;
                    add_folder_to_model(Path::new(&dir_path), Arc::clone(&model), &mut skipped).unwrap();
                    let model = model.lock().unwrap();
                    save_model_as_json(&model, &index_path.display().to_string().as_str()).unwrap();
                });
            }
            server_ep05::start(&address, Arc::clone(&model))
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
