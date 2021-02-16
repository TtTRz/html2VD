use erased_serde::serialize_trait_object;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
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
    fn add_child(&mut self, child: Rc<RefCell<Box<dyn Node>>>) {}

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

    fn add_child(&mut self, child: Rc<RefCell<Box<dyn Node>>>) {
        self.children.push(child);
    }

    fn get_node_type(&self) -> NodeType {
        self.node_type.clone()
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
    fn get_inner_html(&self) -> Option<&std::string::String> {
        self.inner_html.as_ref()
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
