import * as React from "react";
import { ModeType } from "types/custom";

import "./index.scss";

interface IModeInfoProps {
  mode: ModeType;
}

export class ModeInfo extends React.Component<IModeInfoProps, {}> {
  public render() {
    const { mode } = this.props;

    return (
      <div className="mode-info">
        <span>Mode: </span><span>{mode === null ? "" : ` ${mode}`}</span>
      </div>
    );
  }
}
