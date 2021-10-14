use futures_signals::signal::Mutable;
use gloo::events::EventListener;
use shared::domain::jig::module::body::legacy::design::{
    Sprite as RawSprite,
    Animation
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{cell::RefCell, rc::Rc, sync::atomic::AtomicBool};
use web_sys::{Blob, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData, Worker, window};
use crate::base::state::{Base, WorkerKind};
use std::io::Cursor;
use utils::prelude::*;
use dominator::clone;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};
use super::animation::*;

pub enum Sprite {
    Image(Rc<ImagePlayer>),
    Animation(Rc<AnimationPlayer>),
}

impl Sprite {
    pub fn new(base: Rc<Base>, raw: RawSprite) -> Self {
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

pub struct ImagePlayer {
    pub base: Rc<Base>,
    pub raw: RawSprite,
    pub size: Mutable<Option<(f64, f64)>>,
    pub hide: HideController
}

impl ImagePlayer {
    pub fn new(base: Rc<Base>, raw: RawSprite) -> Rc<Self> {

        let hide = HideController::new(&raw);

        Rc::new(Self{
            base,
            raw,
            size: Mutable::new(None),
            hide
        })
    }
}