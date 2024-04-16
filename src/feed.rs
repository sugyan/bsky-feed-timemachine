use serde::Deserialize;
use worker::{console_log, Request, Response, Result};

#[derive(Debug, Deserialize)]
struct Query {
    feed: String,
    limit: Option<usize>,
}

pub fn feed_skeleton(req: &Request) -> Result<Response> {
    console_log!("{:?}", req.query::<Query>());
    console_log!("{:?}", req.headers());
    Response::from_json(&atrium_api::app::bsky::feed::get_feed_skeleton::Output {
        cursor: None,
        feed: vec![],
    })
}
