use derive_more::Display;
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
    MainGame,
}

pub enum GameType {
    SingleComputer,
    MultiComputer,
}

pub enum PlayerSymbol {
    X,
    O,
}

pub enum PlayerStatus {
    Win,
    Lose,
}

pub struct Player {
    pub name: String,
    pub symbol: PlayerSymbol,
    pub status: Option<PlayerStatus>,
}

pub struct Players {
    pub player_one: Option<Player>,
    pub player_two: Option<Player>,
}

// `Model` describes our app state.
pub struct Model {
    pub game_type: GameType,
    pub game_status: GameStatus,
    pub game_mode: Option<GameMode>,
    pub players: Players,
    pub board_state: [[i32; 3]; 3],
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Clone)]
// `Msg` describes the different events you can modify state with.
pub enum Msg {
    StartGame,
    PickGameMode(GameMode),
    SetPlayerOne(String),
    SetPlayerTwo(String),
    GoBack,
}