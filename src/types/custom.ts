import {
  MODE_MULTI,
  MODE_SINGLE,
  PLAYER_INFO_HUMANNAME_HUMAN1,
  PLAYER_INFO_HUMANNAME_HUMAN2,
  PLAYER_INFO_ORDER_FIRST,
  PLAYER_INFO_ORDER_LAST,
  PLAYER_INFO_PLAYER_CIRCLE,
  PLAYER_INFO_PLAYER_CROSS,
  PLAYER_INFO_TYPE_COMPUTER,
  PLAYER_INFO_TYPE_HUMAN,
  VIEW_STATE_GAME,
  VIEW_STATE_MODE_SELECTION,
  VIEW_STATE_ORDER_ROLL,
  VIEW_STATE_PLAYER_SELECTION,
} from "utils/constants";

export type ModeType = typeof MODE_SINGLE | typeof MODE_MULTI | null;
export type ViewStateType = typeof VIEW_STATE_GAME
  | typeof VIEW_STATE_MODE_SELECTION
  | typeof VIEW_STATE_PLAYER_SELECTION
  | typeof VIEW_STATE_ORDER_ROLL;

export interface IPlayerInfoType {
  player: typeof PLAYER_INFO_PLAYER_CIRCLE | typeof PLAYER_INFO_PLAYER_CROSS;
  order: typeof PLAYER_INFO_ORDER_FIRST | typeof PLAYER_INFO_ORDER_LAST;
  type: typeof PLAYER_INFO_TYPE_HUMAN | typeof PLAYER_INFO_TYPE_COMPUTER;
  humanName: typeof PLAYER_INFO_HUMANNAME_HUMAN1 | typeof PLAYER_INFO_HUMANNAME_HUMAN2 | null;
}

export interface ITurnInfoType {
  playerTurn: typeof PLAYER_INFO_PLAYER_CIRCLE | typeof PLAYER_INFO_PLAYER_CROSS | null;
  turn: number;
  winner: typeof PLAYER_INFO_PLAYER_CIRCLE | typeof PLAYER_INFO_PLAYER_CROSS | null;
}

export type BoardSquareContentType = typeof PLAYER_INFO_PLAYER_CIRCLE | typeof PLAYER_INFO_PLAYER_CROSS | "";

export type BoardGameStateType = [
  [BoardSquareContentType, BoardSquareContentType, BoardSquareContentType],
  [BoardSquareContentType, BoardSquareContentType, BoardSquareContentType],
  [BoardSquareContentType, BoardSquareContentType, BoardSquareContentType],
];
