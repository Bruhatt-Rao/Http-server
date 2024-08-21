use std::io::{prelude::*, BufReader};
use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::HashMap;
use std::path::Path;
use std::fs;

pub fn exists(path: &str) -> PathCheck {
    let files: Vec<&str> = (path.split("/")).collect();
    let mut file = "".to_owned();
    let mut i = 0;
    for f in &files {
        if f == &"" {
            continue;
        }
        if i != 0 {
            file.push_str("/");
        }
        i += 1;
        file.push_str(&f);
    }
    let p = file;
    let exist = Path::new(&p).exists();
    return PathCheck::new(p, exist);

}

pub fn start() {

    let mut paths = HashMap::<String, String>::new();
    paths.insert("/".to_string(), "Welcome to the home page".to_string());

    const HOST : &str = "localhost";
    const PORT : &str = "8080";

    let end_point : String = HOST.to_owned() + ":" + PORT;

    let listener = TcpListener::bind(end_point).unwrap();

    println!("Web Server is live at: http://{}:{}", HOST, PORT);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        handle_connection(_stream, &paths);
    }
}

fn handle_connection(mut stream: TcpStream, paths: &HashMap<String, String>) {
    let reader = BufReader::new(&mut stream);

    let r = reader.lines().next();
    match r {
        Some(Ok(ref _val)) => {

            let request_line = r.unwrap().unwrap();
            let request = Req::new(&request_line);

            let response = handle_request(&request, &paths);

            stream.write_all(response.as_bytes()).unwrap();

        }
        Some(Err(_e)) => { println!("HTTPS not supported");return },
        None => { println!("None Error"); }
    }
}

fn handle_request(req: &Req, paths: &HashMap<String, String>) -> String {
    let binding = "Not Found".to_string();
    let body = paths.get(&req.path).unwrap_or(&binding);
    println!("{:?}", &req.path);
    let ex = exists(&req.path);
    if req.path.contains(".") && ex.exists {
        // println!("File Found {}", req.path);
        let contents = fs::read_to_string(ex.p).unwrap();
        format_req(&req, "200");
        format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents)
    } else if body.to_string() == binding {
        format_req(&req, "404");
        return "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 9\r\n\r\nNot Found".to_string();
    } else {
        // println!("Found");
        format_req(&req, "200");
        format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", body.len(), body)
    }
}

fn format_req(val: &Req, code: &str) {
    let value = " ".to_owned() + code + " - " + &val.path + " ";
    println!("{}", value);
}

pub struct Req {
    req_type: String,
    path: String,
    proto: String,
}

impl Req {
    fn new(req_line: &str) -> Req {
        let seper: Vec<&str> = (req_line.split(" ")).collect();
        let req_type = seper[0].to_owned();
        let mut path = seper[1].to_owned();
        let proto = seper[2].to_owned();
        if path == "/" {
            path = "/index.html".to_owned();
        }
        Req{ req_type, path, proto }
    }
}

pub struct PathCheck {
    p: String,
    pub exists: bool,
}

impl PathCheck {
    fn new(p: String, exists: bool) -> PathCheck {
        PathCheck { p, exists }
    }
}