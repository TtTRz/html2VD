use super::node::{Node, NodeType, TagAttr};
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Serialize)]
pub struct FragmentNode {
    tag: Option<String>,
    inner_html: Option<String>,
    attrs: Vec<TagAttr>,
    node_type: NodeType,
    pub children: Vec<Rc<RefCell<Box<dyn Node>>>>,
}

impl FragmentNode {
    pub fn new() -> Self {
        FragmentNode {
            ..FragmentNode::default()
        }
    }
}

impl Node for FragmentNode {
    fn add_attr(&mut self, name: String, value: String) {
        self.attrs.push(TagAttr { name, value });
    }

    fn set_inner_html(&mut self, inner_html: String) {
        self.inner_html = Some(inner_html);
    }

    fn get_node_type(&self) -> NodeType {
        self.node_type.clone()
    }

    fn add_child(&mut self, child: Rc<RefCell<Box<dyn Node>>>) {
        self.children.push(child);
    }

    fn get_inner_html(&self) -> Option<&std::string::String> {
        self.inner_html.as_ref()
    }
    fn get_tag(&self) -> Option<&std::string::String> {
        self.tag.as_ref()
    }
    fn get_attrs(&self) -> Vec<TagAttr> {
        self.attrs.clone()
    }

    fn get_children(&self) -> Vec<Rc<RefCell<Box<dyn Node>>>> {
        self.children.clone()
    }
}

impl Default for FragmentNode {
    fn default() -> Self {
        FragmentNode {
            tag: None,
            inner_html: None,
            attrs: vec![],
            node_type: NodeType::FragmentNode,
            children: vec![],
        }
    }
}
