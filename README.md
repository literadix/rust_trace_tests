# Enable tracing in rust to a stdout or log files

<a href="https://github.com/literadix/rust_trace_tests/actions">
  <img src="https://github.com/literadix/rust_trace_tests/actions/workflows/rust.yml/badge.svg" height="20" alt="Build workflow">
</a>


My private example how to use tracing, logging and testing in rust.
Feel free to play with it and copy code to your projects.
Merge requests, comments etc are welcome.

> let _guard = init_tracing(true, Level::TRACE);

```
cargo run                   
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target\debug\test_tokio.exe`
2023-05-11T06:00:04.094565Z DEBUG main ThreadId(01) something happened inside my_span
2023-05-11T06:00:04.094821Z TRACE main ThreadId(01) main:hello_string: return="hello world 🦀 ❤ ❤\u{fe0f} ☮ 翿"
2023-05-11T06:00:04.094904Z  INFO main ThreadId(01) main: hello world 🦀 ❤ ❤️ ☮ 翿
2023-05-11T06:00:04.094917Z  INFO main ThreadId(01) main: 340282366920938463463374607431768211455
```

Or when switched to file output:

> let _guard = init_tracing(false, Level::TRACE);


```
cargo run
   Compiling test_tokio v0.1.0 (C:\Users\Maciej Bednarz\projects\rust\test_tokio)
    Finished dev [unoptimized + debuginfo] target(s) in 1.52s
     Running `target\debug\test_tokio.exe`
     
cat logs/service.log.2023-05-11

2023-05-11T06:51:25.627924Z DEBUG main ThreadId(01) something happened inside my_span
2023-05-11T06:51:25.628087Z TRACE main ThreadId(01) main:hello_string: return="hello world 🦀 ❤ ❤\u{fe0f} ☮ 翿"
2023-05-11T06:51:25.628158Z  INFO main ThreadId(01) main: hello world 🦀 ❤ ❤️ ☮ 翿
2023-05-11T06:51:25.628182Z  INFO main ThreadId(01) main: 340282366920938463463374607431768211455

```

Implemented by this function:

```
/// Initialize tracing
pub fn init_tracing(
    stdout: bool,
    filter: tracing::Level,
) -> tracing_appender::non_blocking::WorkerGuard {
    // Decide which output should be used
    let (writer, guard) = if stdout {
        let (writer, guard) = tracing_appender::non_blocking(std::io::stdout());
        (writer, guard)
    } else {
        let file_appender = tracing_appender::rolling::daily("logs", "service.log");
        let (writer, guard) = tracing_appender::non_blocking(file_appender);
        (writer, guard)
    };

    // Initialize tracing instance
    tracing_subscriber::fmt()
        .with_writer(writer)
        .with_max_level(filter)
        .with_ansi(stdout)
        .with_target(false)
        .with_file(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    guard
}
```

Usage example:

```
mod helpers;
use helpers::*;

use std::str::{self};

use tracing::{debug, info, span, Level};

#[tracing::instrument(ret, level=Level::TRACE)]
async fn hello_string<'life>() -> &'life str {
    "hello world 🦀 ❤ ❤️ ☮ \u{7fff}"
}

#[tokio::main]
async fn main() {

    let _guard = init_tracing(false, Level::TRACE);
    let op = hello_string();

    // records an event within "my_span".
    debug!("something happened inside my_span");
    let span = span!(Level::INFO, "main");
    let _guard = span.enter();

    let res = op.await;
    info!("{}", res);
    info!("{}", u128::MAX);
}
```

