use crate::app::controller::http_to_https::is_letsencrypt_request_uri;

#[test]
fn valid_letsencrypt() {
    let url = "/.well_knows/acme-challenge/some-id";
    let is_letsencrypt = is_letsencrypt_request_uri(url);
    assert!(is_letsencrypt);
}