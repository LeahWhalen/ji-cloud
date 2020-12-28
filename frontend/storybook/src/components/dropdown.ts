import "@elements/dropdowns/selected-dropdown";
import "@elements/dropdowns/tree-dropdown";

import {ListHover} from "~/components/lists/school";
import { InputUnderlined } from "./input";
import "@elements/inputs/checkbox";

export default {
  title: 'Dropdown',
}


export const SelectedDropdown = ({label}) => {
    return `<selected-dropdown label="Title">
    <div slot="search">${InputUnderlined(label)}</div>
      <div slot="list">${ListHover()}</div>
    </selected-dropdown>
`
}

export const TreeDropdown = ({label, path}) => {
  return `<tree-dropdown label="${label}" path="${path}">

  </tree-dropdown>
`
}


SelectedDropdown.args = {
  label: "Search"
}

TreeDropdown.args = {
  label: "Category",
  path: "/icon-chevron-categories-24-px.svg"
}
