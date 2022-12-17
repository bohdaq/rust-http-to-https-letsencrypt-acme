use std::io::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use crate::app::App;

use rust_web_server::request::{METHOD, Request};
use rust_web_server::response::{Response, STATUS_CODE_REASON_PHRASE};
use rust_web_server::header::Header;
use crate::log::Log;

pub struct Server {}
impl Server {
    pub fn process_request(mut stream: TcpStream) -> Vec<u8> {
        let mut buffer :[u8; 1024] = [0; 1024];
        let boxed_read = stream.read(&mut buffer);
        if boxed_read.is_err() {
            eprintln!("unable to read TCP stream {}", boxed_read.err().unwrap());

            let raw_response = Server::bad_request_response();
            let boxed_stream = stream.write(&raw_response);
            if boxed_stream.is_ok() {
                stream.flush().unwrap();
            };
            return raw_response;
        }

        boxed_read.unwrap();
        let request : &[u8] = &buffer;


        let boxed_request = Request::parse_request(request);
        if boxed_request.is_err() {
            eprintln!("unable to parse request: {}", boxed_request.err().unwrap());

            let raw_response = Server::bad_request_response();
            let boxed_stream = stream.write(&raw_response);
            if boxed_stream.is_ok() {
                stream.flush().unwrap();
            };
            return raw_response;
        }


        let request: Request = boxed_request.unwrap();
        let (response, request) = App::handle_request(request);

        let mut peer_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)), 0);
        let boxed_peer_addr = stream.peer_addr();
        if boxed_peer_addr.is_ok() {
            peer_addr = boxed_peer_addr.unwrap()
        } else {
            eprintln!("\nunable to read peer addr");
        }

        let log_request_response = Log::request_response(&request, &response, &peer_addr);
        println!("{}", log_request_response);

        let raw_response = Response::generate_response(response, request);

        let boxed_stream = stream.write(&raw_response);
        if boxed_stream.is_ok() {
            stream.flush().unwrap();
        };

        raw_response
    }

    pub fn bad_request_response() -> Vec<u8> {
        let error_request = Request {
            method: METHOD.head.to_string(),
            request_uri: "".to_string(),
            http_version: "".to_string(),
            headers: vec![]
        };

        let header_list = Header::get_header_list(&error_request);
        let error_response: Response = Response::get_response(
            STATUS_CODE_REASON_PHRASE.n400_bad_request,
            Some(header_list),
            None
        );

        let response = Response::generate_response(error_response, error_request);
        return response;
    }

}
