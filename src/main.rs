use std::collections::HashMap;
use std::env;
use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Debug)]
struct Url<'surl> {
    scheme: &'surl str,
    host: &'surl str,
    path: &'surl str,
}

#[derive(Debug)]
struct StatusLine<'resp> {
    version: &'resp str,
    status: &'resp str,
    explanation: &'resp str,
}

type Headers<'resp> = HashMap<&'resp str, &'resp str>;

#[derive(Debug)]
struct Response {
    response_buffer: String,
    status_line_end_index: usize,
    headers_end_index: usize,
}

impl Response {
    fn new(response_buffer: String) -> Self {
        let status_line_end_index = response_buffer.find("\r\n").unwrap();

        let headers_end_index = response_buffer.find("\r\n\r\n").unwrap();

        Response {
            response_buffer: response_buffer,
            status_line_end_index,
            headers_end_index,
        }
    }

    fn get_status_line(&self) -> Option<StatusLine> {
        let mut parts = self.response_buffer[0..self.status_line_end_index].split(" ");

        let version = parts.next().unwrap();
        let status = parts.next().unwrap();
        let explanation = parts.next().unwrap();

        Some(StatusLine {
            version,
            status: status,
            explanation,
        })
    }

    fn get_headers(&self) -> Headers {
        let kvs = self.response_buffer[self.status_line_end_index + 2..self.headers_end_index]
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
                    (Some(k), Some(v)) => Some((k, v)),
                    _ => None,
                }
            });
        HashMap::from(kvs.collect())
    }

    fn get_body(&self) -> Body {
        Body {
            body_buffer: &self.response_buffer
                [self.headers_end_index + 4..self.response_buffer.len() - 1],
        }
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

impl<'surl> Url<'surl> {
    fn new(surl: &'surl str) -> Option<Self> {
        let scheme = &surl[0..7];
        let host = get_host(&surl[7..]);
        let path = &surl[7 + host.len()..];
        if scheme == "http://" {
            Some(Url { scheme, host, path })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Body<'resp> {
    body_buffer: &'resp str,
}

fn request<'surl, 'resp>(url: &'surl str) -> Response {
    let my_url = Url::new(url).unwrap();
    let http_string =
        String::from("GET /index.html HTTP/1.0\r\n") + &format!("Host: {}\r\n\r\n", my_url.host);

    let mut stream = TcpStream::connect((my_url.host, 80)).unwrap();
    let _ = stream.write(http_string.as_bytes()).unwrap();

    let mut response_buffer = String::new();
    let _ = stream.read_to_string(&mut response_buffer).unwrap();
    Response::new(response_buffer)
}

fn print_body(body: &Body) {
    let mut in_angle = false;
    body.body_buffer.chars().for_each(|c| {
        if c == '<' {
            in_angle = true;
        } else if c == '>' {
            in_angle = false;
        } else {
            if !in_angle {
                print!("{}", c)
            }
        }
    });
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let url = if args.len() == 2 {
        &args[1]
    } else {
        "http://example.org/index.html"
    };

    let response = request(url);

    print_body(&response.get_body());
}
