mod body_parser;

use std::str::FromStr;
use std::collections::HashMap;
use strum_macros::EnumString;
use body_parser::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Path parsing error")]
    PathParseError,
    #[error("Invalid HTTP Method")]
    MethodParseError(#[from] strum::ParseError)
}

pub struct HTTPRequest {
    request_line: Result<RequestLine, strum::ParseError>,
    //header != headers 
    headers: HashMap<String, String>,
    query_params: Option<HashMap<String, String>>,
    body: Option<Body<MediaType>> //length information
}

impl HTTPRequest {
    pub fn new(request_line_raw: String) -> Self {
        todo!()
    }
}

// enum Header {
// http1.x: header fields are transmitted after the request line  or the response line, the first line of a message. Header fields are colon-separated key-value pairs in clear-text string format, terminated by a CR and LF character sequence. 
// The end of the header section is indicated by an empty field line, resulting in the transmission of two consecutive CR-LF pairs. 
//Header field names are case-insensitive
// it is not uncommon to limit the size and number of headers for security / practicality
// }

struct RequestLine {
    method: HttpMethod,
    path: Path,
    version: HttpVersion
}

impl RequestLine {
    fn parse(request_line_raw: &str) -> Result<RequestLine, RequestError> {
        let request_line: Vec<&str> = request_line_raw.split(" ").collect();
        return Ok(RequestLine {
            method: HttpMethod::from_str(request_line[0])?,
            path: Path::from_str(request_line[1])?,
            version: HttpVersion::HTTP1 //TODO: add support for other versions
        })
        
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
    HTTP1
}

struct Path {
     // http-URI = "http" "://" authority path-abempty [ "?" query ]
    //idk if query_params should live as part of uri, probs just its own entity
}

impl Path {
    fn from_str(uri: &str) -> Result<Path, strum::ParseError> {
        //split on ? if it exists
        todo!()
    }
}

//query params, uri vars?
//what happens if a malicious actor sends a request with no size or that never ends, send 431 or
//414
//rustls for tls if needed
//nom crate

#[cfg(test)]
mod test {
    use super::HttpMethod;
    use std::str::FromStr;
     
    #[test]
    fn httpmethod_new() {
        let verb = "GET";
        assert_eq!(HttpMethod::GET, HttpMethod::from_str(verb).unwrap());

        let verb = "POST";
        assert_eq!(HttpMethod::POST, HttpMethod::from_str(verb).unwrap());

    }

    #[test]
    #[should_panic]
    fn bad_verb() {
        let verb = "BADSTRING";
        HttpMethod::from_str(verb).unwrap();
    }
}
