use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TagAttr {
    name: String,
    value: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum NodeType {
    ElementNode,
    FragmentNode,
    TextNode,
    None,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Node {
    tag: Option<String>,
    inner_html: Option<String>,
    attrs: Vec<TagAttr>,
    node_type: NodeType,
    pub children: Vec<Rc<RefCell<Node>>>,
    // parent: Option<Weak<RefCell<Node>>>,
    // root: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new() -> Self {
        Node { ..Node::default() }
    }

    pub fn init(&mut self, tag: Option<String>, node_type: NodeType) {
        self.tag = tag;
        self.node_type = node_type;
    }

    pub fn add_attr(&mut self, name: String, value: String) {
        self.attrs.push(TagAttr { name, value });
    }

    pub fn set_inner_html(&mut self, inner_html: String) {
        self.inner_html = Some(inner_html);
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            tag: None,
            inner_html: None,
            attrs: vec![],
            node_type: NodeType::None,
            children: vec![],
            // parent: None,
            // root: None,
        }
    }
}
