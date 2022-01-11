use crate::{model::BoardState, Msg};
use seed::{prelude::*, *};

pub fn view(board_state: &BoardState) -> Node<Msg> {
    div![board_state
        .iter()
        .map(|row| {
            div![
                C!["flex flex-row justify-around"],
                row.iter()
                    .map(|cell| {
                        div![
                            C!["w-12 h-12 flex items-center justify-center bg-gray-100"],
                            match cell {
                                None => "".to_string(),
                                Some(player) => player.symbol.to_string(),
                            }
                        ]
                    })
                    .collect::<Vec<Node<Msg>>>()
            ]
        })
        .collect::<Vec<Node<Msg>>>()]
}
