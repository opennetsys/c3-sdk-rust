use std::collections::HashMap;
use self::c3_sdk::client::{Client, State};

fn main() {
    let mut client = client::Client::new();
    client.register_method("setItem".to_string(), |key: String, val: String| -> String {
        let store = HashMap::new();
        let mut state = client::State::new(store);
        state.set(key.clone(), val);
        return key;
    });
    client.serve();
}
