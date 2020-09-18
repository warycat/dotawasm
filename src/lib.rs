// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
mod bitsets;
mod dota;

use dota::*;

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    let mut model = Model::new();
    model.game.init_hero();
    model
}

struct Model {
    game: Game,
}

impl Model {
    fn new() -> Self {
        Model { game: Game::new() }
    }
}

enum Msg {
    ToggleHero(usize),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ToggleHero(i) => {
            model.game.toggle_hero(i);
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            C!["sticky top-0 title flex justify-center"],
            format!("{} Perfect Build", model.game.query_count)
        ],
        ol![model
            .game
            .heroes_by_tier()
            .into_iter()
            .map(|(tier, i, hero, alliances)| li![div![
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
                    button![
                        C![
                            "col-span-2 sm:col-span-1",
                            if model.game.hero_locked(i) {
                                "checked"
                            } else {
                                ""
                            }
                        ],
                        div![
                            C!["grid grid-cols-12"],
                            div![C![format!("col-span-1 tier tier-{}", tier)], tier],
                            div![C!["col-span-11 flex justify-left px-4"], &hero.name],
                        ],
                        ev(Ev::Click, move |_| Msg::ToggleHero(i))
                    ],
                    div![
                        C!["alliances col-span-2 place-content-end sm:col-span-1 grid grid-cols-3"],
                        alliances.iter().map(|alliance| button![
                            C!["flex justify-center px-4 py-2 m-2"],
                            &alliance.name
                        ])
                    ]
                ]
            ]])]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
