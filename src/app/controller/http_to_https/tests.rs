use crate::app::controller::http_to_https::HttpToHttpsController;

#[test]
fn valid_letsencrypt() {
    let url = "/.well-known/acme-challenge/some-id";
    let is_letsencrypt = HttpToHttpsController::is_letsencrypt_request_uri(url);
    assert!(is_letsencrypt);
}