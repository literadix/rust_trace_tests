mod trace;
use trace::init_tracing;

use mini_redis::{client, Result};
use std::str::{self};

use tracing::{debug, info, span, warn, Level};

#[tracing::instrument(ret, level=Level::TRACE)]
async fn hello_string<'life>() -> &'life str {
    return "hello world";
}

#[tokio::main]
async fn main() -> Result<()> {
    //https://docs.rs/spinners/latest/spinners/struct.Spinner.html
    //https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
    let _guard = init_tracing(true, Level::TRACE);
    let op = hello_string();

    // records an event within "my_span".
    debug!("something happened inside my_span");
    let span = span!(Level::INFO, "main");
    let _guard = span.enter();

    // Open a connection to the mini-redis address (start mini-redis-server)
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world!".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    match result {
        Some(val) => {
            let val_str = str::from_utf8(&val).unwrap();
            info!("got value from the server; result={:?}", val_str);
        }
        None => {
            warn!("No result!");
        }
    }

    let res = op.await;
    info!("{}", res);

    Ok(())
}
