use std::net::{TcpListener, TcpStream};
use std::{fs, thread, io};
use std::io::{Write, Read, BufRead};
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc;

/// Starts a minimal static serve server that servers index.html along with the required json files
/// On any change the server receives the changed json and updates settings.json in realtime
pub fn start_gui_server(){
    println!("Starting GUI Server");

    println!("Press enter to terminate GUI Server");
    let (tx, rx) = mpsc::channel();

    let ip_port = String::from("127.0.0.1:7878");
    let mut gui_server = GuiServer::new(ip_port.clone());
    let listener = gui_server.start(true);

    thread::spawn(move ||
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream);
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("GUI Server is terminating.");
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
        }
    );
    let mut line = String::new();
    let stdin = io::stdin();
    let _ = stdin.lock().read_line(&mut line);
    let _ = tx.send(());
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

    pub fn start(&mut self, open:bool) -> TcpListener {
        let listener = TcpListener::bind(&self.ip_port).unwrap();
        if open {
            if webbrowser::open(&format!("{}{}","http://",self.ip_port)).is_ok() { }
        }
        listener
    }
}

fn handle_connection(mut stream: TcpStream) {
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
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./src/index.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}