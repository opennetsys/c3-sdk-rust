extern crate regex;
extern crate serde;
extern crate serde_json;

use std::str;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, BufWriter, Write};
use std::net::{TcpStream, TcpListener};
use std::env;
use self::regex::Regex;
//use self::serde_derive::{Serialize, Deserialize};

// TODO: improve (sorry i'm a rust noob)

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    store: HashMap<String, String>
}

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
            let f = File::create("/tmp/state.json".to_string()).expect("Unable to create file");
            let mut f = BufWriter::new(f);
            f.write_all(data.as_bytes()).expect("Unable to write to file");
            println!("wrote to tmp state file");
    }
    pub fn get(&mut self, key: &str) -> String {
        let mut data = String::new();
        let mut f = File::open("/tmp/state.json".to_string()).expect("Unable to open file");
        f.read_to_string(&mut data).expect("Unable to read file");

        let store: HashMap<String, String> = serde_json::from_str(&data).unwrap();
        self.store = store;

        let result = &self.store[key];
        return result.to_string();
    }
}

pub type Callback = fn(String, String) -> String;

pub struct Client {
    registered_methods: HashMap<String, Callback>,
    state: State,
}

impl Client {
    pub fn new() -> Client {
        let registered_methods = HashMap::new();
        let store = HashMap::new();
        let state = State::new(store);

        let mut client = Client{
            registered_methods: registered_methods,
            state: state,
        };

        client.set_initial_state();
        return client;
    }
    fn set_initial_state(&mut self) {
        println!("initial state");
        let mut data = String::new();
        let mut f = File::open("/tmp/state.json".to_string()).expect("Unable to open file");
        f.read_to_string(&mut data).expect("Unable to read file");

        let store: HashMap<String, String> = serde_json::from_str(&data).unwrap();
        self.state.store = store;
    }
    fn process_payload(&mut self, payload: String) {
        let json = clean_json(&payload);
        println!("processing; received {}", json);
        let params: [String; 3] = serde_json::from_str(&json).unwrap();
        let method_name = params[0].to_string();
        let mut args: [String; 2] = Default::default();
        args.clone_from_slice(&params[1..]);
        self.invoke(method_name, args);
    }
    fn invoke(&mut self, method_name: String, params: [String; 2]) {
        println!("invoking {}", method_name);
        let f = &self.registered_methods[&method_name];
        f(params[0].to_string(), params[1].to_string());
    }
    pub fn serve(&mut self) {
        let port = match env::var("PORT") {
            Ok(val) => val,
            Err(_) => "3333".to_string(),
        };

        let host = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(host).unwrap();
        println!("Listening on port {}", port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    //thread::spawn(|| {
                        self.handle_client(stream);
                    //});
                },
                Err(e) => {
                    println!("Unable to connect: {}", e);
                },
            }
        }
    }

    fn handle_client(&mut self, stream: TcpStream) {
        self.handle_read(&stream);
        self.handle_write(stream);
    }

    fn handle_read(&mut self, mut stream: &TcpStream) {
        let mut buf = [0u8; 4096];
        match stream.read(&mut buf) {
            Ok(_) => {
                let req_str = String::from_utf8_lossy(&buf);
                println!("request {}", req_str);

                self.process_payload(req_str.to_string());
            },
            Err(e) => println!("Unable to read stream: {}", e),
        }
    }

    fn handle_write(&self, mut stream: TcpStream) {
        let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body></body></html>\r\n";

        match stream.write(response) {
            Ok(_) => println!("response sent"),
            Err(e) => println!("failed to send response: {}", e),
        }
    }
    pub fn refresh_state(&mut self) {
        self.set_initial_state();
    }
    // TODO: make register_method accept arbitrary functions (can it be done with macro?)
    pub fn register_method(&mut self, key: String, f: Callback) {
        self.registered_methods.insert(key.clone(), f);
        println!("registered method: {}", key);
    }
}

fn clean_json(s: &str) -> String {
    let re = Regex::new(r"^.*?(\[|\{)").unwrap();
    let newstr = re.replace(s, "$1");

    let re = Regex::new(r"(\]|\})[^]}].*?$").unwrap();
    let newstr = re.replace(&newstr, "$1");

    return newstr.to_string();
}
