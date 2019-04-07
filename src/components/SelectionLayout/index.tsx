import * as React from "react";

import { IconProp } from "@fortawesome/fontawesome-svg-core";

import { SelectionContent } from "./SelectionContent";

import "./index.scss";

interface ISelectionLayoutProps {
  onItemClick: (itemKey: string) => void;
  onGenerateLabel: (itemKey: string) => string | IconProp;
  title: string;
  contentItemKey: [string, string] | string;
  labelAsIcon?: boolean;
  doubleItem?: boolean;
}

export class SelectionLayout extends React.Component<ISelectionLayoutProps, {}> {
  public render() {
    const {
      onItemClick,
      contentItemKey,
      title,
      onGenerateLabel,
      labelAsIcon,
      doubleItem,
    } = this.props;

    return (
      <div className="selection-layout">
        <header>
          <h2>{title}</h2>
        </header>
        <SelectionContent
          contentItemKey={contentItemKey}
          labelAsIcon={labelAsIcon}
          doubleItem={doubleItem}
          onGenerateLabel={onGenerateLabel}
          onItemClick={onItemClick}
        />
      </div>
    );
  }
}
