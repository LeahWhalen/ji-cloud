use simple_html_template::{TemplateCache, html_map, hash_map};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use utils::settings::SETTINGS;
use std::fmt;

thread_local! {
    pub static TEMPLATES: Templates = Templates::new(); 
}
macro_rules! template_path {
    ($e:tt) => { 
        concat!("../../../../../../../.template_output/", $e)
    } 
}


const SIDEBAR:&'static str = "sidebar";
const HEADER:&'static str = "header";
const FOOTER:&'static str = "footer";
const MAIN:&'static str = "main";
const SIDEBAR_LAYOUT:&'static str = "sidebar-layout";
const SIDEBAR_LAYOUT_ITEM:&'static str = "sidebar-layout-item";
const SIDEBAR_IMAGES:&'static str = "sidebar-images";

pub fn sidebar() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(SIDEBAR))
}

pub fn header(title:&str, subtitle:&str) -> HtmlElement {
    //This is unsafe - because currently subtitle uses raw html, need to preserve it
    TEMPLATES.with(|t| t.cache.render_elem(HEADER, &hash_map!(
        "title" => title,
        "subtitle" => subtitle
    )).unwrap_throw())
}
pub fn footer() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(FOOTER))
}
pub fn main() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(MAIN))
}
pub fn sidebar_layout() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(SIDEBAR_LAYOUT))
}
pub fn sidebar_layout_item(label:&str, src: &str) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(SIDEBAR_LAYOUT_ITEM, &html_map!{
        "label" => label,
        "src" => src,
    }).unwrap_throw())
}

pub fn sidebar_images() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(SIDEBAR_IMAGES))
}


pub struct Templates {
    pub cache: TemplateCache<'static>
}

impl fmt::Debug for Templates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        f.debug_list()
            .entries(self.cache.templates.keys())
         .finish()
    }
}

impl Templates {
    pub fn new() -> Self {
        let cache = TemplateCache::new(&vec![
            (SIDEBAR, include_str!(
                template_path!("module/poster/edit/sidebar/sidebar.html")
            )),
            (HEADER, include_str!(
                template_path!("module/poster/edit/header.html")
            )),
            (FOOTER, include_str!(
                template_path!("module/poster/edit/footer.html")
            )),
            (MAIN, include_str!(
                template_path!("module/poster/edit/main.html")
            )),
            (SIDEBAR_LAYOUT, include_str!(
                template_path!("module/poster/edit/sidebar/layout.html")
            )),
            (SIDEBAR_LAYOUT_ITEM, include_str!(
                template_path!("module/poster/edit/sidebar/layout-item.html")
            )),
            (SIDEBAR_IMAGES, include_str!(
                template_path!("module/poster/edit/sidebar/images.html")
            )),
        ]);

        Self { cache }
    }
}