use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

mod model;
use model::node::Node;
use model::vd::VD;

mod algo;
use algo::virtual_dom::VirtualDom;

mod utils;
use utils::html_filter::html_filter;

mod tests;

#[cfg(feature = "wee_alloc")]
#[global_allocator]

static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(text: &str);
    #[wasm_bindgen(js_namespace=console)]
    fn warn(text: &str);
    #[wasm_bindgen(js_namespace=console)]
    fn error(text: &str);
}

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct IJsNode {
    #[wasm_bindgen(skip)]
    pub root: Rc<RefCell<Node>>,
}

#[wasm_bindgen]
pub fn html_2_vd(html: &str) -> JsValue {
    let mut vd = VD::new();
    let target_html = html_filter(html.into());
    vd.parse_html(target_html.as_str());
    JsValue::from_serde(&vd.get_vd()).unwrap()
}
