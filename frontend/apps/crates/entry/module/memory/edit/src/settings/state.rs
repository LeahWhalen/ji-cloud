use crate::state::Base;
use std::rc::Rc;
use shared::domain::jig::module::body::memory::ModuleData as RawData;

pub struct Settings {
}

impl Settings {
    pub fn new(raw:&RawData) -> Self {
        Self {
        }
    }
}
