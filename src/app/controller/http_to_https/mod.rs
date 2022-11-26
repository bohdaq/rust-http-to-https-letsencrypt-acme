mod tests;

use file_ext::FileExt;
use rust_web_server::mime_type::MimeType;
use rust_web_server::range::Range;
use rust_web_server::request::Request;
use rust_web_server::response::{Response, STATUS_CODE_REASON_PHRASE};

pub struct HttpToHttpsController;

impl HttpToHttpsController {

    pub fn is_matching_request(_request: &Request) -> bool {
        HttpToHttpsController::is_letsencrypt_request_uri(_request.request_uri)
    }

    pub fn process_request(_request: &Request, mut response: Response) -> Response {
        response.status_code = *STATUS_CODE_REASON_PHRASE.n301_moved_permanently.status_code;
        response.reason_phrase = STATUS_CODE_REASON_PHRASE.n301_moved_permanently.reason_phrase.to_string();


        let boxed_host = _request.get_header("Host");
        if boxed_host.is_ok() {
            let host = boxed_host.unwrap();
            let location_header = [ "https://", host, "/", _request.request_uri ].join("");
            response.headers.push(
                Header {
                    name: Header::_LOCATION.to_string(),
                    value: location_header.to_string()
                }
            );
        }

        response
    }
}

fn is_letsencrypt_request_uri(request_uri: &str) -> bool {
    false
}