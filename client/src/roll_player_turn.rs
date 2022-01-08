use crate::{
    components,
    model::{Player, PlayerSymbol, Players},
    Msg,
};
use seed::{prelude::*, *};

pub fn view(first_player_turn: &Option<Player>, players: &Players) -> Node<Msg> {
    let first_player_symbol = match first_player_turn {
        None => None,
        Some(player) => Some(&player.symbol),
    };

    let second_player_turn = match first_player_symbol {
        None => None,
        Some(symbol) => match symbol {
            PlayerSymbol::X => players.player_two.clone(),
            PlayerSymbol::O => players.player_one.clone(),
        },
    };

    div![
        C!["flex items-center justify-between h-full"],
        div![
            C!["flex w-2/4 h-full justify-center flex-col items-center"],
            components::player_symbol::view(first_player_turn),
            p![
                style! {
                    St::MarginBottom => "-3px",
                },
                "First Turn",
            ]
        ],
        div![
            C!["flex w-2/4 h-full justify-center flex-col items-center"],
            components::player_symbol::view(&second_player_turn),
            p![
                style! {
                    St::MarginBottom => "-3px",
                },
                "Second Turn",
            ]
        ],
    ]
}
