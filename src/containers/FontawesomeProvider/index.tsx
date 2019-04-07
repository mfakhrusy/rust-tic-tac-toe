import * as React from "react";

import { library } from "@fortawesome/fontawesome-svg-core";
import { faCircle } from "@fortawesome/free-regular-svg-icons";
import { faArrowLeft, faArrowRight, faTimes } from "@fortawesome/free-solid-svg-icons";

library.add(
  faCircle,
  faTimes,
  faArrowLeft,
  faArrowRight,
);

interface IFontawesomeProviderProps {
  children: React.ReactNode;
}

export class FontawesomeProvider extends React.Component<IFontawesomeProviderProps, {}> {
  public render() {
    const { children } = this.props;

    return (
      <React.Fragment>
        {children}
      </React.Fragment>
    );
  }
}
