use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{always, Signal, ReadOnlyMutable, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::jig::module::body::{Sprite, Transform};
use super::{
    super::state::Text,
    super::super::state::Stickers
};

pub fn render(stickers:Rc<Stickers>, index: ReadOnlyMutable<Option<usize>>, text: Rc<Text>) -> Dom {
    html!("div", {
        .children(&mut [
            html!("menu-line", {
                .property("icon", "duplicate")
                .event(clone!(stickers, text => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "move-forward")
                .event(clone!(stickers, text => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "move-backward")
                .event(clone!(stickers, text => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
            html!("menu-line", {
                .property("icon", "delete")
                .event(clone!(stickers, text => move |evt:events::Click| {
                    log::info!("TODO!");
                }))
            }),
        ])
    })
}