# c3-sdk-rust

> The [C3](https://github.com/c3systems/c3-go) SDK for Rust.

[![License](http://img.shields.io/badge/license-GNU%20AGPL%203.0-blue.svg)](https://raw.githubusercontent.com/c3systems/c3-sdk-rust-sdk/master/LICENSE) [![Crates.io](https://img.shields.io/crates/v/c3_sdk.svg)](https://crates.io/crates/c3_sdk) [![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental)

NOTE: This is a work-in-progress.

## Install

```bash
cargo install c3_sdk
```

## Getting started

```rust
extern crate c3_sdk;

use std::collections::HashMap;
use self::c3_sdk::client::{Client, State};

fn main() {
    let mut client = Client::new();
    client.register_method("setItem".to_string(), |key: String, val: String| -> String {
        let store = HashMap::new();
        let mut state = State::new(store);
        state.set(key.clone(), val);
        return key;
    });
    client.serve();
}
```

## Test

```bash
make test
```

## License

[GNU AGPL 3.0](LICENSE)
