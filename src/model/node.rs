use erased_serde::serialize_trait_object;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct TagAttr {
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum NodeType {
    ElementNode,
    FragmentNode,
    TextNode,
    None,
}
pub trait Node: erased_serde::Serialize {
    fn add_attr(&mut self, _name: String, _value: String) {}
    fn set_inner_html(&mut self, _inner_html: String) {}
    fn add_child(&mut self, _child: Rc<RefCell<Box<dyn Node>>>) {}

    fn get_node_type(&self) -> NodeType;
    fn get_inner_html(&self) -> Option<&std::string::String>;
    fn get_tag(&self) -> Option<&std::string::String> {
        None
    }
    fn get_attrs(&self) -> Vec<TagAttr> {
        vec![]
    }
    fn get_children(&self) -> Vec<Rc<RefCell<Box<dyn Node>>>> {
        vec![]
    }
}

serialize_trait_object!(Node);

use core::fmt::Debug;

impl Debug for dyn Node {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:?} {{{:?}}}",
            self.get_node_type(),
            format_args!(
                "tag: {:?}, node_type: {:?}, attrs: {:?}, inner_html: {:?}, children: {:?}",
                self.get_tag(),
                self.get_node_type(),
                self.get_attrs(),
                self.get_inner_html(),
                self.get_children()
            )
        )
    }
}
