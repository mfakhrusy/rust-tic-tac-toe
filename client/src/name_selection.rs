use crate::{
    components,
    model::{GameMode, GameType, Player, Players},
    Msg,
};
use seed::{prelude::*, *};

static NAMES: &'static [&str] = &["Red", "Blue", "Cyan", "Yellow", "Orange", "Green"];

enum PlayerType {
    First,
    Second,
}

fn view_human_player(player_type: PlayerType, player: &Option<Player>) -> Node<Msg> {
    let options = NAMES
        .iter()
        .map(|name| option![attrs! {At::Value => name}, name]);

    div![
        C!["py-4 flex flex-col items-center justify-center"],
        components::player_symbol::view(player),
        select![
            attrs! {At::Name => "name"},
            option![
                attrs! {At::Value => "", At::Selected => true, At::Disabled => true},
                "Select Name"
            ],
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

pub fn view(game_mode: &GameMode, game_type: &GameType, players: &Players) -> Node<Msg> {
    match game_type {
        &GameType::SingleComputer => {
            div![
                C!["flex h-full w-full"],
                div![
                    C!["flex w-2/4 h-full justify-center"],
                    match game_mode {
                        GameMode::ComputerVsComputer => div![], // todo
                        GameMode::PlayerVsComputer =>
                            view_human_player(PlayerType::First, &players.player_one),
                        GameMode::PlayerVsPlayerOneComputer =>
                            view_human_player(PlayerType::First, &players.player_one),
                        _ => empty![],
                    },
                ],
                div![
                    C!["flex w-2/4 h-full justify-center"],
                    view_human_player(PlayerType::Second, &players.player_two),
                ],
            ]
        }
        &GameType::MultiComputer => {
            div![]
        }
    }
}
