// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::prelude::web_sys::Event;
use seed::{prelude::*, *};
mod bitsets;
mod dota;

use dota::*;

// ------ ------
//     Init
// ------ ------
use seed::{prelude::*, *};

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    let mut model = Model::new();
    model.game.init_hero();
    model
}

// `Model` describes our app state.
struct Model {
    game: Game,
}

impl Model {
    fn new() -> Self {
        Model { game: Game::new() }
    }
}

// `Msg` describes the different events you can modify state with.
enum Msg {
    ToggleHero(usize),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ToggleHero(i) => {
            model.game.toggle_hero(i);
        }
    }
}

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![div![ol![model.game.heroes.iter().enumerate().map(
        |(i, hero)| li![
            div![
                C![
                    "heroes",
                    if model.game.hero_filtered(i) {
                        ""
                    } else {
                        "removed"
                    }
                ],
                div![
                    C!["grid grid-cols-2"],
                    div![
                        C![if model.game.hero_locked(i) {
                            "checked"
                        } else {
                            ""
                        },],
                        &hero.name
                    ],
                    div![
                        C!["alliances grid grid-cols-3"],
                        model
                            .game
                            .hero_alliances(i)
                            .iter()
                            .map(|alliance| div![&alliance.name])
                    ],
                ]
            ],
            ev(Ev::Click, move |_| Msg::ToggleHero(i))
        ]
    )]],]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
