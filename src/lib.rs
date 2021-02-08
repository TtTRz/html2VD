extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

use virtual_dom::{Node, VirtualDom, VD};

#[cfg(test)]
mod tests {
    mod virtual_dom;
}

mod virtual_dom;
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
pub fn html2VD(html: &str) -> JsValue {
    let mut vd = VD::new();
    vd.parse_html(html);
    JsValue::from_serde(&vd.get_vd()).unwrap()
}
