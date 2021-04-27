use dominator::{Dom, html, clone};
use std::rc::Rc;
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use strum::IntoEnumIterator;
use super::super::super::wysiwyg_types::{Align, ElementType, Font, Weight};
use super::super::super::state::State;
use super::color_controls;

const STR_WEIGHT_LABEL: &'static str = "Weight";
const STR_FONT_LABEL: &'static str = "Font";


pub fn render(state: Rc<State>) -> Dom {
    html!("text-editor-controls", {
        .children(&mut [
            html!("dropdown-select", {
                .property("slot", "font")
                .property("label", STR_FONT_LABEL)
                .property_signal("value", state.controls.signal_cloned().map(|controls| controls.font.to_string()))
                .children(Font::iter().map(|font| render_font_option(state.clone(), font)))
            }),
            html!("button-collection", {
                .property("slot", "type")
                .children(ElementType::iter()
                    .map(|element| render_element_option(state.clone(), element))
                )
            }),
            html!("dropdown-select", {
                .property("slot", "weight")
                .property("label", STR_WEIGHT_LABEL)
                .property_signal("value", state.controls.signal_cloned().map(|controls| controls.weight.to_string()))
                .children(Weight::iter().map(|weight| render_weight_option(state.clone(), weight)))
            }),
            html!("input-inc-dec", {
                .property("slot", "font-size")
                .property("min", 1)
                .property("max", 50)
                .property_signal("value", state.controls.signal_cloned().map(|controls| {
                    controls.font_size
                }))
                .event(clone!(state => move |e: events::CustomChange| {
                    let value = e.value();
                    let value = u8::from_str_radix(&value, 10).unwrap_or(24);
                    state.set_font_size(value);
                }))
            }),
            html!("button-collection", {
                .property("slot", "style")
                .children(&mut [
                    html!("text-editor-control", {
                        .property("type", "bold")
                        .property_signal("active", state.controls.signal_cloned().map(|controls| {
                            controls.bold
                        }))
                        .event(clone!(state => move |_: events::Click| {
                            state.toggle_bold();
                        }))
                    }),
                    html!("text-editor-control", {
                        .property("type", "italic")
                        .property_signal("active", state.controls.signal_cloned().map(|controls| {
                            controls.italic
                        }))
                        .event(clone!(state => move |_: events::Click| {
                            state.toggle_italic();
                        }))
                    }),
                    html!("text-editor-control", {
                        .property("type", "underline")
                        .property_signal("active", state.controls.signal_cloned().map(|controls| {
                            controls.underline
                        }))
                        .event(clone!(state => move |_: events::Click| {
                            state.toggle_underline();
                        }))
                    }),
                ])
            }),
            color_controls::render(state.clone()),
            html!("button-collection", {
                .property("slot", "justify")
                .children(Align::iter()
                    .map(|align| render_align_option(state.clone(), align))
                )
                .children(&mut [
                    html!("text-editor-control", {
                        .property("type", "dir-ltr")
                        .property_signal("active", state.controls.signal_cloned().map(|controls| {
                            controls.indent_count > 0
                        }))
                        .event(clone!(state => move |_: events::Click| {
                            let mut count: u8 = state.controls.lock_ref().indent_count + 1;
                            state.set_indent_count(count);
                        }))
                    }),
                    html!("text-editor-control", {
                        .property("type", "dir-rtl")
                        .event(clone!(state => move |_: events::Click| {
                            let mut count: u8 = state.controls.lock_ref().indent_count;
                            if count > 0 {
                                count = count - 1;
                            }
                            state.set_indent_count(count);
                        }))
                        .property_signal("active", state.controls.signal_cloned().map(|controls| {
                            controls.indent_count == 0
                        }))
                    }),
                ])
            }),
            html!("button-sidebar", {
                .property("slot", "hewbrew-keyboard")
                .property("mode", "keyboard")
            }),
            html!("button-sidebar", {
                .property("slot", "dicta")
                .property("mode", "dicta")
            }),
            html!("button-sidebar", {
                .property("slot", "sefaria")
                .property("mode", "sefaria")
            }),
        ])
    })
}


fn render_element_option(state: Rc<State>, element: ElementType) -> Dom {
    html!("text-editor-control", {
        .property("type", element.to_string())
        .property_signal("active", state.controls.signal_cloned().map(clone!(state, element => move |controls| {
            if controls.element == element {
                true
            } else {
                false
            }
        })))
        .event(clone!(state, element => move |_: events::Click| {
            state.set_element(element.clone());
        }))
    })
}

fn render_align_option(state: Rc<State>, align: Align) -> Dom {
    html!("text-editor-control", {
        .property("type", match align {
            Align::Left => "align-left",
            Align::Center => "align-center",
            Align::Right => "align-right",
        })
        .property_signal("active", state.controls.signal_cloned().map(clone!(state, align => move |controls| {
            if controls.align == align {
                true
            } else {
                false
            }
        })))
        .event(clone!(state, align => move |_: events::Click| {
            state.set_align(align.clone());
        }))
    })
}

fn render_weight_option(state: Rc<State>, weight: Weight) -> Dom {
    html!("li-check", {
        .property_signal("selected", state.controls.signal_cloned().map(clone!(state, weight => move |controls| {
            if controls.weight == weight {
                true
            } else {
                false
            }
        })))
        .text(&weight.to_string())
        .event(clone!(state, weight => move |_: events::Click| {
            state.set_weight(weight.clone());
        }))
    })
}

fn render_font_option(state: Rc<State>, font: Font) -> Dom {
    html!("li-check", {
        .property_signal("selected", state.controls.signal_cloned().map(clone!(state, font => move |controls| {
            if controls.font == font {
                true
            } else {
                false
            }
        })))
        .text(&font.to_string())
        .event(clone!(state, font => move |_: events::Click| {
            state.set_font(font.clone());
        }))
    })
}