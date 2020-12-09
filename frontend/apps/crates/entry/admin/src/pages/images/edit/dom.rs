use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlTextAreaElement, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods};
use dominator_helpers::{elem, with_data_id, futures::{spawn_future, AsyncLoader}};
use crate::templates;
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use discard::DiscardOnDrop;
use utils::{
    routes::{Route, AdminRoute},
};
use components::image::data::*;
use shared::{
    error::image::UpdateError,
    domain::{
        user::UserProfile,
        category::Category,
        image::SearchQuery,
    },
    media::{MediaLibraryKind, MediaVariant}
};
use super::actions::{self, Init, Id, EditCategory, EditCategoryMode};
use std::collections::{HashSet, HashMap};
use gloo::timers::callback::Timeout;


pub struct ImageEdit {
    image: Mutable<SimpleImage>,
    error_message: Mutable<Option<String>>,
    publish_message: Mutable<Option<String>>,
    refs:RefCell<Option<ImageEditRefs>>,
    styles: RefCell<HashSet<Id>>,
    age_ranges: RefCell<HashSet<Id>>,
    affiliations: RefCell<HashSet<Id>>,
    init: Mutable<Option<Init>>,
    section: Mutable<Section>,
    save_loader: AsyncLoader,
    publish_loader: AsyncLoader,
    category_expansions: RefCell<HashMap<Id, Mutable<bool>>>,
    selected_categories: Mutable<HashSet<Id>>,
    from_search: Option<SearchQuery>,
    redirect_timeout: RefCell<Option<Timeout>>,
}

#[derive(Clone, Copy, Debug)]
enum Section {
    Meta,
    Categories,
    Overview
}

struct SaveInfo {
    pub id: String,
    pub is_premium: bool,
    pub name:String,
    pub description: String,
    pub styles: Vec<Id>,
    pub age_ranges: Vec<Id>,
    pub affiliations: Vec<Id>,
    pub categories: Vec<Id>,
}

fn make_img(id:&str) -> SimpleImage {
    (id, MediaLibraryKind::Global).into()
}

impl ImageEdit{
    pub fn new(id:String, from_search: Option<SearchQuery>) -> Rc<Self> {
        let start_section = if from_search.is_some() { Section::Overview } else { Section::Meta };

        let _self = Rc::new(Self { 
            //utils
            image: Mutable::new(make_img(&id)),
            //UI
            error_message: Mutable::new(None),
            publish_message: Mutable::new(None),
            refs: RefCell::new(None),
            section: Mutable::new(start_section),
            category_expansions: RefCell::new(HashMap::new()),
            //Load/Save
            init: Mutable::new(None),
            publish_loader: AsyncLoader::new(),
            save_loader: AsyncLoader::new(),
            //Data
            styles: RefCell::new(HashSet::new()),
            age_ranges: RefCell::new(HashSet::new()),
            affiliations: RefCell::new(HashSet::new()),
            selected_categories: Mutable::new(HashSet::new()),
            from_search,
            redirect_timeout: RefCell::new(None),
        });

        let _self_clone = _self.clone();

        spawn_local(async move {
            match actions::Init::load(&id).await {
                Ok(init) => {
                    //These maps have to be set on init
                    //Even though the elements themselves aren't like "controlled components"
                    //It is not required for things which have no indirection, like
                    //name, description, and is_premium
                    let mut styles = _self.styles.borrow_mut();
                    for (id, _, _) in init.styles.iter().filter(|(_, _, contains)| *contains) {
                        styles.insert(id.to_string());
                    }

                    let mut age_ranges = _self.age_ranges.borrow_mut();
                    for (id, _, _) in init.age_ranges.iter().filter(|(_, _, contains)| *contains) {
                        age_ranges.insert(id.to_string());
                    }

                    let mut affiliations = _self.affiliations.borrow_mut();
                    for (id, _, _) in init.affiliations.iter().filter(|(_, _, contains)| *contains) {
                        affiliations.insert(id.to_string());
                    }

                    let mut selected_categories = _self.selected_categories.lock_mut();
                    for id in init.selected_categories.iter() {
                        selected_categories.insert(id.to_string());
                    }

                    fn set_expansions(curr:&Vec<EditCategory>, expansions: &mut HashMap<Id, Mutable<bool>>) {
                        for cat in curr.iter() {
                            expansions.insert(cat.id.clone(), Mutable::new(false));
                            if !cat.children.is_empty() {
                                set_expansions(&cat.children, expansions);
                            }
                        }
                    };
                    set_expansions(&init.categories, &mut _self.category_expansions.borrow_mut());

                    _self.init.set(Some(init)); 
                },
                Err(_) => { log::error!("GOT ERROR!!"); }
            }
        });
        _self_clone
    }


