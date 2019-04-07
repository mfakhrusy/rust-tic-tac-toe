import * as React from "react";

import { SelectionLayout } from "components/SelectionLayout";

import { ModeType } from "types/custom";
import { MODE_SINGLE } from "utils/constants";

interface IOrderRollProps {
  onRoll: () => void;
  mode: ModeType;
}

export class OrderRoll extends React.Component<IOrderRollProps, {}> {
  public render() {
    const { onRoll } = this.props;

    return (
      <SelectionLayout
        contentItemKey="roll"
        onItemClick={onRoll as (itemKey: string) => void}
        onGenerateLabel={this.generateLabel as (itemKey: string) => string}
        title={this.generateTitle()}
      />
    );
  }

  private generateTitle = (): string => {
    const { mode } = this.props;

    if (mode === MODE_SINGLE) {
      return `Roll your turn`;
    } else {
      return `Roll (whoever picked the) X turn`;
    }
  }

  private generateLabel = (): string => {
    return `Roll!`;
  }
}
