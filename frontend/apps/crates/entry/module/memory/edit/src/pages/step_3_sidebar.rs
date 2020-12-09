use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal, ReadOnlyMutable},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement, HtmlTextAreaElement};
use dominator::{DomBuilder, Dom, html, events, with_node, clone, apply_methods};
use dominator_helpers::{elem, with_data_id};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::data::*;
use crate::debug;
use super::steps_nav::apply_steps_nav;


pub struct Step3Sidebar {
    state: Rc<State>, 
    game_mode: GameMode,
}

impl Step3Sidebar {
    pub fn new(state: Rc<State>, game_mode:GameMode) -> Rc<Self> {
        Rc::new(Self { 
            state, 
            game_mode,
        })
    }

    pub fn render(_self: Rc<Self>) -> Dom {
        elem!(templates::sidebar_step_3(), {
            .apply(|dom| apply_steps_nav(dom, _self.state.clone()))
        })
    }
}
