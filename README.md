# `wasm-bindgen-console-logger`

This small utility crate integrates the [`log`](https://crates.io/crates/log)
crate with the JavaScript console logging functions with the help of
[`wasm-bindgen`](https://crates.io/crates/wasm-bindgen).

## Example

```rust
use log::{error, info, warn};
use wasm_bindgen::prelude::*;
use wasm_bindgen_console_logger::DEFAULT_LOGGER;

#[wasm_bindgen]
pub fn start() {
    log::set_logger(&DEFAULT_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    error!("Error message");
    warn!("Warning message");
    info!("Informational message");
}
```

## Related libraries

By sheer coincidence, this crate was published at almost exactly the same
time as [`console_log`](https://crates.io/crates/console_log). Assuming that
it receives continued maintenance, `console_log` may end up being the "go-to"
option for logging in WebAssembly applications, but I'm planning to provide
at least basic maintenance on this crate until the situation becomes clearer.
