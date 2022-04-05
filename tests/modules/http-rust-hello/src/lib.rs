use wasi_outbound_http::*;

wit_bindgen_rust::import!("wit/ephemeral/wasi-outbound-http.wit");
wit_bindgen_rust::export!("../../test.wit");

struct Test {}

impl test::Test for Test {
    fn test() -> Result<(), test::Error> {
        let req = Request {
            method: Method::Get,
            uri: "https://example.com",
            headers: &[],
            params: &[],
            body: None,
        };
        let res = wasi_outbound_http::request(req, None).unwrap();
        let body = &res.body.unwrap();
        let body = std::str::from_utf8(body).unwrap();

        println!("Status: {}", res.status);
        println!("Body: {}", body);

        assert_eq!(200, res.status);

        Ok(())
    }
}
