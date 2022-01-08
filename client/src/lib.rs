// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

mod init_game;
mod main_game;
mod model;
mod name_selection;
mod pick_game_mode;

use model::{
    GameMode, GameStatus, GameType, Model, Msg, Player, PlayerStatus, PlayerSymbol, Players,
};
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        board_state: [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
        game_status: GameStatus::InitGame,
        game_mode: None,
        players: Players {
            player_one: None,
            player_two: None,
        },
        game_type: GameType::SingleComputer,
    }
}

// ------ ------
//    Update
// ------ ------

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::StartGame => model.game_status = GameStatus::GameMode,
        Msg::PickGameMode(game_mode) => {
            model.game_mode = Some(game_mode);
            model.game_status = GameStatus::NameSelection;
            match model.game_mode {
                Some(GameMode::PlayerVsPlayerTwoComputer) => {
                    model.game_type = GameType::MultiComputer;
                }
                _ => {
                    model.game_type = GameType::SingleComputer;
                }
            }
        }
        Msg::GoBack => match model.game_status {
            GameStatus::MainGame => {
                model.game_status = GameStatus::NameSelection;
            }
            GameStatus::GameMode => {
                model.game_status = GameStatus::InitGame;
                model.game_mode = None;
            }
            GameStatus::NameSelection => {
                model.game_status = GameStatus::GameMode;
                model.game_mode = None;
            }
            _ => {}
        },
        Msg::SetPlayerOne(name) => {
            model.players.player_one = Some(Player {
                name,
                symbol: PlayerSymbol::X,
                status: None,
            });
            // model.game_status = GameStatus::RollPlayer(RollPlayer::Init);
        }
        Msg::SetPlayerTwo(name) => {
            model.players.player_two = Some(Player {
                name,
                symbol: PlayerSymbol::O,
                status: None,
            });
            // model.game_status = GameStatus::RollPlayer(RollPlayer::Init);
        }
    }
}

// ------ ------
//     View
// ------ ------

fn header(model: &Model) -> Node<Msg> {
    div![
        C!["flex h-20 flex-col"],
        p![
            C![match model.game_status {
                GameStatus::InitGame => "text-sm font-bold mb-2 invisible",
                _ => "text-sm font-bold mb-2 flex ",
            }],
            span![
                C!["w-2/4"],
                span!["Player one: "],
                span![match &model.players.player_one {
                    Some(player) => format!("{}", player.name),
                    None => "".to_string(),
                }]
            ],
            span![
                C!["w-2/4"],
                span!["Player two: "],
                span![match &model.players.player_two {
                    Some(player) => format!("{}", player.name),
                    None => "".to_string(),
                }]
            ]
        ],
        p![
            span![
                C![match model.game_status {
                    GameStatus::InitGame => "text-sm font-bold mb-2 invisible",
                    _ => "text-sm font-bold mb-2",
                }],
                "Game Mode: "
            ],
            span![
                C!["text-sm font-bold mb-2"],
                match model.game_mode {
                    Some(GameMode::PlayerVsPlayerOneComputer) =>
                        GameMode::PlayerVsPlayerOneComputer.to_string(),
                    Some(GameMode::PlayerVsPlayerTwoComputer) =>
                        GameMode::PlayerVsPlayerTwoComputer.to_string(),
                    Some(GameMode::PlayerVsComputer) => GameMode::PlayerVsComputer.to_string(),
                    Some(GameMode::ComputerVsComputer) => GameMode::ComputerVsComputer.to_string(),
                    None => "".to_string(),
                }
            ]
        ],
    ]
}

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["w-screen h-screen flex flex-col items-center justify-center"],
        div![
            C!["w-96"],
            header(&model),
            div![
                C!["h-96 bg-gray-200 rounded-sm"],
                match model.game_status {
                    GameStatus::InitGame => init_game::view(),
                    GameStatus::GameMode => pick_game_mode::view(),
                    GameStatus::NameSelection =>
                        name_selection::view(
                            &model.game_mode.as_ref().unwrap_or(&GameMode::PlayerVsPlayerOneComputer),
                            &model.game_type
                        ),
                    GameStatus::MainGame => main_game::view(),
                }
            ],
            button![
            C![
                match model.game_status {
                    GameStatus::InitGame => "invisible font-bold py-2 mt-4",
                    _ => "bg-gray-500 hover:bg-gray-600 text-white font-bold py-2 px-4 rounded w-20 mt-4",
                }
                ],
            "Back",
            ev(Ev::Click, |_| Msg::GoBack),
        ],
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
