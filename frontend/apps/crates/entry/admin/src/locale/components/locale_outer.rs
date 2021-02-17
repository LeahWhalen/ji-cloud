use super::table::TableComponent;
use dominator::{Dom, html, with_node, clone};
use std::rc::Rc;
use super::super::state::*;
use web_sys::HtmlSelectElement;
use utils::events;


pub struct LocaleOuterDom {

}

impl LocaleOuterDom {
    pub fn render(state: Rc<State>) -> Dom {

        html!("main", {
            .children(&mut [
                html!("select" => HtmlSelectElement, {
                    .attribute("multiple", "")
                    .with_node!(elem => {
                        .event(clone!(elem => move |_:events::Change| {
                            let selected_bundle: Bundle = elem.value();
                            super::super::temp_utils::log(&selected_bundle);
                        }))
                    })
                    .children(
                        state.bundles.iter().map(|(e, selected)| {
                            html!("option", {
                                .property("text", e.to_string())
                                .property("value", e.to_string())
                                .property("selected", selected.clone())
                            })
                        })
                    )
                }),
                html!("div", {
                    .class("icon-button")
                    .class("select-columns")
                    .children(&mut [
                        html!("button", {
                            .child(html!("img", {
                                .attribute("src", "assets/select-columns-icon.png")
                            }))
                            .event(clone!(state => move |_event: events::Click| {
                                state.dialog_ref
                                    .lock_ref().clone().expect("Can't get dialog")
                                    .show_modal().expect("Can't open dialog");
                            }))
                        }),
                        html!("span", {
                            .text("Select columns to display")
                        }),
                    ])
                }),
                html!("div", {
                    .class("icon-button")
                    .class("add-text")
                    .children(&mut [
                        html!("button", {
                            .child(html!("img", {
                                .attribute("src", "assets/add-icon.png")
                            }))
                            .event(clone!(state => move |_event: events::Click| {
                                state.loader.load(clone!(state => async move {
                                    state.add_entry().await;
                                }))
                            }))
                        }),
                        html!("span", {
                            .text("Add a text")
                        }),
                    ])
                }),
                TableComponent::render(state),
            ])
        })
    }
}