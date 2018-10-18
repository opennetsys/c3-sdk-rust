#[macro_use]
extern crate serde_derive;
use std::collections::HashMap;
mod client;

fn main() {
    let mut client = client::Client::new();
    client.register_method("setItem".to_string(), |key: String, val: String| -> String {
        let store = HashMap::new();
        let mut state = client::State::new(store);
        state.set(key.clone(), val);
        return key;
    });
    client.serve();
    let json = r#"["setItem","foo","bar"]"#;
    client.kit.process_payload(json.to_string());
}
