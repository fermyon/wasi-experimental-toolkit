use futures::executor::block_on;
use http::HeaderMap;
use outbound_http::*;
use reqwest::{Client, Url};
use std::str::FromStr;
use tokio::runtime::Handle;

wit_bindgen_wasmtime::export!("wit/ephemeral/outbound_http.wit");

struct OutboundHttp {}

impl outbound_http::OutboundHttp for OutboundHttp {
    fn request(&mut self, req: Request<'_>) -> Result<Response, HttpError> {
        let method = http::Method::from(req.method);
        let url = Url::parse(req.uri).map_err(|_| HttpError::InvalidUrl)?;
        let headers = headers(req.headers)?;
        let body = req.body.unwrap_or_default().to_vec();

        match Handle::try_current() {
            // If running in a Tokio runtime, spawn a new blocking executor
            // that will send the HTTP request, and block on its execution.
            // This attempts to avoid any deadlocks from other operations
            // already executing on the same executor (compared with just
            // blocking on the current one).
            Ok(r) => block_on(r.spawn_blocking(move || -> Result<Response, HttpError> {
                let client = Client::builder().build().unwrap();
                let res = block_on(
                    client
                        .request(method, url)
                        .headers(headers)
                        .body(body)
                        .send(),
                )?;

                Ok(Response::try_from(res)?)
            }))
            .map_err(|_| HttpError::RuntimeError)?,
            Err(_) => {
                let res = reqwest::blocking::Client::new()
                    .request(method, url)
                    .headers(headers)
                    .body(body)
                    .send()?;
                Ok(Response::try_from(res)?)
            }
        }
    }
}

impl From<Method> for http::Method {
    fn from(m: Method) -> Self {
        match m {
            Method::Get => http::Method::GET,
            Method::Post => http::Method::POST,
            Method::Put => http::Method::PUT,
            Method::Delete => http::Method::DELETE,
            Method::Patch => http::Method::PATCH,
            Method::Head => http::Method::HEAD,
            Method::Options => http::Method::OPTIONS,
        }
    }
}

impl TryFrom<reqwest::Response> for Response {
    type Error = HttpError;

    fn try_from(res: reqwest::Response) -> Result<Self, Self::Error> {
        let status = res.status().as_u16();
        // TODO (@radu-matei)
        let headers = Some(Vec::new());
        let body = Some(block_on(res.bytes())?.to_vec());

        Ok(Response {
            status,
            headers,
            body,
        })
    }
}

impl TryFrom<reqwest::blocking::Response> for Response {
    type Error = HttpError;

    fn try_from(res: reqwest::blocking::Response) -> Result<Self, Self::Error> {
        let status = res.status().as_u16();
        // TODO (@radu-matei)
        let headers = Some(Vec::new());
        let body = Some(res.bytes()?.to_vec());

        Ok(Response {
            status,
            headers,
            body,
        })
    }
}

fn headers(h: HeadersParam) -> anyhow::Result<HeaderMap> {
    let mut res = HeaderMap::new();
    for (k, v) in h {
        res.insert(
            http::header::HeaderName::from_str(k)?,
            http::header::HeaderValue::from_str(v)?,
        );
    }
    Ok(res)
}

impl From<anyhow::Error> for HttpError {
    fn from(_: anyhow::Error) -> Self {
        Self::RuntimeError
    }
}

impl From<reqwest::Error> for HttpError {
    fn from(_: reqwest::Error) -> Self {
        Self::RequestError
    }
}
