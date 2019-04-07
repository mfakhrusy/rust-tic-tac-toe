import * as React from "react";

import { IconProp } from "@fortawesome/fontawesome-svg-core";

import { ContentItem } from "./ContentItem";

interface ISelectionContentProps {
  onItemClick: (itemKey: string) => void;
  onGenerateLabel: (itemKey: string) => string | IconProp;
  contentItemKey: [string, string] | string;
  labelAsIcon?: boolean;
  doubleItem?: boolean;
}

export class SelectionContent extends React.Component<ISelectionContentProps, {}> {
  public render() {
    const {
      doubleItem,
      onGenerateLabel,
      contentItemKey,
      onItemClick,
      labelAsIcon,
    } = this.props;

    return (
      <div className={`content ${doubleItem ? "double-item" : "single-item"}`}>
        {
          doubleItem ? (
            <React.Fragment>
              <ContentItem
                buttonLabel={onGenerateLabel(contentItemKey[0])}
                itemKey={contentItemKey[0]}
                onClick={onItemClick}
                labelAsIcon={labelAsIcon}
              />
              <ContentItem
                buttonLabel={onGenerateLabel(contentItemKey[1])}
                itemKey={contentItemKey[1]}
                onClick={onItemClick}
                labelAsIcon={labelAsIcon}
              />
           </React.Fragment>
          ) : (
            <ContentItem
              buttonLabel={onGenerateLabel(contentItemKey as string)}
              itemKey={contentItemKey as string}
              onClick={onItemClick}
              labelAsIcon={labelAsIcon}
            />
          )
        }
      </div>
    );
  }
}
