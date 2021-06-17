import {ThemeKind} from "@elements/_themes/themes";

export const themeIconPath = (theme:ThemeKind, hover:boolean):string => {
    return `theme/module/_groups/design/${theme}/icon${hover ? "-hover" : ""}.jpg`;
}