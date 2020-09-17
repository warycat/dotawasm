// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]
#![feature(const_fn)]

use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Result, Value};
mod dota;

use dota::*;

// ------ ------
//     Init
// ------ ------
use seed::{prelude::*, *};

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::new()
}

// `Model` describes our app state.
#[derive(Default)]
struct Model {
    data: Data,
}

impl Model {
    fn new() -> Self {
        Model {
            data: serde_json::from_str(include_str!("dota.json")).unwrap(),
        }
    }
}

// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => {}
    }
}

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![ul![
        if let Table::Alliances { rows } = &model.data.objects[0] {
            rows.to_vec()
        } else {
            vec![]
        }
        .iter()
        .map(|alliance| li![format!("{}", alliance.id)]),
    ],]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
