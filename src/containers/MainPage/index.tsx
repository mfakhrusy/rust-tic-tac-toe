import * as React from "react";

import { Game } from "components/Game";
import { ModeInfo } from "components/ModeInfo";
import { ModeSelection } from "components/ModeSelection";
import { Navigator } from "components/Navigator";
import { OrderRoll } from "components/OrderRoll";
import { PlayerInfo } from "components/PlayerInfo";
import { PlayerSelection } from "components/PlayerSelection";

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

import {
  IPlayerInfoType,
  ITurnInfoType,
  ModeType,
  ViewStateType,
} from "types/custom";

import "./index.scss";

interface IMainPageState {
  viewState: ViewStateType;
  mode: ModeType;
  playerInfo: IPlayerInfoType[];
  turnInfo: ITurnInfoType;
}

export class MainPage extends React.Component<{}, IMainPageState> {
  constructor(props: {}) {
    super(props);

    this.state = {
      mode: null,
      playerInfo: [
        {
          humanName: null,
          order: PLAYER_INFO_ORDER_FIRST,
          player: PLAYER_INFO_PLAYER_CIRCLE,
          type: PLAYER_INFO_TYPE_COMPUTER,
        },
        {
          humanName: null,
          order: PLAYER_INFO_ORDER_LAST,
          player: PLAYER_INFO_PLAYER_CROSS,
          type: PLAYER_INFO_TYPE_COMPUTER,
        },
      ],
      turnInfo: {
        playerTurn: null,
        turn: 0,
        winner: null,
      },
      viewState: VIEW_STATE_MODE_SELECTION,
    };
  }

  public render() {
    const { viewState, mode, playerInfo, turnInfo } = this.state;
    const showBackButton = mode !== null && viewState !== VIEW_STATE_GAME;
    const showForwardButton = mode !== null && false;

    return (
      <section className="mainpage">
        <header>
          <ModeInfo mode={mode} />
          <PlayerInfo playerInfo={playerInfo} turnInfo={turnInfo} mode={mode} />
        </header>
        <main className="content">
          {this.renderContent(viewState)}
        </main>
        <footer>
          <Navigator
            showBackButton={showBackButton}
            showForwardButton={showForwardButton}
            onClickBack={this.handleClickBack}
            onClickForward={this.handleClickForward}
          />
        </footer>
      </section>
    );
  }

  private handleSetViewState = (viewState: ViewStateType): void => {
    this.setState({ viewState });
  }

  private handleSetMode = (mode: ModeType): void => {
    this.setState({ mode });
  }

  private handleModeSelection = (mode: ModeType): void => {
    this.setState({ mode }, () => {
      this.handleSetViewState(VIEW_STATE_PLAYER_SELECTION);
    });
  }

  private handleSetNextTurn = (): void => {
    const { turnInfo } = this.state;

    let nextPlayer = null;
    if (turnInfo.playerTurn === PLAYER_INFO_PLAYER_CIRCLE) {
      nextPlayer = PLAYER_INFO_PLAYER_CROSS;
    } else {
      nextPlayer = PLAYER_INFO_PLAYER_CIRCLE;
    }
    const newState = Object.assign({}, turnInfo, { playerTurn: nextPlayer, turn: turnInfo.turn + 1 });
    this.setState({ turnInfo: newState });
  }

  private handlePlayerSelection = (playerType: IPlayerInfoType["player"]): void => {
    const { mode, playerInfo } = this.state;

    switch (mode) {
      case MODE_MULTI: {
        const newState = playerInfo
          .map((item) => ({
            ...item,
            type: PLAYER_INFO_TYPE_HUMAN,
          }))
          .map((item) => {
            if (item.player === playerType) {
              return ({
                ...item,
                humanName: PLAYER_INFO_HUMANNAME_HUMAN1,
              });
            } else {
              return ({
                ...item,
                humanName: PLAYER_INFO_HUMANNAME_HUMAN2,
              });
            }
          });
        this.setState({ playerInfo: newState }, () => {
          this.handleSetViewState(VIEW_STATE_ORDER_ROLL);
        });
        break;
      }
      case MODE_SINGLE: {
        const newState = playerInfo.map((item) => {
          if (item.player === playerType) {
            return ({
              ...item,
              type: PLAYER_INFO_TYPE_HUMAN,
            });
          } else {
            return ({
              ...item,
              type: PLAYER_INFO_TYPE_COMPUTER,
            });
          }
        });
        this.setState({ playerInfo: newState }, () => {
          this.handleSetViewState(VIEW_STATE_ORDER_ROLL);
        });
        break;
      }
      default:
        break;
    }
  }

