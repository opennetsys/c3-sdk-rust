mod client;

pub struct App {
    client: client::Client,
    set_item: client::Callback,
}

impl App {
    pub fn new() -> App {
        let mut cl = client::Client::new();

        let mut f: client::Callback = |key: String, val: String| -> String {
            // TODO: figure this out
            //cl.state().set("foo".to_string(), "f".to_string());
           return "".to_string();
        };

        return App{
            client: cl,
            set_item: f,
        }
    }
    pub fn get_item(&self, key: String, _: String) -> String {
        // TODO: figure this out
        //return self.client.state().get(&key);
        return "".to_string();
    }
}

fn main() {
    let mut app = App::new();
    app.client.register_method("setItem".to_string(), app.set_item);
    // TODO:
    //app.client.register_method("getItem".to_string(), app.get_item)
    app.client.serve()
}
