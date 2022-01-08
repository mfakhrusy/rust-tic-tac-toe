use crate::{
    model::{GameMode, GameType},
    Msg,
};
use seed::{prelude::*, *};

static NAMES: &'static [&str] = &["bob", "alice", "habibi", "rob", "jay", "ahmed"];

enum PlayerType {
    First,
    Second,
}

fn view_human_player(player_type: PlayerType) -> Node<Msg> {
    let options = NAMES
        .iter()
        .map(|name| option![attrs! {At::Value => name}, name]);

    div![
        C!["py-4 flex flex-col items-center justify-center"],
        div![
            C!["text-7xl mb-4 bg-gray-100 w-36 h-36 flex items-center justify-center"],
            "?"
        ],
        select![
            attrs! {At::Name => "name"},
            options,
            input_ev(
                Ev::Input,
                match player_type {
                    PlayerType::First => Msg::SetPlayerOne,
                    PlayerType::Second => Msg::SetPlayerTwo,
                }
            ),
        ]
    ]
}

pub fn view(game_mode: &GameMode, game_type: &GameType) -> Node<Msg> {
    match game_type {
        &GameType::SingleComputer => {
            div![
                C!["flex h-full w-full"],
                div![
                    C!["flex w-2/4 h-full justify-center"],
                    match game_mode {
                        GameMode::ComputerVsComputer => div![], // todo
                        GameMode::PlayerVsComputer => view_human_player(PlayerType::First),
                        GameMode::PlayerVsPlayerOneComputer => view_human_player(PlayerType::First),
                        _ => empty![],
                    },
                ],
                div![
                    C!["flex w-2/4 h-full justify-center"],
                    view_human_player(PlayerType::Second)
                ],
            ]
        }
        &GameType::MultiComputer => {
            div![]
        }
    }
}
