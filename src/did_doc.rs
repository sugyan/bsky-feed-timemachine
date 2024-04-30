use crate::common_web::did_doc::{DidDocument, Service};
use worker::{Env, Response, Result};

pub fn did_doc(env: &Env) -> Result<Response> {
    Response::from_json(&DidDocument {
        context: Some(vec![String::from("https://www.w3.org/ns/did/v1")]),
        id: env.var("SERVICE_DID")?.to_string(),
        also_known_as: None,
        verification_method: None,
        service: Some(vec![Service {
            id: String::from("#bsky_fg"),
            r#type: String::from("BskyFeedGenerator"),
            service_endpoint: env.var("SERVICE_ENDPOINT")?.to_string(),
        }]),
    })
}
