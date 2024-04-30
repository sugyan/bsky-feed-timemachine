use super::did_resolver::{Error, Resolver, Result};
use atrium_api::xrpc::HttpClient;
use http::Request;

pub struct DidPlcResolver<T> {
    client: T,
    plc_url: String,
}

impl<T> DidPlcResolver<T> {
    pub fn new(client: T, plc_url: impl AsRef<str>) -> Self {
        Self {
            client,
            plc_url: plc_url.as_ref().into(),
        }
    }
}

impl<T> Resolver for DidPlcResolver<T>
where
    T: HttpClient,
{
    async fn resolve_no_check(&self, did: &str) -> Result<Option<Vec<u8>>> {
        let response = self
            .client
            .send_http(
                Request::get(format!("{}/{did}", self.plc_url))
                    .body(Vec::new())
                    .map_err(Error::Http)?,
            )
            .await
            .map_err(Error::HttpClient)?;
        Ok(if response.status().is_success() {
            Some(response.body().to_vec())
        } else {
            None
        })
    }
}
