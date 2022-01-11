use crate::{components, model::BoardState, Msg};
use seed::{prelude::*, *};

pub fn view(board_state: &BoardState) -> Node<Msg> {
    div![
        C!["w-full h-full grid grid-cols-3 grid-rows-3 gap-1"],
        board_state.iter().enumerate().map(|(i, player)| {
            div![
                C!["w-full h-full bg-gray-100 rounded-sm"],
                match player {
                    None => button![
                        C!["w-full h-full"],
                        ev(Ev::Click, move |_| Msg::PressBoardItem(i))
                    ],
                    Some(curr_player) => div![
                        C!["mb-4 bg-gray-100 w-28 h-28 flex items-center justify-center"],
                        span![C!["text-7xl font-mono"], curr_player.symbol.to_string()],
                        span![
                            C!["text-sm mt-12 font-mono"],
                            curr_player
                                .name
                                .chars()
                                .nth(0)
                                .unwrap()
                                .to_uppercase()
                                .to_string()
                        ]
                    ],
                }
            ]
        })
    ]
}
