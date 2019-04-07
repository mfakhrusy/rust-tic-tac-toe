import * as React from "react";

import { SelectionLayout } from "components/SelectionLayout";

import { IPlayerInfoType, ModeType} from "types/custom";

import { MODE_SINGLE, PLAYER_INFO_TYPE_HUMAN, PLAYER_INFO_HUMANNAME_HUMAN1 } from "utils/constants";

interface IOrderRollProps {
  onRoll: (rolled: boolean) => void;
  mode: ModeType;
  playerInfo: IPlayerInfoType[];
}

interface IOrderRollState {
  orderRolled: boolean;
}

export class OrderRoll extends React.Component<IOrderRollProps, IOrderRollState> {
  constructor(props: IOrderRollProps) {
    super(props);

    this.state = {
      orderRolled: false,
    };
  }

  public componentWillUnmount() {
    this.setState({ orderRolled: false });
  }

  public render() {
    return (
      <SelectionLayout
        contentItemKey="roll"
        onItemClick={this.handleRoll as (itemKey: string) => void}
        onGenerateLabel={this.generateLabel as (itemKey: string) => string}
        title={this.generateTitle()}
      />
    );
  }

  private handleRoll = (): void => {
    const { onRoll } = this.props;
    const { orderRolled } = this.state;

    if (orderRolled === false) {
      // param false in onRoll means it haven't been rolled so it count the roll order
      // to change the app state
      this.setState({ orderRolled: true }, () => { onRoll(false); });
    } else {
      // if it's rolled, send the true param, and view will change to game state
      onRoll(true);
    }
  }

  private generateTitle = (): string => {
    const { mode, playerInfo } = this.props;
    const { orderRolled } = this.state;

    if (orderRolled === false) {
      if (mode === MODE_SINGLE) {
        return `Roll your turn`;
      } else {
        return `Roll (whoever picked the) X turn`;
      }
    } else {
      if (mode === MODE_SINGLE) {
        return `HUMAN -> ${playerInfo
          .reduce((prev, curr) => (curr.type === PLAYER_INFO_TYPE_HUMAN ? curr.order : prev), "")}`;
      } else {
        playerInfo.sort((a, _) => a.humanName === PLAYER_INFO_HUMANNAME_HUMAN1 ? -1 : 1);
        return (
          `${playerInfo[0].humanName} -> ${playerInfo[0].order} | ${playerInfo[1].humanName} -> ${playerInfo[1].order}`
        );
      }
    }
  }

  private generateLabel = (): string => {
    const { orderRolled } = this.state;

    if (orderRolled === false) {
      return `Roll!`;
    } else {
      return "READY TO PLAY!";
    }
  }
}
