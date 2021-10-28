use futures_signals::signal::Mutable;
use gloo::events::EventListener;
use shared::domain::jig::module::body::legacy::design::{
    Sticker as RawSticker,
    Animation
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{cell::RefCell, rc::Rc, sync::atomic::AtomicBool};
use web_sys::{Blob, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData, Worker, window};
use crate::base::state::Base;
use std::io::Cursor;
use utils::prelude::*;
use dominator::clone;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};
use super::animation::AnimationPlayer;
use super::image::ImagePlayer;

pub enum Sticker {
    Image(Rc<ImagePlayer>),
    Animation(Rc<AnimationPlayer>),
}

impl Sticker {
    pub fn new(base: Rc<Base>, raw: RawSticker) -> Self {
        match raw.animation.clone() {
            Some(animation) => {
                Self::Animation(AnimationPlayer::new(base, raw, animation))
            },
            None => {
                Self::Image(ImagePlayer::new(base, raw))
            }
        }
    }
}