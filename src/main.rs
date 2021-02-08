mod virtual_dom;
use virtual_dom::{VirtualDom, VD};

fn main() {
    let html = "<div><span>hello</span></div>";
    let mut vd = VD::new();
    vd.parse_html(html);
    println!("{:?}", vd.root);
}

