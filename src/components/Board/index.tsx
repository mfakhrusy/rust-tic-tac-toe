import * as React from "react";

import { Square } from "components/Square";

import "./index.scss";

interface BoardProps {};
interface BoardState {};

export class Board extends React.Component<BoardProps, BoardState> {
  render() {
    return (
      <div className="board">
        <div className="board-row board-top">
          <Square />
          <Square />
          <Square />
        </div>
        <div className="board-row board-middle">
          <Square />
          <Square />
          <Square />
        </div>
        <div className="board-row board-bottom">
          <Square />
          <Square />
          <Square />
        </div>
      </div>
    )
  }
}
