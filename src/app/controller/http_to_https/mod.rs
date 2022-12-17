#[cfg(test)]
mod tests;

use rust_web_server::header::Header;
use rust_web_server::request::Request;
use rust_web_server::response::{Response, STATUS_CODE_REASON_PHRASE};

pub struct HttpToHttpsController;

impl HttpToHttpsController {
    pub const _LETSENCRYPT_REQUEST_URI_PREFIX: &'static str = "/.well-known/acme-challenge";


    pub fn is_matching_request(_request: &Request) -> bool {
        HttpToHttpsController::is_not_letsencrypt_request_uri(&_request.request_uri)
    }

    pub fn process_request(_request: &Request, mut response: Response) -> Response {
        response.status_code = *STATUS_CODE_REASON_PHRASE.n301_moved_permanently.status_code;
        response.reason_phrase = STATUS_CODE_REASON_PHRASE.n301_moved_permanently.reason_phrase.to_string();


        let boxed_host = _request.get_header("Host".to_string());
        if boxed_host.is_some() {
            let host = boxed_host.unwrap();
            let location_header = [ "https://", &host.value, &_request.request_uri ].join("");
            response.headers.push(
                Header {
                    name: Header::_LOCATION.to_string(),
                    value: location_header.to_string()
                }
            );
        }

        response
    }

    fn is_not_letsencrypt_request_uri(request_uri: &str) -> bool {
        println!("{} {}", request_uri, HttpToHttpsController::_LETSENCRYPT_REQUEST_URI_PREFIX);
        let uri = request_uri.chars().filter(|c| !c.is_ascii_control()).collect::<String>();
        let starts_with = uri.starts_with(HttpToHttpsController::_LETSENCRYPT_REQUEST_URI_PREFIX);
        return !starts_with;
    }
}

