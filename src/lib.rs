mod did_doc;
mod feed;

use did_doc::did_doc;
use feed::feed_skeleton;
use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    match req.url()?.path() {
        "/xrpc/app.bsky.feed.getFeedSkeleton" => feed_skeleton(&req),
        "/.well-known/did.json" => did_doc(&env),
        _ => Response::error("Not Found", 404),
    }
}
