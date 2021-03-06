extern crate websocket;
extern crate mount;
extern crate staticfile;
extern crate iron;

mod server;
mod page;
mod board;

fn main() {
    server::start();
    page::serve();
}