  private handleRoll = (rolled: boolean): void => {
    const { mode, playerInfo, turnInfo } = this.state;

    // tbh I haven't checked whether this will be EXACTLY 50 by 50 percent, lol
    // but you get the idea
    // if it's not 50 by 50, it should be negligible anyway
    if (rolled === false) {
      const order = Math.ceil(Math.random() + .5) === 1 ? PLAYER_INFO_ORDER_FIRST : PLAYER_INFO_ORDER_LAST;
      const otherPlayerOrder = order === PLAYER_INFO_ORDER_FIRST ? PLAYER_INFO_ORDER_LAST : PLAYER_INFO_ORDER_FIRST;

      switch (mode) {
        case MODE_SINGLE: {
          const newState = playerInfo.map((item) => {
            return ({
              ...item,
              order: item.type === PLAYER_INFO_TYPE_HUMAN ? order : otherPlayerOrder,
            });
          });

          this.setState({ playerInfo: newState });
          break;
        }
        case MODE_MULTI: {
          const newState = playerInfo.map((item) => {
            return ({
              ...item,
              order: item.player === PLAYER_INFO_PLAYER_CROSS ? order : otherPlayerOrder,
            });
          });

          this.setState({ playerInfo: newState });
          break;
        }
        default:
          break;
      }
    } else {
      const firstTurn: ITurnInfoType["playerTurn"] = playerInfo
        .reduce((prev: ITurnInfoType["playerTurn"] | null, curr) => (
          curr.order === PLAYER_INFO_ORDER_FIRST ? curr.player : prev
        ), null);
      const newTurnInfoState: ITurnInfoType = {
        ...turnInfo,
        playerTurn: firstTurn as Exclude<ITurnInfoType["playerTurn"], null>,
        turn: 1,
      };

      this.setState({ turnInfo: newTurnInfoState }, () => { this.handleSetViewState(VIEW_STATE_GAME); })
    }
  }

  private handleClickBack = (e: React.MouseEvent): void => {
    e.preventDefault();
    const { viewState, playerInfo } = this.state;

    switch (viewState) {
      case VIEW_STATE_PLAYER_SELECTION:
        this.handleSetViewState(VIEW_STATE_MODE_SELECTION);
        this.handleSetMode(null);
        break;
      case VIEW_STATE_ORDER_ROLL: {
        this.handleSetViewState(VIEW_STATE_PLAYER_SELECTION);
        const newState = playerInfo
          .map((item) => ({
            ...item,
            type: PLAYER_INFO_TYPE_COMPUTER,
          }))
          .map((item) => ({
            ...item,
            humanName: null,
          }));
        this.setState({ playerInfo: newState });
        break;
      }
      default:
        break;
    }
  }

  private handleClickForward = (e: React.MouseEvent): void => {
    e.preventDefault();
  }

  private renderContent = (viewState: ViewStateType): React.ReactElement | null => {
    const { mode, playerInfo, turnInfo } = this.state;

    let content = null;

    switch (viewState) {
      case VIEW_STATE_GAME:
        content = <Game turnInfo={turnInfo} onClickSquare={this.handleSetNextTurn} />;
        break;
      case VIEW_STATE_MODE_SELECTION:
        content = <ModeSelection onSelectMode={this.handleModeSelection} />;
        break;
      case VIEW_STATE_PLAYER_SELECTION:
        content = (
          <PlayerSelection
            mode={mode as Exclude<ModeType, null>}
            onSelectPlayer={this.handlePlayerSelection}
          />
        );
        break;
      case VIEW_STATE_ORDER_ROLL:
        content = <OrderRoll onRoll={this.handleRoll} mode={mode} playerInfo={playerInfo} />;
        break;
      default:
        content = null;
        break;
    }

    return content;
  }
}
