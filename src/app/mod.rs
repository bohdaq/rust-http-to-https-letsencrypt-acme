pub mod controller;

use rust_web_server::app::controller::index::IndexController;
use rust_web_server::app::controller::not_found::NotFoundController;
use rust_web_server::app::controller::static_resource::StaticResourceController;
use rust_web_server::header::Header;
use rust_web_server::request::Request;
use rust_web_server::response::{Response, STATUS_CODE_REASON_PHRASE};
use crate::app::controller::http_to_https::HttpToHttpsController;

pub struct App {}

impl App {
    pub fn handle_request(request: Request) -> (Response, Request) {
        let header_list = Header::get_header_list(&request);

        let mut response: Response = Response::get_response(
            STATUS_CODE_REASON_PHRASE.n501_not_implemented,
            Some(header_list),
            None
        );

        if HttpToHttpsController::is_matching_request(&request) {
            response = HttpToHttpsController::process_request(&request, response);
            return (response, request)
        }

        (response, request)
    }

}