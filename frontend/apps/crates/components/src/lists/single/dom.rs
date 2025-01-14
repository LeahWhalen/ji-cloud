use dominator::{clone, html, Dom};
use std::rc::Rc;
use utils::prelude::*;

use super::state::*;
use crate::{
    hebrew_buttons::HebrewButtons,
    overlay::handle::OverlayHandle,
    tooltip::{
        callbacks::TooltipErrorCallbacks,
        state::{
            Anchor, ContentAnchor, MoveStrategy, State as TooltipState, TooltipData, TooltipError,
            TooltipTarget,
        },
    },
};
use futures_signals::{map_ref, signal::SignalExt, signal_vec::SignalVecExt};

const STR_DELETE_TITLE: &str = "Warning";
const STR_DELETE_CONTENT: &str = "Are you sure you want to delete this list?";
const STR_DELETE_CONFIRM: &str = "Yes, go ahead!";
const STR_DELETE_CANCEL: &str = "No, keep this list";

pub fn render(state: Rc<State>) -> Dom {
    html!("sidebar-widget-single-list", {
        .children(&mut [
            HebrewButtons::full().render(Some("hebrew-buttons")),
            html!("button-rect", {
                .property("slot", "clear")
                .property("kind", "text")
                .property("color", "blue")
                .text(super::strings::STR_CLEAR)
                .event(clone!(state => move |_evt:events::Click| {
                    state.confirm_clear.set_neq(true);
                }))
            }),
            html!("button-rect", {
                .property_signal("disabled", state.is_valid_signal().map(|valid| !valid))
                .property("size", "small")
                .property("iconAfter", "done")
                .property("slot", "done-btn")
                .text(super::strings::STR_DONE)
                .event(clone!(state => move |_evt:events::Click| {
                    match state.derive_list() {
                        Some(list) => {
                            (state.callbacks.replace_list) (list);
                        },
                        None => {
                            (state.callbacks.set_tooltip_error) (Some(
                                    Rc::new(TooltipState::new(
                                        TooltipTarget::Element(
                                            state.error_element_ref.borrow().as_ref().unwrap_ji().clone(),
                                            MoveStrategy::None
                                        ),

                                        TooltipData::Error(Rc::new(TooltipError {
                                            max_width: Some(185.0),
                                            target_anchor: Anchor::MiddleRight,
                                            content_anchor: ContentAnchor::OppositeH,
                                            body: super::strings::error::STR_NUM_WORDS.to_string(),
                                            callbacks: TooltipErrorCallbacks::new(
                                                Some(clone!(state => move || {
                                                    (state.callbacks.set_tooltip_error) (None);
                                                }))
                                            )
                                        }))
                                    ))
                            ));
                        }
                    }
                }))
            })
        ])
        .children_signal_vec(
            state.list.signal_vec_cloned()
                .enumerate()
                .map(clone!(state => move |(index, value)| {

                        let index = index.get().unwrap_or_default();

                        html!("sidebar-widget-single-list-input", {
                            .property_signal("value", {
                                clone!(state => map_ref! {
                                    let value = value.signal_cloned(),
                                    let is_placeholder = state.is_placeholder.signal()
                                        => move {
                                            if *is_placeholder {
                                                (state.callbacks.get_placeholder) (index)
                                                    .unwrap_or_else(|| "".to_string())
                                            } else {
                                                value.clone()
                                            }
                                        }
                                })
                            })
                            .property("constrain", state.callbacks.constrain.as_ref())
                            .property_signal("placeholder", state.is_placeholder.signal())
                            .event(clone!(state => move |_evt:events::Focus| {
                                //log::info!("got focus!");
                                state.is_placeholder.set_neq(false);
                            }))
                            .event(move |evt:events::CustomInput| {
                                value.set_neq(evt.value());
                            })
                            .after_inserted(clone!(index, state => move |elem| {
                                if index == 2 {
                                    *state.error_element_ref.borrow_mut() = Some(elem);
                                }

                            }))
                        })
                }))
        )
        .child_signal(state.confirm_clear.signal_cloned().map(clone!(state => move |confirm_clear| {
            if confirm_clear {
                Some(html!("empty-fragment", {
                    .style("display", "none")
                    .apply(OverlayHandle::lifecycle(clone!(state => move || {
                        html!("modal-confirm", {
                            .property("dangerous", true)
                            .property("title", STR_DELETE_TITLE)
                            .property("content", STR_DELETE_CONTENT)
                            .property("cancel_text", STR_DELETE_CANCEL)
                            .property("confirm_text", STR_DELETE_CONFIRM)
                            .event(clone!(state => move |_evt: events::CustomCancel| state.confirm_clear.set_neq(false)))
                            .event(clone!(state => move |_evt: events::CustomConfirm| {
                                state.confirm_clear.set_neq(false);
                                state.clear();
                            }))
                        })
                    })))
                }))
            } else {
                None
            }
        })))
    })
}
