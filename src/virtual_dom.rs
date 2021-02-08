use htmlstream::HTMLTagState;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::rc::{Rc, Weak};
use std::{cell::RefCell, thread::current};

#[derive(Deserialize, Serialize, Debug)]
pub struct Node {
    tag: Option<String>,
    // inner_html: Option<String>,
    attrs: Vec<TagAttr>,
    node_type: NodeType,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Weak<RefCell<Node>>>,
    // root: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new() -> Self {
        Node { ..Node::default() }
    }
    fn init(&mut self, tag: Option<String>, node_type: NodeType) {
        self.tag = tag;
        self.node_type = node_type;
    }
    fn add_attr(&mut self, name: String, value: String) {
        self.attrs.push(TagAttr { name, value });
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            tag: None,
            // inner_html: None,
            attrs: vec![],
            node_type: NodeType::None,
            children: vec![],
            parent: None,
            // root: None,
        }
    }
}
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

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Parser {
//     current_node: Option<Rc<RefCell<Node>>>,
// }

// impl Default for Parser {
//     fn default() -> Self {
//         Self { current_node: None }
//     }
// }

// impl Parser {
//     fn new() -> Self {
//         Self { ..Self::default() }
//     }
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct VD {
    root: Option<Rc<RefCell<Node>>>,
}

pub trait VirtualDom {
    fn new() -> Self;
    fn parse_html(&mut self, html: &str);
    fn get_vd(&mut self) -> Option<Rc<RefCell<Node>>>;
}

impl VirtualDom for VD {
    fn new() -> Self {
        Self { ..Self::default() }
    }
    fn parse_html(&mut self, html: &str) {
        let mut root = Node::new();
        root.init(None, NodeType::FragmentNode);
        let root = Rc::new(RefCell::new(root));
        self.root = Some(Rc::clone(&root));
        let mut current_node = Rc::clone(&root);
        // parent = root
        let mut parent_node = Rc::clone(&root);
        for (_, tag) in htmlstream::tag_iter(html) {
            match tag.state {
                HTMLTagState::Closing => {
                    // parser_mut.current_node = current_node.parent.clone().unwrap().upgrade();
                    current_node = Rc::clone(&parent_node);
                }
                HTMLTagState::Opening => {
                    let node_type = NodeType::ElementNode;
                    let mut node = Node::new();
                    node.init(Some(tag.name), node_type);
                    // attr
                    for (_, attr) in htmlstream::attr_iter(&tag.attributes) {
                        node.add_attr(attr.name, attr.value);
                    }
                    // parent =
                    parent_node = Rc::clone(&current_node);
                    // node.parent = Some(Rc::downgrade(&(parser_mut.current_node.clone().unwrap())));
                    let node = Rc::new(RefCell::new(node));
                    {
                        let mut current_node_mut = current_node.borrow_mut();
                        current_node_mut.children.push(Rc::clone(&node));
                    }
                    current_node = Rc::clone(&node);
                }
                _ => {
                    let node_type = match tag.state {
                        HTMLTagState::Text => NodeType::TextNode,
                        _ => NodeType::ElementNode,
                    };
                    let mut node = Node::new();
                    for (_, attr) in htmlstream::attr_iter(&tag.attributes) {
                        node.add_attr(attr.name, attr.value);
                    }
                    node.init(Some(tag.name), node_type);
                    parent_node = Rc::clone(&current_node);
                    // node.parent = Some(Rc::downgrade(&(parser_mut.current_node.clone().unwrap())));
                    let node = Rc::new(RefCell::new(node));
                    let mut current_node = current_node.borrow_mut();
                    current_node.children.push(Rc::clone(&node));
                }
            }
        }
    }
    fn get_vd(&mut self) -> Option<Rc<RefCell<Node>>> {
        let root = self.root.take();
        let r = match root {
            Some(rm) => {
                self.root = Some(rm.clone());
                Some(rm)
            }
            None => None,
        };
        r
    }
}

impl Default for VD {
    fn default() -> Self {
        VD {
            // parser: Rc::new(RefCell::new(Parser::new())),
            root: None,
        }
    }
}
