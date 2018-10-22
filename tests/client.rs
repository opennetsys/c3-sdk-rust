#[cfg(test)]

mod tests {
    extern crate c3_sdk;

    use std::collections::HashMap;
    use self::c3_sdk::client::{Client, State};

    #[test]
    fn it_works() {
        let mut client = Client::new();
        client.register_method("setItem".to_string(), |key: String, val: String| -> String {
            let store = HashMap::new();
            let mut state = State::new(store);
            state.set(key.clone(), val);
            return key;
        });
        //client.serve();
        assert_eq!(true, true);
    }
}
