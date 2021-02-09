use virtual_dom::VirtualDom;

use crate::algo::virtual_dom;
use crate::model::vd::VD;

#[test]
fn test_vd_new() {
    let vd = VD::new();
    println!("{:?}", vd);
}

#[test]
fn test_parse_html() {
    let html = "<div>hello</div>";
    let mut vd = VD::new();
    vd.parse_html(html);
    println!("{:?}", vd.get_vd());
}
