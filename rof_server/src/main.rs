use std::net::{TcpListener, SocketAddr};
use std::env;

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
    }
}

