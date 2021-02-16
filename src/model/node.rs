use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TagAttr {
    name: String,
    value: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum NodeType {
    ElementNode,
    FragmentNode,
    TextNode,
    None,
}

pub trait Node: erased_serde::Serialize {
    fn add_attr(&mut self, name: String, value: String) {}
    fn set_inner_html(&mut self, inner_html: String) {}
    fn get_node_type(&self) -> NodeType;
    fn add_child(&mut self, child: Rc<RefCell<Box<dyn Node>>>) {}
}

serialize_trait_object!(Node);

use core::fmt::Debug;

// impl Debug for Node {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "{:?}: {:?}", self.get_node_type(), self)
//     }
// }

#[derive(Serialize)]
pub struct ElementNode {
    tag: Option<String>,
    inner_html: Option<String>,
    attrs: Vec<TagAttr>,
    node_type: NodeType,
    pub children: Vec<Rc<RefCell<Box<dyn Node>>>>,
}

impl ElementNode {
    pub fn new(tag: Option<String>) -> Self {
        ElementNode {
            tag,
            ..ElementNode::default()
        }
    }
}

impl Node for ElementNode {
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
}

impl Default for ElementNode {
    fn default() -> Self {
        ElementNode {
            tag: None,
            inner_html: None,
            attrs: vec![],
            node_type: NodeType::ElementNode,
            children: vec![],
        }
    }
}

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

#[derive(Debug, Serialize)]
pub struct TextNode {
    inner_html: Option<String>,
    node_type: NodeType,
}

impl TextNode {
    pub fn new() -> Self {
        TextNode {
            ..TextNode::default()
        }
    }
}

impl Node for TextNode {
    fn set_inner_html(&mut self, inner_html: String) {
        self.inner_html = Some(inner_html);
    }

    fn get_node_type(&self) -> NodeType {
        self.node_type.clone()
    }
}

impl Default for TextNode {
    fn default() -> Self {
        TextNode {
            inner_html: None,
            node_type: NodeType::TextNode,
        }
    }
}
