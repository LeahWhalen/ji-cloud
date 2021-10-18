use super::state::*;
use components::{
    module::_common::edit::prelude::*,
    overlay::container::OverlayContainer
};
use dominator::{html, Dom};
use std::rc::Rc;

impl DomRenderable for Overlay {
    fn render(_state: Rc<Overlay>) -> Dom {
        OverlayContainer::new().render()
    }
}
