// use regex::Regex;
// use serde::{Deserialize, Serialize};
// use std::cell::RefCell;
// use std::rc::Rc;

// mod model;
// use model::node::Node;
// use model::vd::VD;

// mod algo;
// use algo::virtual_dom::VirtualDom;

// fn main() {
//     let html = r"
//       <div>
//         rom
//       </div>
//     ";
//     let mut vd = VD::new();
//     let rg = Regex::new(r"[\n]+[\s]*").unwrap();
//     let h = html.trim();
//     let a = rg.replace_all(h, "");
//     println!("{}", a);
//     vd.parse_html(html);
// }
