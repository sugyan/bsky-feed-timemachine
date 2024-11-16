use atrium_api::agent::{store::MemorySessionStore, AtpAgent};
use atrium_api::types::string::{AtIdentifier, Nsid};
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
    let rkey = env::var("RECORD_KEY")?;
    let input = atrium_api::com::atproto::repo::delete_record::Input {
        collection: Nsid::from_str(atrium_api::app::bsky::feed::Generator::NSID)?,
        repo: AtIdentifier::Did(session.did),
        rkey,
        swap_commit: None,
        swap_record: None,
    };
    println!(
        "{:?}",
        agent.api.com.atproto.repo.delete_record(input).await?
    );
    Ok(())
}
