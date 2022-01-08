use seed::{prelude::*, *};

use crate::model::{Msg, Player};

pub fn view(player: &Option<Player>) -> Node<Msg> {
    match player {
        None => div![
            C!["text-7xl mb-4 bg-gray-100 w-28 h-28 flex items-center justify-center font-mono"],
            "?"
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
}
