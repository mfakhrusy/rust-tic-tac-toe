import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as React from "react";

import "./index.scss";

interface INavigatorProps {
  showBackButton: boolean;
  showForwardButton: boolean;
  onClickBack: (e: React.MouseEvent) => void;
  onClickForward: (e: React.MouseEvent) => void;
}

export class Navigator extends React.Component<INavigatorProps, {}> {
  public render() {
    const {
      showBackButton,
      showForwardButton,
      onClickBack,
      onClickForward,
    } = this.props;

    return (
      <div className="navigator">
        <div className={`nav-item nav-back ${showBackButton ? "" : "hidden"}`}>
          <button onClick={onClickBack}>
            <FontAwesomeIcon icon={["fas", "arrow-left"]} />
          </button>
        </div>
        <div className={`nav-item nav-forward ${showForwardButton ? "" : "hidden"}`}>
          <button onClick={onClickForward}>
            <FontAwesomeIcon icon={["fas", "arrow-right"]} />
          </button>
        </div>
      </div>
    );
  }
}
