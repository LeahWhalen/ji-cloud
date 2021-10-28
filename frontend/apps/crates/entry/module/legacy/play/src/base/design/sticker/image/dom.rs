
use crate::base::state::Base;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};

use shared::domain::jig::module::body::legacy::design::{
    Sticker as RawSticker
};
use std::{borrow::Borrow, rc::Rc, cell::RefCell};
use utils::{
    math::{bounds, mat4::Matrix4},
    path,
    prelude::*,
    resize::resize_info_signal,
};
use wasm_bindgen::JsCast;
use awsm_web::{canvas::{get_2d_context, CanvasToBlobFuture}, data::ArrayBufferExt};
use super::state::*;
use super::super::helpers::*;

impl ImagePlayer {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        let transform_matrix = Matrix4::new_direct(state.raw.transform_matrix.clone());
        let transform_signal = resize_info_signal().map(move |resize_info| {
            let mut m = transform_matrix.clone();
            m.denormalize(&resize_info);
            m.as_matrix_string()
        });


        html!("img" => web_sys:: HtmlImageElement, {
            .attribute("src", &state.base.design_media_url(&state.raw.filename))
            .style_signal("opacity", state.controller.hidden.signal().map(|hidden| {
                if hidden {
                    "0"
                } else {
                    "1"
                }
            }))
            .style("cursor", if state.controller.interactive {"pointer"} else {"initial"})
            .style("display", "block")
            .style("position", "absolute")
            .style_signal("width", width_signal(state.size.signal_cloned()))
            .style_signal("height", height_signal(state.size.signal_cloned()))
            .style_signal("top", bounds::size_height_center_rem_signal(state.size.signal()))
            .style_signal("left", bounds::size_width_center_rem_signal(state.size.signal()))
            .style_signal("transform", transform_signal)
            .with_node!(elem => {
                .event(clone!(state => move |_evt:events::Load| {
                    if state.size.get_cloned().is_none() {
                        let width = elem.natural_width() as f64;
                        let height = elem.natural_height() as f64;

                        state.size.set(Some((width, height)));

                        *state.controller.elem.borrow_mut() = Some(elem.clone().unchecked_into());
                        state.base.insert_stage_click_listener(clone!(state => move |stage_click| {
                            state.controller.handle_click(stage_click);
                        }));
                    }
                }))
            })
            .event(clone!(state => move |_evt:events::Click| {
                log::info!("clicked!")
            }))
        })
    }
}