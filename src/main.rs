mod helpers;
use helpers::*;

use std::str::{self};

use tracing::{debug, info, span, Level};

#[tracing::instrument(ret, level=Level::TRACE)]
async fn hello_string<'life>() -> &'life str {
    return "hello world 🦀 ❤ ❤️\u{7fff}";
}

#[tokio::main]
async fn main()      {
    //https://docs.rs/spinners/latest/spinners/struct.Spinner.html
    //https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
    let _guard = init_tracing(true, Level::TRACE);
    let op = hello_string();

    // records an event within "my_span".
    debug!("something happened inside my_span");
    let span = span!(Level::INFO, "main");
    let _guard = span.enter();

    let res = op.await;
    info!("{}", res);
    info!("{}", u128::MAX);

}