    fn get_save_info(_self: Rc<Self>) -> Option<SaveInfo> {
        _self.refs
            .borrow()
            .as_ref()
            .map(|refs| {
                let id = _self.image.get_cloned().id_string();
                let is_premium = refs.is_premium();
                let name = refs.name();
                let description = refs.description();

                let styles:Vec<String> = 
                    _self.styles
                        .borrow()
                        .iter()
                        .map(|s| s.to_string())
                        .collect();

                let age_ranges:Vec<String> =
                    _self.age_ranges
                        .borrow()
                        .iter()
                        .map(|s| s.to_string())
                        .collect();

                let affiliations:Vec<String> =
                    _self.affiliations
                        .borrow()
                        .iter()
                        .map(|s| s.to_string())
                        .collect();

                let categories:Vec<String> =
                    _self.selected_categories
                        .lock_ref()
                        .iter()
                        .map(|s| s.to_string())
                        .collect();

                SaveInfo {
                    id,
                    is_premium,
                    name,
                    description,
                    styles,
                    age_ranges,
                    affiliations,
                    categories
                }
            })
    }
    //could be made more efficient by just saving each part as needed
    //but not a big deal (and then each would need its own AsyncLoader)
    fn save(_self: Rc<Self>) {
        let _self_clone = _self.clone();
        if let Some(info) = Self::get_save_info(_self.clone()) {
            let SaveInfo {
                id,
                is_premium,
                name,
                description,
                styles,
                age_ranges,
                affiliations,
                categories
            } = info;

            _self.save_loader.load(async move {
                _self_clone.error_message.set(None);

                if let Err(err) = actions::save(
                    id, 
                    is_premium, 
                    name, description, 
                    styles, 
                    age_ranges, 
                    affiliations,
                    categories
                ).await {
                    let msg = match err {
                        UpdateError::NonExistantMetadata{id, kind} => {
                            format!("missing metadata!")
                        },
                        _ => {
                            format!("internal error!")
                        }
                    };

                    _self_clone.error_message.set(Some(msg));
                }
            });

        }
    }

    fn publish(_self: Rc<Self>) {
        let _self_clone = _self.clone();

        let id = _self.image.get_cloned().id_string();
        _self_clone.publish_loader.load(async move {
            _self.error_message.set(None);

            if let Err(err) = actions::publish(id).await {
                let msg = match err {
                    UpdateError::NonExistantMetadata{id, kind} => {
                        format!("missing metadata!")
                    },
                    _ => {
                        format!("internal error!")
                    }
                };

                _self.error_message.set(Some(msg));
            } else {
                _self.publish_message.set(Some("Done!".to_string()));
                *_self.redirect_timeout.borrow_mut() = Some(
                    Timeout::new(3_000, clone!(_self => move || {
                        _self.publish_message.set(None);
                        let route = Route::Admin(AdminRoute::ImageSearch(_self.from_search.clone()));
                        let route = String::from(route);
                        dominator::routing::go_to_url(&route);
                    }))
                );
            }
        });
    }
    
