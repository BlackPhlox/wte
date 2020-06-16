use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::{fs, thread};
use webbrowser;
use std::time::Duration;
//use curl::easy;
//use curl::easy::{Easy, Easy2};

fn main() {
    let ip_port = String::from("127.0.0.1:7878");

    let mut o = GuiServer::new(ip_port.clone());

    o.start(true);

    o.stop();
    /*
    let mut easy = Easy::new();
    easy.get(true).unwrap();
    easy.url(&format!("{}{}{}","http://",ip_port,"/stop")).unwrap();
    easy.perform().unwrap();
    println!("{}",easy.response_code().unwrap());
    */

    //thread::sleep(Duration::from_secs(5));

    //reqwest::get(&format!("{}{}{}","http://",ip_port,"/stop"));

}


pub struct GuiServer {
    ip_port: String
}

impl<'a> GuiServer {
    pub fn new(ip_port: String) -> GuiServer {
        GuiServer{
            ip_port
        }
    }

    pub fn stop(&mut self){
        println!("{}",&format!("{}{}{}","http://",self.ip_port,"/stop"));
    }

    pub fn start(&mut self, open:bool) -> &mut GuiServer {
        let listener = TcpListener::bind(&self.ip_port).unwrap();

        if open {
            if webbrowser::open(&format!("{}{}","http://",self.ip_port)).is_ok() { }
        }

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let stop = handle_connection(stream);
            if stop {break;}
        }
        self
    }
}

fn handle_connection(mut stream: TcpStream) -> bool {
    let mut buffer = [0; 512*4];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let wt_schema = b"GET /wt_schema.json HTTP/1.1\r\n";
    let settings = b"GET /settings.json HTTP/1.1\r\n";
    let stop = b"GET /stop HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "./src/index.html")
    } else if buffer.starts_with(wt_schema) {
        ("HTTP/1.1 200 OK\r\n\r\n", "./src/wt_schema.json")
    } else if buffer.starts_with(settings) {
        ("HTTP/1.1 200 OK\r\n\r\n", "./src/settings.json")
    } else if buffer.starts_with(stop) {
        ("HTTP/1.1 200 OK\r\n\r\n", "stop")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./src/index.html")
    };

    if filename == "stop" {return true}

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    false
}