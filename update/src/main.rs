extern crate data;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate web_view;
use web_view::*;

const INDEX: &'static str = include_str!("assets/index.html");
const JS: &'static str = include_str!("assets/app.js");
const CSS: &'static str = include_str!("assets/main.css");
#[derive(Serialize, Debug)]
struct State {
    pub items: Vec<String>,
}
fn main() {
    let size = (800, 800);
    run(
        "Mashton.party Updater",
        Content::Html(INDEX),
        Some(size),
        true,
        true,
        true,
        init,
        event_loop,
        State { items: vec![]}
    );
}

fn init(wv: MyUnique<WebView<State>>) {
    println!("init");
    wv.dispatch(|wv: &mut WebView<State>, s: &mut State| {
        println!("dispatch: {:?}", s);
        wv.inject_css(CSS);
        wv.eval(JS);
    });
}

fn event_loop(wv: &mut WebView<State>, arg: &str, state: &mut State) {
    println!("event_loop {:?}", state);
    state.items.push(arg.to_string());
     let (event_name, state_str) = match serde_json::to_string(&state) {
         Ok(s) => ("state-change", s),
         Err(e) => ("error", format!("error serializing state: {:?}", e))
     };
     wv.eval(&format!("window.dispatchEvent(new CustomEvent('{}', {{detail: {}}}));", event_name, state_str));
}