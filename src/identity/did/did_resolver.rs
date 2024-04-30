use super::atproto_data;
use super::plc_resolver::DidPlcResolver;
use crate::common_web::did_doc::DidDocument;
use atrium_api::xrpc::HttpClient;
use serde_json::from_slice;
use std::future::Future;

#[derive(Debug)]
pub enum Error {
    DidNotFoundError(String),
    PoorlyFormattedDid(String),
    UnsupportedDidMethod(String),
    Http(http::Error),
    HttpClient(Box<dyn std::error::Error + Send + Sync + 'static>),
    SerdeJson(serde_json::Error),
    AtprotoData(atproto_data::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::DidNotFoundError(did) => write!(f, "Could not resolve DID: {did}"),
            Error::PoorlyFormattedDid(did) => write!(f, "Poorly formatted DID: {did}"),
            Error::UnsupportedDidMethod(did) => write!(f, "Unsupported DID method: {did}"),
            Error::Http(err) => write!(f, "HTTP error: {err}"),
            Error::HttpClient(err) => write!(f, "HttpClient error: {err}"),
            Error::SerdeJson(err) => write!(f, "SerdeJson error: {err}"),
            Error::AtprotoData(err) => write!(f, "AtprotoData error: {err}"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Resolver {
    fn resolve_no_check(&self, did: &str) -> impl Future<Output = Result<Option<Vec<u8>>>>;

    fn resolve(
        &self,
        did: &str,
        force_refresh: bool,
    ) -> impl Future<Output = Result<Option<DidDocument>>> {
        async move {
            // TODO: from cache?
            if let Some(got) = self.resolve_no_cache(did).await? {
                // TODO: store in cache
                Ok(Some(got))
            } else {
                // TODO: clear from cache
                Ok(None)
            }
        }
    }
    fn resolve_no_cache(&self, did: &str) -> impl Future<Output = Result<Option<DidDocument>>> {
        async move {
            Ok(if let Some(got) = self.resolve_no_check(did).await? {
                Some(from_slice(got.as_slice()).map_err(Error::SerdeJson)?)
            } else {
                None
            })
        }
    }
    fn ensure_resolve(
        &self,
        did: &str,
        force_refresh: bool,
    ) -> impl Future<Output = Result<DidDocument>> {
        async move {
            self.resolve(did, force_refresh)
                .await?
                .ok_or(Error::DidNotFoundError(did.into()))
        }
    }
    fn resolve_atproto_key(
        &self,
        did: &str,
        force_refresh: bool,
    ) -> impl Future<Output = Result<String>> {
        async move {
            if did.starts_with("did:key:") {
                Ok(did.into())
            } else {
                let did_document = self.ensure_resolve(did, force_refresh).await?;
                Ok(atproto_data::ensure_atproto_key(&did_document).map_err(Error::AtprotoData)?)
            }
        }
    }
}

pub struct DidResolver<T> {
    plc: DidPlcResolver<T>,
}

impl<T> DidResolver<T> {
    pub fn new(client: T, plc_url: impl AsRef<str>) -> Self {
        Self {
            plc: DidPlcResolver::new(client, plc_url),
        }
    }
}

impl<T> Resolver for DidResolver<T>
where
    T: HttpClient,
{
    async fn resolve_no_check(&self, did: &str) -> Result<Option<Vec<u8>>> {
        println!("Resolving DID: {did}");
        let parts = did.split(':').collect::<Vec<_>>();
        if parts.len() < 2 || parts[0] != "did" {
            return Err(Error::PoorlyFormattedDid(did.into()));
        }
        let method = parts[1];
        match method {
            "plc" => self.plc.resolve_no_check(did).await,
            "web" => unimplemented!(),
            _ => Err(Error::UnsupportedDidMethod(did.into())),
        }
    }
}
