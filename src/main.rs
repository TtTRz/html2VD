mod virtual_dom;
use virtual_dom::{VirtualDom, VD};

fn main() {
    let html = "<div class='name'><span></span><span></span></div>";
    let mut vd = VD::new();
    vd.parse_html(html);
    println!("{:?}", vd.get_vd());
}
