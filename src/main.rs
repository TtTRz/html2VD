use regex::Regex;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::{borrow::Borrow, cell::RefCell};

mod utils;
use utils::html_filter::html_filter;

mod model;
use model::node::Node;
use model::vd::VD;

mod algo;
use algo::virtual_dom::VirtualDom;

fn main() {
    let html = r"
      <div>
        rom
      </div>
    ";
    let mut vd = VD::new();
    let target_html = html_filter(html.into());
    vd.parse_html(target_html.as_str());
    println!("{:?}", vd.get_vd());
}
