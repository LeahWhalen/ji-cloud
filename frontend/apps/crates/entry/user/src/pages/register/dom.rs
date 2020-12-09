use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::UnwrapThrowExt;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, HtmlInputElement};
use dominator::{Dom, html, events, clone};
use dominator_helpers::{elem, with_data_id};
use crate::templates;
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use utils::{
    routes::{Route, UserRoute},
    firebase::*,
    storage,
};

use super::sections::{start::RegisterStart, step1::RegisterStep1, step2::RegisterStep2, step3::RegisterStep3, misc::RegisterFinal};
use super::data::*;

pub struct RegisterPage {
    pub step: Rc<Mutable<Step>>,
    pub data: Rc<RefCell<RegisterData>>
}


impl RegisterPage  {
    pub fn new(user:Option<FirebaseUserInfo>) -> Rc<Self> {
        let _self = Rc::new(Self { 
            step: Rc::new(Mutable::new({
                if user.is_some() {
                    Step::One
                } else {
                    Step::Start
                }
            })),
            data: Rc::new(RefCell::new(user.into())),
        });
        _self
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        let step = _self.step.clone();

        html!("div", {
            .child_signal(_self.step.signal_ref(clone!(_self => move |step| {
                Some(match step {
                    Step::Start => RegisterStart::render(RegisterStart::new(_self.step.clone(), _self.data.clone())),
                    Step::One => RegisterStep1::render(RegisterStep1::new(_self.step.clone(), _self.data.clone())),
                    Step::Two=> RegisterStep2::render(RegisterStep2::new(_self.step.clone(), _self.data.clone())),
                    Step::Three=> RegisterStep3::render(RegisterStep3::new(_self.step.clone(), _self.data.clone())),
                    Step::Final=> RegisterFinal::render(RegisterFinal::new(_self.step.clone(), _self.data.clone())),
                })
            })))
        })
    }
}
