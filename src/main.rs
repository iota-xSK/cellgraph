mod app;
mod automaton;
mod cellang;
mod graph;
mod note;
mod saved_state;
mod vec2;
use std::collections::HashMap;

use app::App;
use automaton::Automaton;
use graph::Graph;

#[macroquad::main("cell sound")]
async fn main() {
    let graph = Graph::new();
    let rule_map = HashMap::new();
    let mut app = App::new(Automaton::new(rule_map, graph));
    loop {
        app.mainloop().await;
    }
}
