mod virtual_dom;
use virtual_dom::{VirtualDom, VD};

fn main() {
    let html = "<div class='name'></div>";
    let mut vd = VD::new();
    vd.parse_html(html);
    println!("{:?}", vd.root);
}
