mod layout;
mod lex;
mod response;
mod rpc_server;

fn main() {
    rpc_server::new_server();
}
