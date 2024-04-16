use serde::Serialize;
use worker::{Env, Response, Result};

#[derive(Serialize)]
struct DidDoc {
    #[serde(rename = "@context")]
    context: Vec<String>,
    id: String,
    service: Vec<Service>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Service {
    id: String,
    r#type: String,
    service_endpoint: String,
}

pub fn did_doc(env: &Env) -> Result<Response> {
    Response::from_json(&DidDoc {
        context: vec![String::from("https://www.w3.org/ns/did/v1")],
        id: env.var("SERVICE_DID")?.to_string(),
        service: vec![Service {
            id: String::from("#bsky_fg"),
            r#type: String::from("BskyFeedGenerator"),
            service_endpoint: env.var("SERVICE_ENDPOINT")?.to_string(),
        }],
    })
}
