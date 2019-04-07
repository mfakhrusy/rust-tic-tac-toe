import * as React from "react";

import { SelectionLayout } from "components/SelectionLayout";

import { MODE_MULTI, MODE_SINGLE } from "utils/constants";

import { ModeType } from "types/custom";

interface IModeSelectionProps {
  onSelectMode: (mode: ModeType) => void;
}

export class ModeSelection extends React.Component<IModeSelectionProps, {}> {
  public render() {
    const { onSelectMode } = this.props;

    return (
      <SelectionLayout
        contentItemKey={[MODE_SINGLE, MODE_MULTI]}
        onItemClick={onSelectMode as (itemKey: string) => void}
        onGenerateLabel={this.generateLabel as (itemKey: string) => string}
        title="Select Mode"
        doubleItem={true}
      />
    );
  }

  private generateLabel = (rawLabel: Exclude<ModeType, null>): string => {
    return `${rawLabel[0].toUpperCase()}${rawLabel.slice(1)}-player Mode`;
  }
}
