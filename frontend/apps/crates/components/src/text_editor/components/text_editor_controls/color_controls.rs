use std::rc::Rc;
use dominator::{Dom, html, clone};
use super::super::super::state::State;
use futures_signals::signal::Mutable;
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use wasm_bindgen_futures::spawn_local;
use futures::future::ready;
use rgb::{RGB8, RGBA8};
use crate::color_select::{
    self,
    state::ColorSelectConfig,
    actions::{hex_to_rgba8, rgba8_to_hex},
};


#[derive(Clone)]
enum ColorSelectFor {
    Text,
    Highlight,
}

pub fn render(state: Rc<State>) -> Dom {
    let select_for: Rc<Mutable<Option<ColorSelectFor>>> = Rc::new(Mutable::new(None));
    let select_value: Rc<Mutable<Option<RGBA8>>> = Rc::new(Mutable::new(None));

    spawn_local(select_value.signal_cloned().for_each(clone!(state, select_for => move |color| {
        let color = rgba8_to_hex_optional(&color);
        match select_for.lock_ref().as_ref() {
            Some(ColorSelectFor::Highlight) => {state.set_highlight_color(color)},
            Some(ColorSelectFor::Text) => {state.set_color(color)},
            None => {}
        };
        ready(())
    })));

    Dom::with_state(select_for, move |select_for| {
        html!("anchored-overlay", {
            .property("slot", "color")
            .property("backdropColor", "transparent")
            .property("positionY", "top-in")
            .property_signal("open", select_for.signal_cloned().map(|select_for| select_for.is_some()))
            .event(clone!(select_for => move |_: events::Close| {
                select_for.set(None);
            }))
            .child(html!("button-collection", {
                .property("slot", "anchor")
                .children(&mut [
                    html!("text-editor-control", {
                        .property("type", "color")
                        .event(clone!(state, select_for, select_value => move |_: events::Click| {
                            select_for.set(Some(ColorSelectFor::Highlight));
                            select_value.set(hex_to_rgba8_optional(&state.controls.lock_ref().color));
                        }))
                    }),
                    html!("text-editor-control", {
                        .property("type", "marker-color")
                        .event(clone!(state, select_for, select_value => move |_: events::Click| {
                            select_for.set(Some(ColorSelectFor::Text));
                            select_value.set(hex_to_rgba8_optional(&state.controls.lock_ref().color));
                        }))
                    }),
                ])
            }))
            .child(html!("text-editor-controls-overlay-shadow", {
                .property("slot", "overlay")
                .child(color_select::dom::render(ColorSelectConfig {
                    theme: None,
                    value: select_value.clone()
                }, None))
            }))
        })
    })
}


fn hex_to_rgba8_optional(color: &Option<String>) -> Option<RGBA8> {
    match color {
        Some(color) => Some(hex_to_rgba8(&color)),
        None => None,
    }
}
fn rgba8_to_hex_optional(color: &Option<RGBA8>) -> Option<String> {
    match *color {
        Some(color) => Some(rgba8_to_hex(&color)),
        None => None,
    }
}
