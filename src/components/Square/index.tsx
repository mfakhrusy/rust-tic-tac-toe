import * as React from "react";

import { IconProp } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { generateIconLabel } from "utils/helpers";

import { BoardSquareContentType } from "types/custom";

import "./index.scss";

const CONTENT_EMPTY = "";

interface IBoardSquareProps {
  content: BoardSquareContentType;
  x: number;
  y: number;
  onClick: (x: number, y: number) => void;
}

export class Square extends React.Component<IBoardSquareProps, {}> {
  public render() {
    const { content } = this.props;

    const iconLabel: IconProp = generateIconLabel(content as Exclude<IBoardSquareProps["content"], "">);

    return (
      <div className="square">
        <button onClick={this.handleClick}>
          {content === CONTENT_EMPTY ? null : <FontAwesomeIcon icon={iconLabel} />}
        </button>
      </div>
    );
  }

  private handleClick = (e: React.MouseEvent): void => {
    e.preventDefault();

    const { onClick, x, y } = this.props;

    onClick(x, y);
  }
}
