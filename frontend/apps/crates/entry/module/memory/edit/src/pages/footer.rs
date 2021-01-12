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


pub struct Footer {
    state: Rc<State>, 
    game_mode: GameMode,
}

impl Footer {

    pub fn render(state: Rc<State>, game_mode: GameMode) -> impl Signal<Item = Option<Dom>> {
        let _self = Self::new(state, game_mode);

        _self.state.step.signal().map(clone!(_self => move |step|
            match step {
                Step::Four => None,
                _ => Some(Self::render_default(_self.clone()))
            }
        ))
    }
    fn new(state: Rc<State>, game_mode:GameMode) -> Rc<Self> {
        Rc::new(Self { 
            state, 
            game_mode,
        })
    }


    pub fn render_default(_self: Rc<Self>) -> Dom {
        elem!(templates::footer_default(), {
            .with_data_id!("next", {
                .event(clone!(_self => move |evt:events::Click| {
                    let curr_step = _self.state.step.get();
                    let content_mode = _self.state.content_mode.get();
                    if content_mode != ContentMode::TextInit {
                        let next_step = match curr_step {
                            Step::One => Some(Step::Two),
                            Step::Two => Some(Step::Three),
                            Step::Three => Some(Step::Four),
                            _ => None
                        };

                        if let Some(next_step) = next_step {
                            _self.state.step.set(next_step);
                        }
                    }
                }))
            })
        })
    }
}
