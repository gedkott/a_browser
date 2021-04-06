use std::io::prelude::*;
use std::net::TcpStream;
use std::collections::HashMap;

#[derive(Debug)]
struct Url<'surl> {
    scheme: &'surl str,
    host: &'surl str,
    path: &'surl str
}

#[derive(Debug)]
struct StatusLine<'resp> {
    version: &'resp str,
    status: &'resp str,
    explanation: &'resp str
}

impl StatusLine<'_> {
    fn new() -> Self {
        StatusLine {
            version: "",
            status: "",
            explanation: ""
        }
    }
}

type Headers<'resp> = HashMap<&'resp str, &'resp str>;

#[derive(Debug)]
struct Response {
    response_buffer: String,
}

impl Response {
    fn new(response_buffer: String) -> Self {    
        println!("{:?}", response_buffer);
    
        // let status_line = get_status_line(&response_buffer)
    
        // let mut parts = response_buffer[index..].split("\r\n\r\n");
        // let headers_string = parts.next().unwrap();
        // let headers = get_headers(headers_string);
        // let body = parts.next().unwrap();
    

        // let response = 
        // println!("{:?}", response);
        // // println!("{:?}", body);
    
        // response
        let (status_line, body): (StatusLine<'_>, usize) = {
            let mut parts = response_buffer.match_indices("\r\n\r\n");
            let (status_line_and_headers_index, status_line_and_headers) = parts.next().unwrap();
            let (body_index, _) = parts.next().unwrap();
    
            let status_line = Response::get_status_line(status_line_and_headers).unwrap();
            (status_line, body_index)
        };

        Response {
            response_buffer: response_buffer,
        }
    }

    fn get_status_line(response_buffer: &str) -> Option<StatusLine<'_>> {
        // get status line
        for i in 0..response_buffer.len() {
            if response_buffer.chars().nth(i) == Some('\r') && response_buffer.chars().nth(i + 1) == Some('\n') {
                let status_line_string = &response_buffer[0..i + 2];
                let mut parts = status_line_string.split(" ");
                let count = status_line_string.split(" ").count();
                if count != 3 {
                    return None
                } else {
                    return Some(StatusLine {
                        version: parts.next().unwrap(),
                        status: parts.next().unwrap(),
                        explanation: parts.next().unwrap()
                    })
                }
            }
        }
    
        return None
    }
}

fn get_host(surl: &str) -> &str {
    let stop_char = '/';
    let mut curr_index = 0;

    while let Some(c) = surl.chars().nth(curr_index) {
        if c != stop_char {
            curr_index += 1;
        } else {
            return &surl[0..curr_index];
        }
    }

    &surl[0..curr_index]
}

fn get_headers(response_buffer: &str) -> Headers {
    let kvs = response_buffer
        .split("\r\n")
        .filter(|line| !line.is_empty())
        .map(|header_string| {
            let mut parts = header_string.split(": ");
            let key = parts.next();
            let value = parts.next();
            (key, value)
        })
        .filter_map(|kv_pair| {
            let (key, value) = kv_pair;
            match (key, value) {
                (Some(k), Some(v)) => {
                    Some((k,v))
                }
                _ => None
            }
        });
    HashMap::from(kvs.collect())
}

impl<'surl> Url<'surl> {
    fn new(surl: &'surl str) -> Option<Self> {
        let scheme = &surl[0..7];
        let host = get_host(&surl[7..]);
        let path = &surl[7 + host.len()..];
        if scheme == "http://" {
            Some(Url {
                scheme,
                host,
                path
            })
        } else {
            None
        }
    }
}

type Body<'resp> = &'resp str;

fn request<'surl, 'resp>(url: &'surl str) -> Response {
    let my_url = Url::new(url).unwrap();
    println!("{:?}", my_url);

    let mut stream = TcpStream::connect((my_url.host, 80)).unwrap();
    println!("{:?}", stream);


    let http_string = String::from("GET /index.html HTTP/1.0\r\n") + 
                      "Host: example.org\r\n\r\n";
    
    let request_bytes = stream.write(http_string.as_bytes()).unwrap();
    println!("{:?}", request_bytes);

    let mut response_buffer = String::new();
    let _ = stream.read_to_string(&mut response_buffer).unwrap();
    let response = Response::new(response_buffer);
    
    response
}

fn main() {
    let url = "http://example.org/index.html"; 
    
    request(url);
    
}
