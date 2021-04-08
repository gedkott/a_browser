use crate::layout;
use crate::lex;
use crate::response;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use xml_rpc::{Fault, Server};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct UrlParams {
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RelayoutParams {
    pub width: i32,
    pub height: i32,
    pub body: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ResponseBuffer {
    pub layout: layout::Layout,
    pub body: String,
}

fn load_url(url_params: UrlParams) -> Result<ResponseBuffer, Fault> {
    let resp = response::load(&url_params.url);
    let body = resp.get_body();
    Ok(ResponseBuffer {
        layout: layout::layout(&lex::lex(&body.body_buffer), 800),
        body: body.body_buffer.to_string(),
    })
}

fn relayout(relayout_params: RelayoutParams) -> Result<ResponseBuffer, Fault> {
    Ok(ResponseBuffer {
        layout: layout::layout(&lex::lex(&relayout_params.body), relayout_params.width),
        body: relayout_params.body.to_string(),
    })
}

pub fn new_server() {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let mut server = Server::new();

    server.register_simple("load_url", &load_url);
    server.register_simple("relayout", &relayout);

    let bound_server = server.bind(&socket).unwrap();

    println!("Running server");
    bound_server.run();
}