    pub fn render(_self: Rc<Self>) -> Dom {
        html!("div", {
            .child_signal(_self.init.signal_cloned().map(clone!(_self => move |init| {
                init.map(|init:Init| {
                    elem!(templates::image_edit(), { 
                        .with_data_id!("back-to-search", {
                            .apply_if(_self.from_search.is_none(), |dom| {
                                dom.class("hidden")
                            })
                            .event(clone!(_self => move |evt:events::Click| {
                                let route = Route::Admin(AdminRoute::ImageSearch(_self.from_search.clone()));
                                let route = String::from(route);
                                dominator::routing::go_to_url(&route);
                            }))
                        })
                        .with_data_id!("img", {
                            .property_signal("src", _self.image.signal_cloned().map(|image| {
                                image.thumbnail_src()
                            }))
                        })
                        .with_data_id!("delete", {
                            .event(clone!(_self => move |_evt:events::Click| {
                                spawn_local(clone!(_self => async move {
                                    match actions::delete(_self.image.get_cloned().id_string()).await {
                                        Err(_) => {
                                            _self.error_message.set(Some("Couldn't delete image".to_string()))
                                        },
                                        Ok(_) => {
                                            let route:String = Route::Admin(AdminRoute::ImageSearch( _self.from_search.clone())).into();
                                            dominator::routing::go_to_url(&route);
                                        }
                                    }
                                }));
                            }))
                        })
                        .with_data_id!("error", {
                            .class_signal("hidden", _self.error_message.signal_ref(|err| err.is_none()))
                            .with_data_id!("message", {
                                .text_signal(_self.error_message.signal_ref(|msg| {
                                    match msg {
                                        None => "".to_string(),
                                        Some(msg) => msg.to_string()
                                    }
                                }))
                            })
                        })
                        .with_data_id!("published", {
                            .class_signal("hidden", _self.publish_message.signal_ref(|msg| msg.is_none()))
                            .with_data_id!("publish-img", {
                                .property_signal("src", _self.image.signal_cloned().map(|image| {
                                    image.thumbnail_src()
                                }))
                            })
                        })
                        .with_data_id!("next", {
                            .event(clone!(_self => move |_evt:events::Click| {
                                let section = { *_self.section.lock_ref() };
                                match section {
                                    Section::Meta => {
                                        _self.section.set(Section::Categories);
                                    },
                                    Section::Categories => {
                                        _self.section.set(Section::Overview);
                                    },
                                    Section::Overview => {
                                        Self::publish(_self.clone());
                                    }
                                }
                            }))
                            .text_signal(_self.section.signal().map(|section| {
                                match section {
                                    Section::Meta | Section::Categories => "Next".to_string(),
                                    Section::Overview => "Publish".to_string(),
                                }
                            }))
                        })
                        .with_data_id!("premium", {
                            .property("checked",  *&init.is_premium)
                            .event(clone!(_self => move |_evt:events::Change| {
                                Self::save(_self.clone());
                            }))
                        })
                        .with_data_id!("name", {
                            .property("value", &init.name )
                            .event(clone!(_self => move |_evt:events::Input| {
                                Self::save(_self.clone());
                            }))
                        })
                        .with_data_id!("description", {
                            .property("value", &init.description)
                            .event(clone!(_self => move |_evt:events::Input| {
                                Self::save(_self.clone());
                            }))
                        })

                        .with_data_id!("replace-btn", {
                            .event(clone!(_self => move |_evt:events::Click| {
                                if let Some(refs) = _self.refs.borrow().as_ref() {
                                    refs.file_input.click();
                                }
                            }))
                        })
                        .with_data_id!("file", {
                            .event(clone!(_self => move |_evt:events::Change| {
                                let file =
                                    _self.refs
                                        .borrow()
                                        .as_ref()
                                        .and_then(|refs| refs.file());

                                if let Some(file) = file {
                                    let _self = _self.clone();
                                    spawn_local(async move {
                                        match actions::replace_url(&_self.image.get_cloned().id_str(), file).await {
                                            Err(_) => {
                                                _self.error_message.set(Some("Couldn't upload image".to_string()))
                                            },
                                            Ok(_) => {
                                                //to trigger the src change
                                                _self.image.replace_with(|img| img.clone());
                                            }
                                        }
                                    });
                                }
                            }))

                        })
                        .after_inserted(clone!(_self => move |elem| {
                            *_self.refs.borrow_mut() = Some(ImageEditRefs::new(elem));
                        }))
                        .with_data_id!("right-area", {
                            .child_signal(_self.section.signal_ref(clone!(_self => move |section| {
                                Some(match section {
                                    Section::Meta => Self::render_section_meta(_self.clone(), &init),
                                    Section::Categories => Self::render_section_categories(_self.clone(), &init),
                                    Section::Overview => Self::render_section_overview(_self.clone(), &init),
                                })
                            })))
                        })

                    })
                })
            })))
        })
    }
    
