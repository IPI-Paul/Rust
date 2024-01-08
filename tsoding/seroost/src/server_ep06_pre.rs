use std::{fs::File, str, io, sync::{Mutex, Arc}};
use tiny_http::{Server, Response, Header, Request, Method, StatusCode};
use super::model_ep06_pre::*;

fn serve_400(request: Request, message: &str) -> io::Result<()> {
    request.respond(Response::from_string(format!("400: {message}")).with_status_code(StatusCode(400)))
}

fn serve_404(request: Request) -> io::Result<()> {
    request.respond(Response::from_string("404").with_status_code(StatusCode(404)))
}

fn serve_500(request: Request) -> io::Result<()> {
    request.respond(Response::from_string("500").with_status_code(StatusCode(500)))
}

fn serve_static_file(request: Request, file_path: &str, content_type: &str) -> io::Result<()> {
    let content_type_header = Header::from_bytes("Content-Type", content_type)
        .expect("That we didn't put any garbage in the headers");
    let file = match File::open(file_path) {
      Ok(file) => file,
      Err(err) => {
        eprintln!("ERROR: could not serve file {file_path}: {err}");
        if err.kind() == io::ErrorKind::NotFound {
          return serve_404(request);
        }
        return serve_500(request);
      }
    };
    request.respond(Response::from_file(file).with_header(content_type_header))
}

fn serve_api_search(model: Arc<Mutex<Model>>, mut request: Request) -> io::Result<()> {
  let mut buf = Vec::new();
  if let Err(err) = request.as_reader().read_to_end(&mut buf) {
    eprintln!("ERROR: could not read the body of the request: {err}");
    return serve_500(request);
  }
  let body = match str::from_utf8(&buf) {
    Ok(body) => body.chars().collect::<Vec<_>>(),
    Err(err) => {
      eprintln!("ERROR: could not interpret body as UTF-8 string: {err}");
      return serve_400(request, "Body must be a valid UTF-8 string");
    },
  };
  let model = model.lock().unwrap();
  let result = model.search_query(&body);
  let json = match serde_json::to_string(&result.iter().take(20).collect::<Vec<_>>()) {
    Ok(json) => json,
    Err(err) => {
      eprintln!("ERROR: could not convert search results to JSON: {err}");
      return serve_500(request);
    }
  };
  let content_type_header = Header::from_bytes("Content-Type", "application/json")
      .expect("That we didn't put any garbage in the headers");
  request.respond(Response::from_string(&json).with_header(content_type_header))
}

fn get_file_path_with_extension(model: Arc<Mutex<Model>>, request: &mut Request) -> String {
  let mut url = format!(".{url}", url = request.url().to_string());
  let mut buf = Vec::new();
  if let Err(err) = request.as_reader().read_to_end(&mut buf) {
    eprintln!("ERROR: could not read the body of the request: {err}");
  }
  let body = match str::from_utf8(&buf) {
    Ok(body) => body.chars().collect::<Vec<_>>(),
    Err(err) => {
      eprintln!("ERROR: could not interpret body as UTF-8 string: {err}");
      Vec::new()
    },
  };
  let model = model.lock().unwrap();
  let result = model.search_query(&body);
  let urls = result.iter()
    .filter(|(path, _)| path.with_extension("").to_str() == Some(url.as_str()))
    .map(|(path, _)| path)
    .collect::<Vec<_>>();
  if urls.len() == 1 {
    if Some(url.as_str()) != urls[0].to_str() {
      url = urls[0].as_path().display().to_string()
    }
  } 
  url
}

fn serve_request(model: Arc<Mutex<Model>>, request: Request) -> io::Result<()> {
    println!("INFO: received request! method: {:?}, url: {:?}",
        request.method(), request.url()
    );
    match (request.method(), request.url()) {
        (Method::Post, "/api/search") => {
          serve_api_search(model, request)
        },
        (Method::Get, "/css/style.css") => {
            serve_static_file(request, "css/style.css", "text/css; charset=utf-8")
        },
        (Method::Get, "/scripts/index.js") => {
            serve_static_file(request, "scripts/index.js", "text/javascript; charset=utf-8")
        },
        (Method::Get, "/") | (Method::Get, "/index.html") => {
            serve_static_file(request, "index.html", "text/html; charset=utf-8")
        },
        _ => {
          if request.url().contains("/docs.gl/") {
            let mut request = request;
            let url = get_file_path_with_extension(model, &mut request);
            serve_static_file(request, &url, "text/html; charset=utf-8")
          } else {
            serve_404(request)
          }
        },
    }
}

pub fn start(address: &str, model: Arc<Mutex<Model>>) -> Result<(), ()> {
  let server = Server::http(&address).map_err(|err| {
      eprintln!("ERROR: could not start HTTP server at {address}: {err}");
  })?;
  println!("INFO: listening at http://{address}/");
  for request in server.incoming_requests() {
      serve_request(Arc::clone(&model), request).map_err(|err| {
        eprintln!("ERROR: could not serve the response: {err}");
      }).ok();
  }
  eprintln!("ERROR: the server has shutdown");
  Err(())
}