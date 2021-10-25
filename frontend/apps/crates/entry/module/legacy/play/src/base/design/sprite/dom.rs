use crate::base::state::Base;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};

use shared::domain::jig::module::body::legacy::design::{
    Sprite as RawSprite
};
use std::{borrow::Borrow, rc::Rc, cell::RefCell};
use utils::{
    math::{bounds, mat4::Matrix4},
    path,
    prelude::*,
    resize::resize_info_signal,
};
use awsm_web::{canvas::{get_2d_context, CanvasToBlobFuture}, data::ArrayBufferExt};
use super::{animation::AnimationPlayer, image::ImagePlayer, state::{Sprite}};

// http://localhost:4104/module/legacy/play/debug?game_id=17736&slide_index=0&example=true
impl Sprite {
    pub fn render(self: Self) -> Dom {
        match self {
            Self::Image(state) => state.render(),
            Self::Animation(state) => state.render()
        }
    }
}
