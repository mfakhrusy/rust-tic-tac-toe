// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use derive_more::Display;
use seed::{prelude::*, *};
mod init_game;
mod main_game;
mod pick_game_mode;
mod roll_player;

#[derive(Clone, Display)]
pub enum GameMode {
    #[display(fmt = "Player vs Player (One Computer)")]
    PlayerVsPlayerOneComputer,
    #[display(fmt = "Player vs Player (Two Computer)")]
    PlayerVsPlayerTwoComputer,
    #[display(fmt = "Player vs Computer")]
    PlayerVsComputer,
    #[display(fmt = "Computer vs Computer")]
    ComputerVsComputer,
}

enum RollPlayer {
    Init,
    Loading,
    Finished,
}

enum GameStatus {
    InitGame,
    GameMode,
    RollPlayer(RollPlayer),
    MainGame,
}

enum PlayerSymbol {
    X,
    O,
}

enum PlayerStatus {
    Win,
    Lose,
}

struct Player {
    name: String,
    symbol: PlayerSymbol,
    status: Option<PlayerStatus>,
}

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        board_state: [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
        game_status: GameStatus::InitGame,
        game_mode: None,
        player: None,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    game_status: GameStatus,
    game_mode: Option<GameMode>,
    player: Option<[Player; 2]>,
    board_state: [[i32; 3]; 3],
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Clone)]
// `Msg` describes the different events you can modify state with.
pub enum Msg {
    StartGame,
    PickGameMode(GameMode),
    // PickPlayerName,
    StartRollPlayer,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::StartGame => model.game_status = GameStatus::GameMode,
        Msg::PickGameMode(game_mode) => {
            model.game_mode = Some(game_mode);
            model.game_status = GameStatus::RollPlayer(RollPlayer::Init);
        }
        Msg::StartRollPlayer => {
            model.game_status = GameStatus::RollPlayer(RollPlayer::Loading);
        }
    }
}

// ------ ------
//     View
// ------ ------

fn header(model: &Model) -> Node<Msg> {
    div![
        C!["flex w-96 h-8"],
        p![
            span![C!["text-sm font-bold mb-2"], "Game Mode: "],
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
        header(&model),
        div![
            C!["w-96 h-96 bg-gray-200 rounded-sm"],
            match model.game_status {
                GameStatus::InitGame => init_game::view(),
                GameStatus::GameMode => pick_game_mode::view(),
                GameStatus::RollPlayer(RollPlayer::Init) => roll_player::view_init(),
                GameStatus::RollPlayer(RollPlayer::Loading) => roll_player::view_loading(),
                GameStatus::RollPlayer(RollPlayer::Finished) => roll_player::view_finished(),
                GameStatus::MainGame => main_game::view(),
            }
        ]
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
