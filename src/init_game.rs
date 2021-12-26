use crate::Msg;
use seed::{prelude::*, *};

pub fn view() -> Node<Msg> {
    div![
        C!["w-full h-full flex flex-col items-center justify-center"],
        h1![C!["text-4xl font-bold mb-4"], "Tic Tac Toe"],
        button![
            C!["bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"],
            ev(Ev::Click, |_| Msg::StartGame),
            "Start Game"
        ]
    ]
}
