import * as React from "react";
import { IPlayerInfoType, ITurnInfoType, ModeType } from "types/custom";

import { MODE_SINGLE, PLAYER_INFO_TYPE_HUMAN, PLAYER_INFO_HUMANNAME_HUMAN1, MODE_MULTI } from "utils/constants";

import "./index.scss";

interface IPlayerInfoProps {
  playerInfo: IPlayerInfoType[];
  turnInfo: ITurnInfoType;
  mode: ModeType;
}

export class PlayerInfo extends React.Component<IPlayerInfoProps, {}> {
  public render() {
    const { turnInfo } = this.props;

    return (
      <div className="player-turn-info">
        <span>{`Turn ${turnInfo.turn}: ${turnInfo.playerTurn || ""}`}</span>
        <span>
          {this.generateRightSideContent()}
        </span>
      </div>
    );
  }

  private generateRightSideContent = (): string => {
    const { mode, playerInfo } = this.props;

    if (mode === MODE_SINGLE) {
      const humanIndex = playerInfo.findIndex((i) => i.type === PLAYER_INFO_TYPE_HUMAN);
      const computerInfo = playerInfo.filter((_, i) => i !== humanIndex)[0];
      if (humanIndex < 0) {
        return "";
      }
      return (
        `HUMAN: ${playerInfo[humanIndex].player} | COMPUTER: ${computerInfo.player}`
      );
    } else if (mode === MODE_MULTI) {
      if (playerInfo[0].humanName === null || playerInfo[1].humanName === null) {
        return "";
      }
      playerInfo.sort((a, b) => {
        if (a === null || b === null) {
          return 0;
        } else if (a.humanName === PLAYER_INFO_HUMANNAME_HUMAN1) {
          return -1;
        } else {
          return 1;
        }
      });

      return (
        `${playerInfo[0].humanName}: ${playerInfo[0].player} | ${playerInfo[1].humanName}: ${playerInfo[1].player}`
      );
    }

    return "";
  }
}
