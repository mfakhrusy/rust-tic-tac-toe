import * as React from "react";

import { Square } from "components/Square";

import { BoardGameStateType, ITurnInfoType } from "types/custom";

import "./index.scss";

interface IBoardProps {
  turnInfo: ITurnInfoType;
  onClickSquare: () => void;
}

interface IBoardState {
  gameState: BoardGameStateType;
}

export class Board extends React.Component<IBoardProps, IBoardState> {
  constructor(props: IBoardProps) {
    super(props);

    this.state = {
      gameState: [
        ["", "", ""],
        ["", "", ""],
        ["", "", ""],
      ],
    };
  }

  public render() {
    const { gameState } = this.state;

    return (
      <div className="board">
        <div className="board-row board-top">
          <Square content={gameState[0][0]} x={0} y={0} onClick={this.setBoardSquare} />
          <Square content={gameState[0][1]} x={0} y={1} onClick={this.setBoardSquare} />
          <Square content={gameState[0][2]} x={0} y={2} onClick={this.setBoardSquare} />
        </div>
        <div className="board-row board-middle">
          <Square content={gameState[1][0]} x={1} y={0} onClick={this.setBoardSquare} />
          <Square content={gameState[1][1]} x={1} y={1} onClick={this.setBoardSquare} />
          <Square content={gameState[1][2]} x={1} y={2} onClick={this.setBoardSquare} />
        </div>
        <div className="board-row board-bottom">
          <Square content={gameState[2][0]} x={2} y={0} onClick={this.setBoardSquare} />
          <Square content={gameState[2][1]} x={2} y={1} onClick={this.setBoardSquare} />
          <Square content={gameState[2][2]} x={2} y={2} onClick={this.setBoardSquare} />
        </div>
      </div>
    );
  }

  private setBoardSquare = (x: number, y: number): void => {
    const { onClickSquare, turnInfo } = this.props;
    const { gameState } = this.state;

    const newGameState = gameState.map((row, xIndex) => {
      if (xIndex === x) {
        return row.map((cell, yIndex) => {
          if (yIndex === y) {
            return turnInfo.playerTurn;
          }
          return cell;
        });
      }
      return row;
    });

    this.setState({ gameState: newGameState as any });

    onClickSquare();
  }
}
