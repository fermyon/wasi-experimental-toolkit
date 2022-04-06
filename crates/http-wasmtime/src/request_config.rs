use crate::wasi_outbound_http::*;

#[derive(Clone, Debug)]
pub(crate) struct ReqwestConfig {
    pub accept_invalid_hostnames: bool,
    pub accept_invalid_certificates: bool,
    pub extra_root_certificates: Vec<reqwest::Certificate>,
    pub identity: Option<reqwest::Identity>,
}

impl TryFrom<Identity<'_>> for reqwest::Identity {
    type Error = String;

    fn try_from(identity: Identity) -> Result<Self, Self::Error> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "native-tls")] {
                let pkey = openssl::pkey::PKey::private_key_from_pem(&identity.key)
                    .map_err(|e| format!("Cannot convert identity: {}", e))?;
                let cert = openssl::x509::X509::from_pem(&identity.cert)
                    .map_err(|e| format!("Cannot convert identity: {}", e))?;
                let pkcs12 = openssl::pkcs12::Pkcs12::builder().build("", "", &pkey, &cert)
                    .map_err(|e| format!("Cannot convert identity: {}", e))?;
                let pkcs12_der = pkcs12.to_der()
                    .map_err(|e| format!("Cannot convert identity: {}", e))?;
                reqwest::Identity::from_pkcs12_der(&pkcs12_der, "")
                    .map_err(|e| format!("Cannot convert identity: {}", e))
            } else if #[cfg(feature = "rustls-tls")] {
                let mut pem_bundle = identity.key.clone();
                if pem_bundle[pem_bundle.len() - 1] != b'\n' {
                    pem_bundle.insert(pem_bundle.len(), b'\n');
                }
                pem_bundle.extend_from_slice(&identity.cert);
                reqwest::Identity::from_pem(&pem_bundle.into())
                    .map_err(|e| format!("Cannot create identity: {:?}", e))
            } else {
                Err("Cannot create reqwest identity, neither 'native-tls' feature nor '__rusttls' one are enabled".to_string())
            }
        }
    }
}

impl TryFrom<RequestConfig<'_>> for ReqwestConfig {
    type Error = String;

    fn try_from(cfg: RequestConfig) -> Result<Self, Self::Error> {
        let mut extra_root_certificates: Vec<reqwest::Certificate> = vec![];

        for c in cfg.extra_root_certificates {
            let cert = match c.encoding {
                CertificateEncoding::Pem => reqwest::Certificate::from_pem(c.data),
                CertificateEncoding::Der => reqwest::Certificate::from_der(c.data),
            }
            .map_err(|e| format!("Cannot convert certificate: {}", e))?;
            extra_root_certificates.push(cert);
        }

        let identity = match cfg.identity {
            Some(id) => Some(id.try_into()?),
            None => None,
        };

        Ok(ReqwestConfig {
            accept_invalid_certificates: cfg.accept_invalid_certificates,
            accept_invalid_hostnames: cfg.accept_invalid_hostnames,
            extra_root_certificates,
            identity,
        })
    }
}
