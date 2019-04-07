import { IconProp } from "@fortawesome/fontawesome-svg-core";
import { PLAYER_INFO_PLAYER_CIRCLE, PLAYER_INFO_PLAYER_CROSS } from "utils/constants";

export const generateIconLabel = (
    label: typeof PLAYER_INFO_PLAYER_CIRCLE | typeof PLAYER_INFO_PLAYER_CROSS,
): IconProp => {
  let iconLabel: IconProp = ["fab", "toolbox"];

  switch (label) {
    case PLAYER_INFO_PLAYER_CROSS:
      iconLabel = ["fas", "times"];
      break;
    case PLAYER_INFO_PLAYER_CIRCLE:
      iconLabel = ["far", "circle"];
      break;
    default:
      break;
  }

  return iconLabel;
};
