
use std::rc::Rc;
use super::state::*;
use dominator::{Dom, html, svg, clone, EventOptions};
use utils::{math::BoundsF64, prelude::*, resize::{ResizeInfo, resize_info_signal}};
use futures_signals::{
    signal_vec::{self, SignalVecExt},
    signal::{self, Signal, SignalExt}
};
use crate::base::{
    styles::FULL_STAGE,
    activities::_common::hotspot::*
};

use components::traces::{
    svg::{ShapeStyle, ShapeStyleMode, ShapeStyleKind, ShapeStylePlayMode},
    bubble::TraceBubble
};

use shared::domain::jig::module::body::_groups::design::TraceKind;

impl Soundboard {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("div", {
            .class(&*FULL_STAGE)
            .child(
                svg!("svg", {
                    .class(&*FULL_STAGE)
                    .children_signal_vec(
                        resize_info_signal().map(clone!(state => move |resize_info| {
                            state.items
                                .iter()
                                .map(|item| item.clone().render_svg_path(state.clone(), &resize_info))
                                .collect()
                        }))
                        .to_signal_vec()
                    )
                })
            )
            // .child(
            //     html!("div", {
            //         .class(&*FULL_STAGE)
            //         .style("pointer-events", "none")
            //         .children_signal_vec(
            //             resize_info_signal()
            //                 .switch_signal_vec(clone!(state => move |resize_info| {
            //                     signal_vec::always(
            //                         state.items
            //                             .iter()
            //                             .map(|item| item.text_signal())
            //                             .collect()
            //                     )
            //                     .map_signal(|x| x)
            //                     .filter_map(|x| x)
            //                     .map(|text| {
            //                         TraceBubble::render(TraceBubble::new(
            //                             BoundsF64{
            //                                 x: 100.0,
            //                                 y: 100.0,
            //                                 width: 100.0,
            //                                 height: 100.0,
            //                                 invert_y: true
            //                             },
            //                             None,
            //                             Some(text),
            //                             None::<fn()>
            //                         ))
            //                     })
            //             }))
            //         )
            //     })
            // )
        })
    }
}

impl SoundboardItem {
    pub fn render_svg_path(self: Rc<Self>, parent: Rc<Soundboard>, resize_info: &ResizeInfo) -> Dom {
        let state = self;
        state.hotspot.render(
            &resize_info,
            clone!(state, parent => move || {
                state.clone().on_click(parent.clone());
            }),
            state.revealed.signal().map(|revealed| {
                ShapeStyle {
                    interactive: true,
                    mode: if revealed { ShapeStyleMode::Play(ShapeStylePlayMode::Selected) } else { ShapeStyleMode::Transparent },
                    kind: ShapeStyleKind::General,
                }
            })
        )
    }
}