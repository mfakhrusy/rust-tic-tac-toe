import * as React from "react";

import { Board } from "components/Board";
import { Countdown } from "components/Countdown";

import { ITurnInfoType } from "types/custom";

interface IGameProps {
  turnInfo: ITurnInfoType;
  onClickSquare: () => void;
}

export interface IGameState {
  showBoard: boolean;
}

export class Game extends React.Component<IGameProps, IGameState> {
  constructor(props: IGameProps) {
    super(props);

    this.state = {
      showBoard: false,
    };
  }

  public render() {
    const { turnInfo, onClickSquare } = this.props;
    const { showBoard } = this.state;

    if (showBoard) {
      return <Board turnInfo={turnInfo} onClickSquare={onClickSquare} />;
    }
    return (
      <Countdown timeLimit={5} onFinishCountdown={this.showBoard} />
    );
  }

  private setShowBoard = (value: IGameState["showBoard"]): void => {
    this.setState({ showBoard: value });
  }

  private showBoard = () => {
    this.setShowBoard(true);
  }
}
