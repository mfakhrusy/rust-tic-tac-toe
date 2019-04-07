import * as React from "react";

import { FontawesomeProvider } from "containers/FontawesomeProvider";
import { MainPage } from "containers/MainPage";

import "./index.scss";

export class App extends React.Component<{}, {}> {
  public render() {
    return (
      <FontawesomeProvider>
        <main className="app">
          <MainPage />
        </main>
      </FontawesomeProvider>
    );
  }
}
