use std::collections::HashMap;
use std::str;

pub struct State {
    store: HashMap<String, String>
}

impl State {
    pub fn new(mut store: HashMap<String, String>) -> State {
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

            // TODO:
            // open(statefile)
            // convert state to json
            // write to file
            // close file
    }
    pub fn get(&self, key: &str) -> String {
        let result = &self.store[key];
        return result.to_string();
    }
}

pub type Callback = fn(String, String) -> String;

pub struct Kit {
    statefile: String,
    registered_methods: HashMap<String, Callback>,
}

impl Kit {
    pub fn new() -> Kit{
        // TODO
        // emitter = new emitter
        let mut registered_methods = HashMap::new();
        return Kit{
            statefile: "/tmp/state.json".to_string(),
            registered_methods: registered_methods,
        };
    }
    pub fn set_initial_state(&self) {
        println!("initial state");
        // TODO: set initial state
    }
    pub fn listen(&self) {
        println!("listening");
        // TODO:
        //emitter.on('data', fn {
        //self.process_payload(data)
        //})
    }
    pub fn process_payload(&self, payload: String) {
        println!("processing");
        // TODO:
        //args = jsondecode(payl)
        //self.invoke(args[0], args[1:])
    }
    pub fn invoke(&self, method_name: String, params: String) {
        println!("invokeing");
        // TODO:
        //self.registered_methods[method_name](...params)
    }
}

pub struct Client {
    kit: Kit,
    state: State,
}

impl Client {
    pub fn new() -> Client {
        let kit = Kit::new();
        kit.set_initial_state();
        kit.listen();

        let mut store = HashMap::new();
        let mut state = State::new(store);

        return Client{
            kit: kit,
            state: state,
        };
    }
    pub fn refresh_state(&self) {
        self.kit.set_initial_state();
    }
    pub fn state(self) -> State {
        return self.state;
    }
    // TODO: make register_method accept arbitrary functions (can it be done with macro?)
    pub fn register_method(&mut self, key: String, f: Callback) {
        self.kit.registered_methods.insert(key.clone(), f);
        println!("registered method: {}", key);
    }
    pub fn serve(&self) {
        println!("server")
        // TODO: spawn server and listen for msgs
        //server::serve()
    }
}
