#![cfg(target_arch = "wasm32")]

pub mod auth;
pub mod common_web;
pub mod crypto;
pub mod identity;

mod client;
mod did_doc;
mod feed;

use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    match req.url()?.path() {
        "/xrpc/app.bsky.feed.getFeedSkeleton" => crate::feed::feed_skeleton(&req, &env).await,
        "/.well-known/did.json" => crate::did_doc::did_doc(&env),
        _ => Response::error("Not Found", 404),
    }
}
