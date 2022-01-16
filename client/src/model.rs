use derive_more::Display;
use lazy_static::lazy_static;
// ------ ------
//     Model
// ------ ------

#[derive(Clone, Display)]
pub enum GameMode {
    #[display(fmt = "Player vs Player (One Computer)")]
    PlayerVsPlayerOneComputer,
    #[display(fmt = "Player vs Player (Two Computer)")]
    PlayerVsPlayerTwoComputer,
    #[display(fmt = "Player vs Computer")]
    PlayerVsComputer,
    #[display(fmt = "Computer vs Computer")]
    ComputerVsComputer,
}

pub enum GameStatus {
    InitGame,
    GameMode,
    NameSelection,
    TurnSelection,
    MainGame,
    GameOver,
}

pub enum GameType {
    SingleComputer,
    MultiComputer,
}

#[derive(Display, Clone, Debug)]
pub enum PlayerSymbol {
    #[display(fmt = "X")]
    X,
    #[display(fmt = "O")]
    O,
}

#[derive(Clone, Debug)]
pub enum PlayerStatus {
    Win,
    Lose,
    Tie,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub name: String,
    pub symbol: PlayerSymbol,
    pub status: Option<PlayerStatus>,
}

pub struct Players {
    pub player_one: Option<Player>,
    pub player_two: Option<Player>,
}

pub type BoardState = [Option<Player>; 9];

lazy_static! {
    pub static ref WIN_CONDITION: [[u8; 3]; 8] = [
        // horizontal index win condition
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        // vertical index win condition
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        // diagonal index win condition
        [0, 4, 8],
        [2, 4, 6]
    ];
}

// `Model` describes our app state.
pub struct Model {
    pub game_type: GameType,
    pub game_status: GameStatus,
    pub game_mode: Option<GameMode>,
    pub game_turn: Option<Player>,
    pub players: Players,
    pub board_state: BoardState,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Clone)]
// `Msg` describes the different events you can modify state with.
pub enum Msg {
    GoToGameModeSelection,
    PickGameMode(GameMode),
    SetPlayerOne(String),
    SetPlayerTwo(String),
    RollPlayerTurn,
    StartGame,
    PressBoardItem(usize),
    GoBack,
}
