extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, BufWriter, Write};
use std::str;

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    store: HashMap<String, String>
}

static STATEFILE : &str = "/tmp/state.json";

impl State {
    pub fn new(store: HashMap<String, String>) -> State {
        return State{
            store: store,
        };
    }
    pub fn set(&mut self, key: String, val: String) {
        self.store.entry(key)
            .and_modify(|e| {
                *e = val.clone()
            })
            .or_insert(val);

            let data = serde_json::to_string(&self.store).unwrap();
            let f = File::create(STATEFILE).expect("Unable to create file");
            let mut f = BufWriter::new(f);
            f.write_all(data.as_bytes()).expect("Unable to write to file");
            println!("wrote to tmp state file");
    }
    pub fn get(&mut self, key: &str) -> String {
        let mut data = String::new();
        let mut f = File::open(&STATEFILE).expect("Unable to open file");
        f.read_to_string(&mut data).expect("Unable to read file");

        let store: HashMap<String, String> = serde_json::from_str(&data).unwrap();
        self.store = store;

        let result = &self.store[key];
        return result.to_string();
    }
}

pub type Callback = fn(String, String) -> String;

pub struct Kit {
    registered_methods: HashMap<String, Callback>,
    state: State,
}

impl Kit {
    pub fn new() -> Kit{
        // TODO
        // emitter = new emitter
        let registered_methods = HashMap::new();
        let store = HashMap::new();
        let state = State::new(store);

        return Kit{
            registered_methods: registered_methods,
            state: state,
        };
    }
    pub fn set_initial_state(&mut self) {
        println!("initial state");
        let mut data = String::new();
        let mut f = File::open(&STATEFILE).expect("Unable to open file");
        f.read_to_string(&mut data).expect("Unable to read file");

        let store: HashMap<String, String> = serde_json::from_str(&data).unwrap();
        self.state.store = store;
    }
    pub fn listen(&self) {
        println!("listening");
        // TODO:
        //emitter.on('data', fn {
        //self.process_payload(data)
        //})
    }
    pub fn process_payload(&mut self, payload: String) {
        println!("processing");
        let params: [String; 3] = serde_json::from_str(&payload).unwrap();
        let method_name = params[0].to_string();
        let mut args: [String; 2] = Default::default();
        args.clone_from_slice(&params[1..]);
        self.invoke(method_name, args);
    }
    pub fn invoke(&mut self, method_name: String, params: [String; 2]) {
        println!("invoking {}", method_name);
        let f = &self.registered_methods[&method_name];
        f(params[0].to_string(), params[1].to_string());
    }
}

pub struct Client {
    pub kit: Kit,
}

impl Client {
    pub fn new() -> Client {
        let mut kit = Kit::new();
        kit.set_initial_state();
        kit.listen();

        return Client{
            kit: kit,
        };
    }
    pub fn refresh_state(&mut self) {
        self.kit.set_initial_state();
    }
    pub fn state(self) -> State {
        return self.kit.state;
    }
    // TODO: make register_method accept arbitrary functions (can it be done with macro?)
    pub fn register_method(&mut self, key: String, f: Callback) {
        self.kit.registered_methods.insert(key.clone(), f);
        println!("registered method: {}", key);
    }
    pub fn serve(&mut self) {
        println!("server");
        &self.kit.state.set("foo".to_string(), "bar".to_string());
        // TODO: spawn server and listen for msgs
        //server::serve()
    }
}
