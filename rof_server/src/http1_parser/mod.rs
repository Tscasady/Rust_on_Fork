mod body_parser;

use std::str::FromStr;
use std::collections::HashMap;
use strum_macros::EnumString;
use body_parser::*;


pub struct Request {
    request_line: RequestLine,
    //header != headers 
    headers: Vec<Header>,
    body: Option<Body<MediaType>> //length information
}

enum Header {
// http1.x: header fields are transmitted after the request line  or the response line, the first line of a message. Header fields are colon-separated key-value pairs in clear-text string format, terminated by a CR and LF character sequence. 
// The end of the header section is indicated by an empty field line, resulting in the transmission of two consecutive CR-LF pairs. 
//Header field names are case-insensitive
// it is not uncommon to limit the size and number of headers for security / practicality
}

struct RequestLine {
    method: Result<HttpMethod, strum::ParseError>,
    uri: URI,
    version: HttpVersion
}

impl RequestLine {
    fn parse(request_line_raw: &str) -> RequestLine {
        let request_line: Vec<&str> = request_line_raw.split(" ").collect();
        return RequestLine {
            method: HttpMethod::from_str(request_line[0]),
            uri: URI::from_str(request_line[1]),
            version: HttpVersion::HTTP1 //TODO: add support for other versions
        }
        
    }
}

#[derive(EnumString, Debug, PartialEq)]
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

enum HttpVersion {
    //i dont understand the difference, just handles 1.1 for now.
    HTTP1
}

struct URI {
     // http-URI = "http" "://" authority path-abempty [ "?" query ]
    query_params: HashMap<String, String>
    //idk if query_params should live as part of uri, probs just its own entity
}

impl URI {
    fn from_str(uri: &str) -> URI {
        todo!()
    }
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
#[cfg(test)]
mod test {
    use super::HttpMethod;
    use std::str::FromStr;
     
    fn httpmethod_new() {
        let verb = "GET";
        assert_eq!(HttpMethod::GET, HttpMethod::from_str(verb).unwrap())
    }

}
