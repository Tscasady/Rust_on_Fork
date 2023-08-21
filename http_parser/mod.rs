use std::str::Fromstr

pub struct Request {
    headers: Header,
    //header != headers 
    request_line: RequestLine,
    body: String //this needs to be generic over the types of valid bodies? this also includes
    //length information
};

struct RequestLine {
    method: HttpMethod,
    uri: URI
    version: HttpVersion
}

struct HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
}




//query params, uri vars?
//what happens if a malicious actor sends a request with no size or that never ends, send 431 or
//414
//rustls for tls if needed
//nom crate
