use crate::client::FetchClient;
use atrium_api::client::AtpServiceClient;
use atrium_api::types::LimitedNonZeroU8;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use chrono::{Months, SecondsFormat, Utc};
use serde::Deserialize;
use worker::{console_error, console_log, Request, Response, Result};

#[derive(Debug, Deserialize)]
struct Query {
    #[allow(dead_code)]
    feed: String,
    limit: Option<LimitedNonZeroU8<100>>,
}

#[derive(Debug, Deserialize)]
struct Payload {
    iss: String,
    #[allow(dead_code)]
    aud: String,
    #[allow(dead_code)]
    exp: u64,
}

pub async fn feed_skeleton(req: &Request) -> Result<Response> {
    let mut feed = Vec::new();
    let query = req.query::<Query>()?;
    if let Some(did) = get_user_did(req)? {
        let params = atrium_api::app::bsky::feed::search_posts::Parameters {
            author: did.parse().ok(),
            cursor: None,
            domain: None,
            lang: None,
            limit: query.limit,
            mentions: None,
            q: did,
            since: None,
            sort: None,
            tag: None,
            until: Utc::now()
                .fixed_offset()
                .checked_sub_months(Months::new(6))
                .map(|dt| dt.to_rfc3339_opts(SecondsFormat::Micros, true)),
            url: None,
        };
        console_log!("params: {params:?}");
        let client = AtpServiceClient::new(FetchClient::new("https://api.bsky.app"));
        match client.service.app.bsky.feed.search_posts(params).await {
            Ok(output) => {
                for post in output.posts {
                    feed.push(atrium_api::app::bsky::feed::defs::SkeletonFeedPost {
                        feed_context: None,
                        post: post.uri,
                        reason: None,
                    });
                }
            }
            Err(err) => {
                console_error!("failed to search post: {err}");
            }
        }
    }
    Response::from_json(&atrium_api::app::bsky::feed::get_feed_skeleton::Output {
        cursor: None,
        feed,
    })
}

fn get_user_did(req: &Request) -> Result<Option<String>> {
    let token = req
        .headers()
        .get("Authorization")?
        .and_then(|v| v.strip_prefix("Bearer ").map(|s| s.to_string()));
    // TODO: verify JWT
    if let Some(jwt) = token {
        Ok(jwt
            .split('.')
            .nth(1)
            .and_then(|s| URL_SAFE.decode(s).ok())
            .and_then(|v| serde_json::from_slice::<Payload>(&v).ok())
            .map(|payload| payload.iss))
    } else {
        Ok(None)
    }
}
