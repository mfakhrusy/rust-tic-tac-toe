import * as React from "react";

import "./index.scss";

interface ICountdownProps {
  timeLimit: number; // in seconds
  onFinishCountdown: () => void;
}

interface ICountdownState {
  time: number;
}

export class Countdown extends React.Component<ICountdownProps, ICountdownState> {
  constructor(props: ICountdownProps) {
    super(props);

    this.state = {
      time: props.timeLimit,
    };
  }

  public componentDidMount() {
    setInterval(this.generateCountdown, 1000);
  }

  public render() {
    const { time } = this.state;
    return (
      <div className="countdown">
        <span>{time}</span>
      </div>
    );
  }

  private generateCountdown = (): void => {
    const { onFinishCountdown } = this.props;
    const { time } = this.state;

    if (time > 1) {
      this.setState((prevState) => ({
        time: prevState.time - 1,
      }));
    } else {
      onFinishCountdown();
    }
  }
}
