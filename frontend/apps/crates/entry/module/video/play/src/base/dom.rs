use components::{
    backgrounds::dom::render_backgrounds_raw,
    module::_common::play::prelude::DomRenderable,
    stickers::{
        sprite::dom::render_sticker_sprite_raw,
        text::dom::render_sticker_text_raw,
        video::dom::render_sticker_video_raw
    }
};
use dominator::{Dom, clone, html};
use shared::domain::jig::module::body::_groups::design::Sticker as RawSticker;
use std::rc::Rc;
use crate::base::actions;

use super::state::*;


impl DomRenderable for Base {
    fn render(state: Rc<Base>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .child(
                render_backgrounds_raw(&state.backgrounds, state.theme_id, None)
            )
            .children(
                state.stickers
                    .iter()
                    .map(clone!(state => move |sticker| {
                        match sticker {
                            RawSticker::Sprite(sprite) => render_sticker_sprite_raw(sprite, None),
                            RawSticker::Text(text) => render_sticker_text_raw(text, state.theme_id, None),
                            RawSticker::Video(video) => {
                                let opts = actions::create_video_sticker_options(&state.play_settings);

                                render_sticker_video_raw(video, Some(opts))
                            },
                        }
                    }))
                    .collect::<Vec<Dom>>()
            )
        })
    }
}