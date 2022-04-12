use futures::executor::block_on;
use http::HeaderMap;
use reqwest::{Client, Url};
use std::collections::HashMap;
use std::{
    convert::TryFrom,
    str::FromStr,
    sync::{Arc, RwLock},
};
use tokio::runtime::Handle;
use tracing::{debug, error};
use wasi_outbound_http::*;

mod request_config;
use request_config::*;

pub use wasi_outbound_http::add_to_linker;

wit_bindgen_wasmtime::export!("wit/ephemeral/wasi-outbound-http.wit");

/// A very simple implementation for outbound HTTP requests.
#[derive(Default, Clone)]
pub struct OutboundHttp {
    /// List of hosts guest modules are allowed to make requests to.
    pub allowed_hosts: Arc<Option<Vec<String>>>,
    request_configs: Arc<RwLock<HashMap<String, ReqwestConfig>>>,
}

impl OutboundHttp {
    pub fn new(allowed_hosts: Option<Vec<String>>) -> Self {
        let allowed_hosts = Arc::new(allowed_hosts);
        let cfg: HashMap<String, ReqwestConfig> = HashMap::new();
        let request_configs = Arc::new(RwLock::new(cfg));
        Self {
            allowed_hosts,
            request_configs,
        }
    }

    /// Check if guest module is allowed to send request to URL, based on the list of
    /// allowed hosts defined by the runtime.
    /// If `None` is passed, the guest module is not allowed to send the request.
    fn is_allowed(url: &str, allowed_hosts: Arc<Option<Vec<String>>>) -> Result<bool, HttpError> {
        let url_host = Url::parse(url)
            .map_err(|_| HttpError::InvalidUrl)?
            .host_str()
            .ok_or(HttpError::InvalidUrl)?
            .to_owned();
        match allowed_hosts.as_deref() {
            Some(domains) => {
                let allowed: Result<Vec<_>, _> = domains.iter().map(|d| Url::parse(d)).collect();
                let allowed = allowed.map_err(|_| HttpError::InvalidUrl)?;
                let a: Vec<&str> = allowed.iter().map(|u| u.host_str().unwrap()).collect();
                Ok(a.contains(&url_host.as_str()))
            }
            None => {
                error!("allowed_hosts is empty, blocking the request");
                Ok(false)
            }
        }
    }
}

impl wasi_outbound_http::WasiOutboundHttp for OutboundHttp {
    fn register_request_config(
        &mut self,
        config: RequestConfig,
        id: Option<&str>,
    ) -> Result<String, HttpError> {
        let id = id.map_or_else(|| uuid::Uuid::new_v4().to_string(), |i| i.to_string());
        let mut hash = self.request_configs.write().unwrap();

        let cfg: ReqwestConfig = config.try_into().map_err(|e| {
            error!(error =? e, "cannot convert request config");
            HttpError::InvalidCfg
        })?;

        hash.insert(id.clone(), cfg);

        return Ok(id);
    }

    fn request(&mut self, req: Request, config: Option<&str>) -> Result<Response, HttpError> {
        if !Self::is_allowed(&req.uri, self.allowed_hosts.clone())? {
            return Err(HttpError::DestinationNotAllowed);
        }

        let reqwest_config = config
            .map(|id| {
                let hash = self.request_configs.read().unwrap();
                hash.get(id).map(|i| i.clone()).ok_or_else(|| {
                    error!(?id, "cannot find request config");
                    HttpError::InvalidCfg
                })
            })
            .transpose()?;

        let method = http::Method::from(req.method);
        let url = Url::parse(req.uri).map_err(|_| HttpError::InvalidUrl)?;
        let headers = headers(req.headers)?;
        let body = req.body.unwrap_or_default().to_vec();

        // TODO (@radu-matei)
        // Ensure all  HTTP request and response objects are handled properly (query parameters, headers).

        match Handle::try_current() {
            // If running in a Tokio runtime, spawn a new blocking executor
            // that will send the HTTP request, and block on its execution.
            // This attempts to avoid any deadlocks from other operations
            // already executing on the same executor (compared with just
            // blocking on the current one).
            Ok(r) => block_on(r.spawn_blocking(move || -> Result<Response, HttpError> {
                debug!("running request inside of new blocking executor");
                let mut client_builder = Client::builder();
                if let Some(rc) = reqwest_config {
                    debug!(request_config = ?rc, "using request config");
                    client_builder =
                        client_builder.danger_accept_invalid_certs(rc.accept_invalid_certificates);

                    cfg_if::cfg_if! {
                        if #[cfg(feature = "native-tls")] {
                            client_builder = client_builder
                                .danger_accept_invalid_hostnames(rc.accept_invalid_hostnames);
                        } else {
                            if rc.accept_invalid_hostnames {
                                tracing::info!("request config: accept_invalid_hostnames cannot be enabled when rustls is used");
                            }
                        }
                    }

                    if let Some(identity) = rc.identity {
                        client_builder = client_builder.identity(identity);
                    }

                    for cert in rc.extra_root_certificates {
                        client_builder = client_builder.add_root_certificate(cert);
                    }
                }
                let client = client_builder.build().unwrap();
                let res = block_on(
                    client
                        .request(method, url)
                        .headers(headers)
                        .body(body)
                        .send(),
                );
                if let Err(e) = &res {
                    error!(error =? e, "http request failure");
                }
                Ok(Response::try_from(res?)?)
            }))
            .map_err(|_| HttpError::RuntimeError)?,
            Err(_) => {
                debug!("running request using blocking client");
                let mut client_builder = reqwest::blocking::Client::builder();
                if let Some(rc) = reqwest_config {
                    debug!(request_config = ?rc, "using request config");
                    client_builder =
                        client_builder.danger_accept_invalid_certs(rc.accept_invalid_certificates);

                    cfg_if::cfg_if! {
                        if #[cfg(feature = "native-tls")] {
                            client_builder = client_builder
                                .danger_accept_invalid_hostnames(rc.accept_invalid_hostnames);
                        } else {
                            if rc.accept_invalid_hostnames {
                                tracing::info!("request config: accept_invalid_hostnames cannot be enabled when rustls is used");
                            }
                        }
                    }

                    if let Some(identity) = rc.identity {
                        client_builder = client_builder.identity(identity);
                    }

                    for cert in rc.extra_root_certificates {
                        client_builder = client_builder.add_root_certificate(cert);
                    }
                }
                let client = client_builder.build().unwrap();
                let res = client
                    .request(method, url)
                    .headers(headers)
                    .body(body)
                    .send();
                if let Err(e) = &res {
                    error!(error =? e, "http request failure");
                }
                Ok(Response::try_from(res?)?)
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
