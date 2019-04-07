import * as React from "react";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { IconProp } from "@fortawesome/fontawesome-svg-core";

import "./ContentItem.scss";

interface IContentItemProps {
  buttonLabel: string | IconProp;
  itemKey: string;
  onClick: (itemKey: string) => void;
  labelAsIcon?: boolean;
}

export class ContentItem extends React.Component<IContentItemProps, {}> {
  public render() {
    const {
      buttonLabel,
      labelAsIcon,
    } = this.props;

    return (
      <div className={`content-item ${labelAsIcon ? "icon-label" : ""}`}>
        <button
          onClick={this.handleClick}
        >
          {labelAsIcon ? <FontAwesomeIcon icon={buttonLabel as IconProp} /> : buttonLabel}
        </button>
      </div>
    );
  }

  private handleClick = (e: React.MouseEvent ): void => {
    e.preventDefault();
    const { onClick, itemKey } = this.props;

    onClick(itemKey);
  }
}