    fn render_section_overview(_self: Rc<Self>, init:&Init) -> Dom {

        if let Some(info) = Self::get_save_info(_self.clone()) {

            let styles:Vec<String> = init.styles.iter().filter_map(|(id, label, _)| {
                if info.styles.contains(id) {
                    Some(label.clone())
                } else {
                    None
                }
            }).collect();

            let age_ranges:Vec<String> = init.age_ranges.iter().filter_map(|(id, label, _)| {
                if info.age_ranges.contains(id) {
                    Some(label.clone())
                } else {
                    None
                }
            }).collect();

            let affiliations:Vec<String> = init.affiliations.iter().filter_map(|(id, label, _)| {
                if info.affiliations.contains(id) {
                    Some(label.clone())
                } else {
                    None
                }
            }).collect();

            let SaveInfo {
                id,
                is_premium,
                name,
                description,
                categories,
                ..
            } = info;

            elem!(templates::image_edit_overview(&name, &description), {
                .with_data_id!("styles", {
                    .children(styles.into_iter().map(|label| html!("div", {.text(&label)})))
                })
                .with_data_id!("age_ranges", {
                    .children(age_ranges.into_iter().map(|label| html!("div", {.text(&label)})))
                })
                .with_data_id!("affiliations", {
                    .children(affiliations.into_iter().map(|label| html!("div", {.text(&label)})))
                })
                .with_data_id!("category-summaries", {
                    .children(init.categories.iter().map(clone!(_self => move |cat| {
                        Self::render_category_summary(_self.clone(), cat.clone())
                    })))
                })
                .with_data_id!("edit-meta", {
                    .event(clone!(_self => move |evt:events::Click| {
                        _self.section.set(Section::Meta);
                    }))
                })
                .with_data_id!("edit-categories", {
                    .event(clone!(_self => move |evt:events::Click| {
                        _self.section.set(Section::Categories);
                    }))
                })
            
            })
        } else {
            html!("div")
        }
    }

