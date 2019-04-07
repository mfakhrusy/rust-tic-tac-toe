import * as React from "react";

import { IconProp } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { generateIconLabel } from "utils/helpers";

import { PLAYER_INFO_PLAYER_CIRCLE, PLAYER_INFO_PLAYER_CROSS } from "utils/constants";
import "./index.scss";

const CONTENT_EMPTY = "";

interface ISquareProps {
  content: typeof PLAYER_INFO_PLAYER_CIRCLE | typeof PLAYER_INFO_PLAYER_CROSS | "";
}

export class Square extends React.Component<ISquareProps, {}> {
  public static defaultProps = {
    content: CONTENT_EMPTY,
  };

  public render() {
    const { content } = this.props;

    const iconLabel: IconProp = generateIconLabel(content as Exclude<ISquareProps["content"], "">);

    return (
      <div className="square">
        {content === CONTENT_EMPTY ? null : <FontAwesomeIcon icon={iconLabel} />}
      </div>
    );
  }
}
