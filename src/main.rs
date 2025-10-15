use anyhow::Result;
use log::info;
use mongodb::{Client, bson::doc};
use std::{env, time::Duration};
use tokio::time::timeout;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let uri = env::args().into_iter().nth(1).unwrap();
    let client = Client::with_uri_str(uri).await?;

    let db = client.database("admin");

    loop {
        let cmd = doc! { "ping": 1 };
        let res = timeout(Duration::from_secs(15), db.run_command(cmd)).await;
        match res {
            Ok(Ok(doc)) => {
                if doc.get_i32("ok").map_or(false, |ok| ok == 1)
                    || doc.get_f64("ok").map_or(false, |ok| ok == 1.0)
                {
                    info!("ping returned a document with an `ok`");
                } else {
                    info!(
                        "ping returned a document without an `ok`: {}",
                        serde_json::to_string_pretty(&doc)?
                    );
                }
            }
            Err(e) => info!("ping command timed out: {e}"),
            Ok(Err(e)) => info!("ping returned error: {e}"),
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
