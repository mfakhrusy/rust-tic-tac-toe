use crate::{GameMode, Msg};
use seed::{prelude::*, *};

fn view_pick_game_mode_item(label: &str, game_mode: GameMode) -> Node<Msg> {
    button![
        C!["flex justify-center items-center hover:bg-gray-300"],
        p![C!["text-center"], label],
        ev(Ev::Click, |_| Msg::PickGameMode(game_mode)),
    ]
}

pub fn view() -> Node<Msg> {
    div![
        C!["w-full h-full grid grid-cols-2 grid-rows-2 w-full h-full"],
        view_pick_game_mode_item(
            &GameMode::PlayerVsPlayerOneComputer.to_string(),
            GameMode::PlayerVsPlayerOneComputer
        ),
        view_pick_game_mode_item(
            &GameMode::PlayerVsPlayerTwoComputer.to_string(),
            GameMode::PlayerVsPlayerTwoComputer
        ),
        view_pick_game_mode_item(
            &GameMode::PlayerVsComputer.to_string(),
            GameMode::PlayerVsComputer
        ),
        view_pick_game_mode_item(
            &GameMode::ComputerVsComputer.to_string(),
            GameMode::ComputerVsComputer
        ),
    ]
}
