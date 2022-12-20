pub mod app;
pub mod server;
pub mod log;

use std::net::TcpListener;
use rust_web_server::entry_point::{bootstrap, get_ip_port_thread_count, set_default_values};
use rust_web_server::symbol::SYMBOL;
use rust_web_server::thread_pool::ThreadPool;
use crate::log::Log;
use crate::server::Server;

fn main() {
    start();
}

pub fn start() {
    let info = Log::info("HTTP to HTTPS with LetsEncrypt HTTP verification server");
    println!("{}", info);
    let usage = Log::usage_information();
    println!("{}", usage);
    println!("RWS Configuration Start: \n");
    set_default_values();
    bootstrap();
    let (ip, port, thread_count) = get_ip_port_thread_count();
    create_tcp_listener_with_thread_pool(ip.as_str(), port, thread_count);
}

pub fn create_tcp_listener_with_thread_pool(ip: &str, port: i32, thread_count: i32) {
    let bind_addr = [ip, SYMBOL.colon, port.to_string().as_str()].join(SYMBOL.empty_string);
    println!("Setting up http://{}", bind_addr);

    let boxed_listener = TcpListener::bind(&bind_addr);
    if boxed_listener.is_err() {
        eprintln!("unable to set up TCP listener: {}", boxed_listener.err().unwrap());
    } else {
        let listener = boxed_listener.unwrap();
        let pool = ThreadPool::new(thread_count as usize);

        let server_url_thread_count = Log::server_url_thread_count("http", &bind_addr, thread_count);
        println!("{}", server_url_thread_count);

        for boxed_stream in listener.incoming() {
            if boxed_stream.is_err() {
                eprintln!("unable to get TCP stream: {}", boxed_stream.err().unwrap());
            } else {
                let stream = boxed_stream.unwrap();

                print!("Connection established, ");

                let boxed_local_addr = stream.local_addr();
                if boxed_local_addr.is_ok() {
                    print!("local addr: {}", boxed_local_addr.unwrap())
                } else {
                    eprintln!("\nunable to read local addr");
                }

                let boxed_peer_addr = stream.local_addr();
                if boxed_peer_addr.is_ok() {
                    print!(", peer addr: {}\n", boxed_peer_addr.unwrap())
                } else {
                    eprintln!("\nunable to read peer addr");
                }

                pool.execute(move || {
                    Server::process_request(stream);
                });
            }
        }
    }

}
