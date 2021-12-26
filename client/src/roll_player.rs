use crate::Msg;
use seed::{prelude::*, *};

pub fn view_init() -> Node<Msg> {
    div![
        C!["w-full h-full flex flex-col items-center justify-center"],
        h1![
            C!["text-4xl font-bold mb-4 text-center"],
            "Press to Roll the Player"
        ],
        button![
            C!["bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"],
            ev(Ev::Click, |_| Msg::StartRollPlayer),
            "Roll Player"
        ]
    ]
}

pub fn view_loading() -> Node<Msg> {
    div![
        C!["w-full h-full flex flex-col items-center justify-center"],
        div![
            C!["flex"],
            div![
                C!["flex flex-col mr-4 items-center justify-center"],
                p!["Player 1"],
                p![C!["randomize-player"], ""]
            ],
            div![
                C!["flex flex-col items-center justify-center"],
                p!["Player 2"],
                p![C!["randomize-player-inverse"], ""]
            ],
        ],
        button![
            C!["bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mt-16"],
            // ev(Ev::Click, |_| None),
            "Stop!"
        ]
    ]
}

pub fn view_finished() -> Node<Msg> {
    div![
        C!["w-full h-full flex flex-col items-center justify-center"],
        h1![
            C!["text-4xl font-bold mb-4 text-center"],
            "Press to Roll the Player"
        ],
        button![
            C!["bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"],
            // ev(Ev::Click, |_| None),
            "Roll Player"
        ]
    ]
}
