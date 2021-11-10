use std::{borrow::BorrowMut, rc::Rc};
use super::state::*;
use utils::prelude::*;
use dominator::{Dom, html, clone};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlVideoElement};
use js_sys::Reflect;
use futures_signals::signal::SignalExt;

impl Video {
    pub fn on_start(self: Rc<Self>) {
        self.start_gates.replace_with(|gates| {
            gates.module = true;
            *gates
        });
    }

    pub fn on_ended(&self) {
        if let Some(api) = self.yt_api.borrow().as_ref() {
            api.pause();
        }

        if let Some(api) = self.direct_api.borrow().as_ref() {
            api.pause();
        }

        log::info!("video finished, going next");
        let _ = IframeAction::new(ModuleToJigPlayerMessage::Next).try_post_message_to_top();
    }

}

impl YoutubeApi {
    pub fn play(&self) {
        let play_method = Reflect::get(
            &self.elem,
            &JsValue::from_str("play")
        ).unwrap();

        let play_method = play_method.dyn_ref::<js_sys::Function>().unwrap();
        let _ = play_method.call0(&self.elem);
    }

    pub fn pause(&self) {
        log::warn!("TODO: https://github.com/ji-devs/ji-cloud/issues/1841");
    }
}

impl DirectApi {
    pub fn play(&self) {
        self.elem.play();
    }

    pub fn pause(&self) {
        self.elem.pause();
    }
}