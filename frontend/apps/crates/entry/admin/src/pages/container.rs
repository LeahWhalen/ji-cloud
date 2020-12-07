use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods};
use dominator_helpers::{elem, with_data_id};
use crate::templates;
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use utils::routes::{Route, AdminRoute};
use shared::domain::{
    user::UserProfile,
    category::Category,
    image::ImageKind,
};

pub struct ContainerPage
{ 
    sidebar_closed: Mutable<bool>
}

impl ContainerPage
{
    pub fn new() -> Rc<Self> {
        let _self = Rc::new(Self { 
            sidebar_closed: Mutable::new(false)
        });

        _self
    }

    fn links_dom_signal<R>(_self: Rc<Self>, route_signal: R) -> impl SignalVec<Item = Dom> 
    where 
        R: Signal<Item=Route> + 'static,

    {
        let make_link = |_self: Rc<Self>, label:&'static str, target_route:Route, route_matches: bool| {
            elem!(templates::sidebar_link( label, &String::from(target_route), false), {
                .apply_if(route_matches, |dom| {
                    dom.class("text-white")
                })
            })
        };

        route_signal
            .map(clone!(_self => move |route| {

                let (is_new_image, is_image_search, is_categories) = {
                    match route {
                       Route::Admin(route) => {
                           match route {
                               AdminRoute::ImageAdd => (true, false, false),
                               AdminRoute::ImageSearch(_) => (false, true, false),
                               AdminRoute::Categories => (false, false, true),
                               _ => (false, false, false)
                           }
                       },
                       _ => (false, false, false)
                    }
                };
                vec![
                    make_link(_self.clone(), "New Image", Route::Admin(AdminRoute::ImageAdd), is_new_image),
                    make_link(_self.clone(), "Search Images", Route::Admin(AdminRoute::ImageSearch(None)), is_image_search), 
                    make_link(_self.clone(), "Categories", Route::Admin(AdminRoute::Categories), is_categories), 
                    //make_link(_self.clone(), "JIGs", Route::Admin(AdminRoute::Images))
                ]
            }))
            .to_signal_vec()
    }
    
    pub fn render<S, R>(_self: Rc<Self>, child_signal: S, route_signal: R) -> Dom 
    where 
        S: Signal<Item=Option<Dom>> + 'static,
        R: Signal<Item=Route> + 'static,
    {

        elem!(templates::container(), {
            .with_data_id!("sidebar", {
                .class_signal("hidden", _self.sidebar_closed.signal())
                .with_data_id!("close", {
                    .event(clone!(_self => move |evt:events::Click| {
                        _self.sidebar_closed.set(true);
                    }))
                })
                .with_data_id!("links", {
                    .children_signal_vec(Self::links_dom_signal(_self.clone(), route_signal))
                })
            })
            .with_data_id!("main", {
                .child_signal(child_signal)
            })
        })
    }
}
