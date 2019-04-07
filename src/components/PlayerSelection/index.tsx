import * as React from "react";

import { IconProp } from "@fortawesome/fontawesome-svg-core";
import { SelectionLayout } from "components/SelectionLayout";

import {
  IPlayerInfoType,
  ModeType,
} from "types/custom";

import { MODE_SINGLE, PLAYER_INFO_PLAYER_CIRCLE, PLAYER_INFO_PLAYER_CROSS } from "utils/constants";
import { generateIconLabel } from "utils/helpers";

interface IPlayerSelectionProps {
  onSelectPlayer: (playerType: IPlayerInfoType["player"]) => void;
  mode: Exclude<ModeType, null>;
}

export class PlayerSelection extends React.Component<IPlayerSelectionProps, {}> {
  public render() {
    const { onSelectPlayer, mode } = this.props;

    return (
      <SelectionLayout
        contentItemKey={[PLAYER_INFO_PLAYER_CIRCLE, PLAYER_INFO_PLAYER_CROSS]}
        onItemClick={onSelectPlayer as (itemKey: string) => void}
        onGenerateLabel={generateIconLabel as (itemKey: string) => IconProp}
        title={`Select ${mode === MODE_SINGLE ? "your" : "1st"} player`}
        labelAsIcon={true}
        doubleItem={true}
      />
    );
  }
}
