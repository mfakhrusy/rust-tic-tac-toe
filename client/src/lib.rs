// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

mod components;
mod init_game;
mod main_game;
mod model;
mod name_selection;
mod pick_game_mode;
mod roll_player_turn;

use model::{
    GameMode, GameStatus, GameType, Model, Msg, Player, PlayerStatus, PlayerSymbol, Players,
};
use rand::Rng;
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        board_state: [[None, None, None], [None, None, None], [None, None, None]],
        game_status: GameStatus::InitGame,
        game_mode: None,
        players: Players {
            player_one: None,
            player_two: None,
        },
        game_type: GameType::SingleComputer,
        game_turn: None,
    }
}

// ------ ------
//    Update
// ------ ------

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::GoToGameModeSelection => model.game_status = GameStatus::GameMode,
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
                model.game_status = GameStatus::TurnSelection;
            }
            GameStatus::GameMode => {
                model.game_status = GameStatus::InitGame;
                model.game_mode = None;
            }
            GameStatus::NameSelection => {
                model.game_status = GameStatus::GameMode;
                model.players = Players {
                    player_one: None,
                    player_two: None,
                };
                model.game_mode = None;
            }
            GameStatus::TurnSelection => {
                model.game_status = GameStatus::NameSelection;
                model.game_turn = None;
            }
            _ => {}
        },
        Msg::SetPlayerOne(name) => {
            model.players.player_one = Some(Player {
                name,
                symbol: PlayerSymbol::X,
                status: None,
            });
        }
        Msg::SetPlayerTwo(name) => {
            model.players.player_two = Some(Player {
                name,
                symbol: PlayerSymbol::O,
                status: None,
            });
        }
        Msg::RollPlayerTurn => {
            // roll first player
            let num = rand::thread_rng().gen_range(0..=1);
            model.game_turn = match num {
                0 => model.players.player_one.clone(),
                _ => model.players.player_two.clone(),
            };
            model.game_status = GameStatus::TurnSelection;
        }
        Msg::StartGame => {
            model.game_status = GameStatus::MainGame;
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
        p![
            C![match model.game_status {
                GameStatus::InitGame => "text-sm font-bold mb-2 invisible",
                _ => "text-sm font-bold mb-2 flex ",
            }],
            span![
                C!["w-2/4"],
                span!["Player one (X): "],
                span![match &model.players.player_one {
                    Some(player) => format!("{}", player.name),
                    None => "".to_string(),
                }]
            ],
            span![
                C!["w-2/4"],
                span!["Player two (O): "],
                span![match &model.players.player_two {
                    Some(player) => format!("{}", player.name),
                    None => "".to_string(),
                }]
            ]
        ],
        p![
            C![match model.game_status {
                GameStatus::InitGame => "text-sm font-bold mb-2 invisible",
                GameStatus::GameMode => "text-sm font-bold mb-2 invisible",
                GameStatus::NameSelection => "text-sm font-bold mb-2 invisible",
                _ => "text-sm font-bold mb-2 flex ",
            }],
            span![
                C!["w-2/4"],
                span!["Current Turn: "],
                span![match &model.game_turn {
                    Some(player) => format!("{}", player.name),
                    None => "".to_string(),
                }],
                span![match &model.game_turn {
                    Some(player) => format!(" ({})", player.symbol),
                    None => "".to_string(),
                }]
            ],
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
                            &model.game_type,
                            &model.players,
                        ),
                    GameStatus::TurnSelection => roll_player_turn::view(&model.game_turn, &model.players),
                    GameStatus::MainGame => main_game::view(&model.board_state),
                }
            ],
            div![
                C!["flex justify-between"],
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
                button![
                    C![
                        match model.game_status {
                            GameStatus::NameSelection => {
                            match (model.players.player_one.as_ref(), model.players.player_two.as_ref()) {
                                (Some(_), Some(_)) => "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded w-40 mt-4",
                                _ => "bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded w-40 mt-4 cursor-not-allowed"
                            }
                        },
                            _ => "hidden font-bold py-2 mt-4",
                        }
                    ],
                    "Roll Player Turn",
                    attrs! {
                        At::Disabled => match (model.players.player_one.as_ref(), model.players.player_two.as_ref()) {
                            (Some(_), Some(_)) => false.as_at_value(),
                            _ => true.as_at_value(),
                        }
                    },
                    ev(Ev::Click, |_| Msg::RollPlayerTurn),
                ],
                button![
                    C![
                        match model.game_status {
                            GameStatus::TurnSelection => "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded w-40 mt-4",
                            _ => "hidden font-bold py-2 mt-4",
                        }
                    ],
                    "Start Game",
                    ev(Ev::Click, |_| Msg::StartGame),
                ],
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
