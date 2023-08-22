use std::str::FromStr;
use strum_macros::EnumString;
mod body_parser;


pub struct Request {
    request_line: RequestLine,
    headers: Vec<Header>,
    //header != headers 
    body: body_parser::Body<T> //length information
}

enum Header {
    
}

struct RequestLine {
    method: HttpMethod,
    uri: URI,
    version: HttpVersion
}

impl RequestLine {
    fn parse_requestline(&self) -> RequestLine {
        todo!()
    }
}

#[derive(EnumString)]
enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
}

// impl FromStr for HttpMethod {
//
//     type Err = ();
//
//     fn from_str(input: &str) -> Result<HttpMethod, Self::Err> {
//         match input {
//             "GET"  => Ok(HttpMethod::GET),
//             "HEAD"  => Ok(HttpMethod::HEAD),
//             "POST"  => Ok(HttpMethod::POST),
//             "PUT"  => Ok(HttpMethod::PUT),
//             "DELETE"  => Ok(HttpMethod::DELETE),
//             "CONNECT"  => Ok(HttpMethod::CONNECT),
//             "OPTIONS"  => Ok(HttpMethod::OPTIONS),
//             "TRACE"  => Ok(HttpMethod::TRACE),
//             _      => Err(()),
//         }
//     }
// }

enum HttpVersion {}
struct URI {
     // http-URI = "http" "://" authority path-abempty [ "?" query ]
}





//query params, uri vars?
//what happens if a malicious actor sends a request with no size or that never ends, send 431 or
//414
//rustls for tls if needed
//nom crate


    // let buf_reader = BufReader::new(&mut stream);
    // let request_line = buf_reader.lines().next().unwrap().unwrap();
    // let (filename, status_line) = match &request_line[..] {
    //     "GET / HTTP/1.1" => ("hello.html", "HTTP/1.1 200 OK"),
    //     "GET /sleep HTTP/1.1" => {
    //         thread::sleep(Duration::from_secs(5));
    //         ("hello.html", "HTTP/1.1 200 OK")
    //     }
    //     _ => ("404.html", "HTTP/1.1 404 NOT FOUND"),
    // };
    //
    // let contents = fs::read_to_string(filename).unwrap();
    // let length = contents.len();
    // let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    //
    // stream.write_all(response.as_bytes()).unwrap();
