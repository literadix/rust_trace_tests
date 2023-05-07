// build.rs (sample pre-build script)

use std::env;

fn main() {
    // You need to rely on env. vars for target; `#[cfg(…)]` are for host.
    let _target_os = env::var("CARGO_CFG_TARGET_OS");
}
