use dominator::{html, Dom};
use std::rc::Rc;

use wasm_bindgen::prelude::*;

use futures_signals::signal::SignalExt;

use super::card::dom::render as render_card;
use super::state::*;
use crate::module::_groups::cards::edit::state::*;

pub fn render<RawData: RawDataExt, E: ExtraExt>(state: Rc<MainPair<RawData, E>>) -> Dom {
    html!("main-card-pair", {
        .property_signal("index", state.index.signal().map(|x| {
            JsValue::from_f64(x.unwrap_or_default() as f64)
        }))
        .child(render_card(state.left.clone()))
        .child(render_card(state.right.clone()))
    })
}
