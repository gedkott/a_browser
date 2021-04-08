use crate::lex;
use crate::response;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use xml_rpc::{Fault, Server};
#[derive(Clone, Debug, Serialize, Deserialize)]
struct UrlParams {
    pub url: String,
}

type Layout = Vec<(i32, i32, char)>;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ResponseBuffer {
    pub layout: Layout,
}

fn layout(body: &str) -> Layout {
    let mut display_list = vec![];
    let hstep = 13;
    let vstep = 18;
    let width = 800;
    let height = 600;
    let mut cursor_x = 100;
    let mut cursor_y = 100;
    body.chars().for_each(|c| {
        if cursor_x >= width - hstep {
            cursor_y = cursor_y + vstep;
            cursor_x = hstep;
        }
        cursor_x = cursor_x + hstep;
        display_list.push((cursor_x, cursor_y, c))
    });
    display_list
}

fn load_url(url_params: UrlParams) -> Result<ResponseBuffer, Fault> {
    Ok(ResponseBuffer {
        layout: layout(&lex::lex(&response::load(&url_params.url).get_body())),
    })
}

pub fn new_server() {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let mut server = Server::new();

    server.register_simple("load_url", &load_url);

    let bound_server = server.bind(&socket).unwrap();

    println!("Running server");
    bound_server.run();
}