    fn render_section_meta(_self: Rc<Self>, init:&Init) -> Dom {
        elem!(templates::image_edit_meta(), {
            .with_data_id!("styles", {
                .children(
                    init.styles
                        .iter()
                        .map(|(id, label, contains)| {
                            elem!(templates::checkbox(&id, &label), {
                                .with_data_id!(id, {
                                    .property("checked", *contains)
                                    .event(clone!(_self, id => move |evt:events::Change| {
                                        if let Some(checked) = evt.checked() {
                                            {
                                                let mut styles = _self.styles.borrow_mut();
                                                if checked {
                                                    styles.insert(id.to_string());
                                                } else {
                                                    styles.remove(&id);
                                                }
                                            }
                                            Self::save(_self.clone());
                                        }
                                    }))
                                })
                            })
                        })
                )
            })

            .with_data_id!("age_ranges", {
                .children(
                    init.age_ranges
                        .iter()
                        .map(|(id, label, contains)| {
                            elem!(templates::checkbox(&id, &label), {
                                .with_data_id!(id, {
                                    .property("checked", *contains)
                                    .event(clone!(_self, id => move |evt:events::Change| {
                                        if let Some(checked) = evt.checked() {
                                            {
                                                let mut age_ranges = _self.age_ranges.borrow_mut();
                                                if checked {
                                                    age_ranges.insert(id.to_string());
                                                } else {
                                                    age_ranges.remove(&id);
                                                }
                                            }
                                            Self::save(_self.clone());
                                        }
                                    }))
                                })
                            })
                        })
                )
            })

            .with_data_id!("affiliations", {
                .children(
                    init.affiliations
                        .iter()
                        .map(|(id, label, contains)| {
                            elem!(templates::checkbox(&id, &label), {
                                .with_data_id!(id, {
                                    .property("checked", *contains)
                                    .event(clone!(_self, id => move |evt:events::Change| {
                                        if let Some(checked) = evt.checked() {
                                            {
                                                let mut affiliations = _self.affiliations.borrow_mut();
                                                if checked {
                                                    affiliations.insert(id.to_string());
                                                } else {
                                                    affiliations.remove(&id);
                                                }
                                            }
                                            Self::save(_self.clone());
                                        }
                                    }))
                                })
                            })
                        })
                )
            })

        })
    }
    fn render_section_categories(_self: Rc<Self>, init:&Init) -> Dom {
        elem!(templates::image_edit_categories(), {
            .with_data_id!("categories-expand", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.category_expansions
                        .borrow()
                        .values()
                        .for_each(|x| x.set(true));
                }))
            })
            .with_data_id!("categories-collapse", {
                .event(clone!(_self => move |evt:events::Click| {
                    _self.category_expansions
                        .borrow()
                        .values()
                        .for_each(|x| x.set(false));
                }))
            })
            .with_data_id!("select-list", {
                .children(init.categories.iter().map(clone!(_self => move |cat| {
                    Self::render_category_select(_self.clone(), cat.clone())
                })))
            })
            .with_data_id!("summary-list", {
                .children(init.categories.iter().map(clone!(_self => move |cat| {
                    Self::render_category_summary(_self.clone(), cat.clone())
                })))
            })
        })
    }

    fn render_category_summary(_self: Rc<Self>, cat: EditCategory) -> Dom {
        let id = cat.id.clone();

        let elem = match cat.mode {
            EditCategoryMode::Parent => templates::image_edit_category_summary_parent(&cat.name),
            EditCategoryMode::Child => templates::image_edit_category_summary_child(&cat.name)
        };

        elem!(elem, {
            .with_data_id!("children", {
                .children(cat.children.iter().map(clone!(_self => move |cat| {
                    Self::render_category_summary(_self.clone(), cat.clone())
                })))
            })
            .class_signal("hidden", _self.selected_categories.signal_ref(move |selected| {
                !cat.contains_leaf_set(selected)
            }))
        })
    }

    fn render_category_select(_self: Rc<Self>, cat: EditCategory) -> Dom {
        let id = cat.id.clone();

        let elem = match cat.mode {
            EditCategoryMode::Parent => {
                if cat.is_end {
                    templates::image_edit_category_parent_end(&cat.name)
                } else {
                    templates::image_edit_category_parent(&cat.name)
                }
            }
            EditCategoryMode::Child => {
                if cat.is_end {
                    templates::image_edit_category_child_end(&cat.name)
                } else {
                    templates::image_edit_category_child(&cat.name)
                }
            }
        };

        if cat.is_end {
            elem!(elem, {
                .with_data_id!("checkbox", {
                    .property("checked", cat.assigned)
                    .event(clone!(_self, id => move |evt:events::Change| {
                        if let Some(checked) = evt.checked() {
                            {
                                let mut selected = _self.selected_categories.lock_mut();
                                if checked {
                                    selected.insert(id.to_string());
                                } else {
                                    selected.remove(&id);
                                }
                            }
                            Self::save(_self.clone());
                        }
                    }))
                })
            })
        } else {
            elem!(elem, {
                .with_data_id!("children", {
                    .children_signal_vec(
                        _self.category_expansions.borrow().get(&cat.id).unwrap_throw().signal().map(clone!(_self => move |expanded| {
                                if expanded {
                                    let children:Vec<Dom> = cat.children.iter().map(clone!(_self => move |cat| {
                                        Self::render_category_select(_self.clone(), cat.clone())
                                    })).collect();
                                    children
                                } else {
                                    Vec::new()
                                }
                        }))
                        .to_signal_vec()
                    )
                })
                .with_data_id!("arrow", {
                    .class_signal(
                        ["transform", "rotate-90", "-m-1"],
                        _self.category_expansions.borrow().get(&id).unwrap_throw().signal()
                    )
                    .event(clone!(_self, id => move |evt:events::Click| {
                        _self.category_expansions.borrow()
                            .get(&id)
                            .unwrap_throw()
                            .replace_with(|x| !*x);
                    }))
                })
            })
        }
    }

}

fn is_checked(possible:&[(Id, String)], item_list:&[Id], id:&Id) -> bool {
    true
}
struct ImageEditRefs {
    is_premium_elem: HtmlInputElement,
    name_elem: HtmlInputElement,
    description_elem: HtmlTextAreaElement,
    file_input:HtmlInputElement
}

impl ImageEditRefs {

    fn new(elem:HtmlElement) -> Self {
        Self {
            is_premium_elem: elem.try_select(&data_id("premium")).expect_throw("can't get premium element"),
            name_elem: elem.try_select(&data_id("name")).expect_throw("can't get name element"),
            description_elem: elem.try_select(&data_id("description")).expect_throw("can't get description element"),
            file_input: elem.try_select(&data_id("file")).expect_throw("can't get file element"),
        }
    }

    pub fn is_premium(&self) -> bool {
        self.is_premium_elem.checked()
    }

    pub fn name(&self) -> String {
        self.name_elem.value()
    }

    pub fn description(&self) -> String {
        self.description_elem.value()
    }

    pub fn file(&self) -> Option<web_sys::File> {
        self.file_input.files()
            .and_then(|files| files.get(0))
    }

    /*
    pub fn styles(&self) -> String {
        self.description_elem.value()
    }
    */
}

