use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use webpki;
use webpki_roots;

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
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

    pub fn get_response_buffer(&self) -> &str {
        &self.response_buffer
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

    pub fn get_body(&self) -> Body {
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
        let s_spot = surl.chars().nth(4);
        let scheme_end_index = if s_spot == Some('s') {
            8
        } else if s_spot == Some(':') {
            7
        } else {
            return None;
        };
        let scheme = &surl[0..scheme_end_index];
        let host_and_path = &surl[scheme_end_index..];

        let host_end_index = host_and_path.find("/").unwrap_or(host_and_path.len());

        let host = &host_and_path[0..host_end_index];
        let path = &host_and_path[host.len()..];
        let path = if path == "" { "/" } else { path };
        Some(Url { scheme, host, path })
    }
}

#[derive(Debug)]
pub struct Body<'resp> {
    pub body_buffer: &'resp str,
}

fn request<'surl, 'resp>(url: &'surl str) -> Response {
    let my_url = Url::new(url).unwrap();
    let headers: String = vec![("Host", my_url.host), ("Connection", "Close")]
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<String>>()
        .join("\r\n");
    let http_string = format!("GET {} HTTP/1.1\r\n{}\r\n\r\n", my_url.path, headers);

    if my_url.scheme == "https://" {
        let mut config = rustls::ClientConfig::new();
        config
            .root_store
            .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

        let rc_config = Arc::new(config);
        let dns_name = webpki::DNSNameRef::try_from_ascii_str(my_url.host).unwrap();
        let mut sess = rustls::ClientSession::new(&rc_config, dns_name);
        let mut stream = TcpStream::connect((my_url.host, 443)).unwrap();
        let mut tls = rustls::Stream::new(&mut sess, &mut stream);

        let _ = tls.write(http_string.as_bytes()).unwrap();

        let mut response_buffer = String::new();
        tls.read_to_string(&mut response_buffer).unwrap();

        Response::new(response_buffer)
    } else {
        let mut stream = TcpStream::connect((my_url.host, 80)).unwrap();
        let _ = stream.write(http_string.as_bytes()).unwrap();

        let mut response_buffer = String::new();
        let _ = stream.read_to_string(&mut response_buffer).unwrap();

        Response::new(response_buffer)
    }
}

pub fn load(url: &str) -> Response {
    request(url)
}
