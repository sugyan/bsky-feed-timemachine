use atrium_api::agent::{store::MemorySessionStore, AtpAgent};
use atrium_api::records::{KnownRecord, Record};
use atrium_api::types::string::{AtIdentifier, Datetime, Nsid};
use atrium_api::types::Collection;
use atrium_xrpc_client::reqwest::ReqwestClient;
use std::env;
use std::str::FromStr;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent = AtpAgent::new(
        ReqwestClient::new("https://bsky.social"),
        MemorySessionStore::default(),
    );
    let session = agent
        .login(
            env::var("BLUESKY_IDENTIFIER")?,
            env::var("BLUESKY_PASSWORD")?,
        )
        .await?;
    println!(
        "feeds: {:?}",
        agent
            .api
            .app
            .bsky
            .feed
            .get_actor_feeds(atrium_api::app::bsky::feed::get_actor_feeds::Parameters {
                actor: AtIdentifier::Did(session.did.clone()),
                cursor: None,
                limit: 10.try_into().ok(),
            })
            .await?
    );
    println!(
        "{:?}",
        agent
            .api
            .com
            .atproto
            .repo
            .put_record(atrium_api::com::atproto::repo::put_record::Input {
                collection: Nsid::from_str(atrium_api::app::bsky::feed::Generator::NSID)
                    .expect("invalid nsid"),
                record: Record::Known(KnownRecord::AppBskyFeedGenerator(Box::new(
                    atrium_api::app::bsky::feed::generator::Record {
                        avatar: None,
                        created_at: Datetime::now(),
                        description: None,
                        description_facets: None,
                        did: env::var("SERVICE_DID")?
                            .parse()
                            .expect("invalid did"),
                        display_name: String::from("test feed generator"),
                        labels: None,
                    }
                ))),
                repo: AtIdentifier::Did(session.did.clone()),
                rkey: String::from("test"),
                swap_commit: None,
                swap_record: None,
                validate: None
            })
            .await?
    );

    Ok(())
}
