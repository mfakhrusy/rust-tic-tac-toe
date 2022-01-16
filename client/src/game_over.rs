use crate::{
    model::{PlayerStatus, Players},
    Msg,
};
use seed::{prelude::*, *};

pub fn view(players: &Players) -> Node<Msg> {
    div![
        C!["w-full h-full flex flex-col items-center justify-center"],
        match &players.player_one {
            None => empty![],
            Some(player) => match &player.status {
                None => empty![],
                Some(status) => p![match status {
                    PlayerStatus::Win => "Player One Wins!",
                    PlayerStatus::Lose => "Player Two Wins!",
                    PlayerStatus::Tie => "Tie!",
                }],
            },
        }
    ]
}
