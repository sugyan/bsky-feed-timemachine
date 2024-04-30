use crate::auth::{self, verify_jwt, SigningKeyProvider};
use crate::client::{FetchClient, FetchHttpClient};
use crate::identity::did::did_resolver::{DidResolver, Resolver};
use atrium_api::client::AtpServiceClient;
use atrium_api::types::LimitedNonZeroU8;
use chrono::{Months, SecondsFormat, Utc};
use serde::Deserialize;
use worker::{console_error, console_log, Env, Request, Response, Result};

#[derive(Debug, Deserialize)]
struct Query {
    #[allow(dead_code)]
    feed: String,
    limit: Option<LimitedNonZeroU8<100>>,
}

pub async fn feed_skeleton(req: &Request, env: &Env) -> Result<Response> {
    let mut feed = Vec::new();
    let query = req.query::<Query>()?;
    if let Some(did) = get_user_did(req, env).await? {
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

struct KeyProvider<T> {
    did_resolver: DidResolver<T>,
}

impl SigningKeyProvider for KeyProvider<FetchHttpClient> {
    async fn get_signing_key(&self, iss: &str, force_refresh: bool) -> auth::Result<String> {
        self.did_resolver
            .resolve_atproto_key(iss, force_refresh)
            .await
            .map_err(auth::Error::DidResolver)
    }
}

async fn get_user_did(req: &Request, env: &Env) -> Result<Option<String>> {
    let token = req
        .headers()
        .get("Authorization")?
        .and_then(|v| v.strip_prefix("Bearer ").map(|s| s.to_string()));
    if let Some(jwt) = token {
        match verify_jwt(
            &jwt,
            Some(&env.var("SERVICE_DID")?.to_string()),
            KeyProvider {
                did_resolver: DidResolver::new(FetchHttpClient, "https://plc.directory"),
            },
        )
        .await
        {
            Ok(payload) => {
                console_log!("verified jwt: {payload:?}");
                return Ok(Some(payload.iss));
            }
            Err(err) => console_error!("failed to verify jwt: {err}"),
        }
    }
    Ok(None)
}
