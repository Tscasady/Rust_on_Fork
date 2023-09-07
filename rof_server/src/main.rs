mod http1_parser;

use http1_parser::{HTTPRequest, RequestError};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::env;
use std::io::{prelude::*, BufReader};

pub struct Config {
    pub port: String,
    pub host: String,
}

impl Config {
    //default port 3000
    fn new(port: Result<String, env::VarError>, host: Result<String, env::VarError>) -> Self {
        Config { 
            port: port.unwrap_or("3000".to_owned()),
            host: host.unwrap_or("127.0.0.1".to_owned()),
        }
    }
}

fn main() {
    let config = Config::new(env::var("PORT"), env::var("HOST"));
    let addr = config.host + ":" + &config.port;
    println!("Starting Server on {}", addr);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(_) => ()
        } 
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request_raw: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request = HTTPRequest::new(http_request_raw)


}

//add a timeout for dead/unused connections
//slowloris attack?

