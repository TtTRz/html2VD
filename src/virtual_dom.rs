use htmlstream::HTMLTagState;

use serde::{Deserialize, Serialize};

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Deserialize, Serialize, Debug)]
pub struct Node {
    tag: Option<String>,
    inner_html: Option<String>,
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
    fn set_inner_html(&mut self, inner_html: String) {
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
        VD { ..Self::default() }
    }
    fn parse_html(&mut self, html: &str) {
        let mut root = Node::new();
        root.init(None, NodeType::FragmentNode);
        let root = Rc::new(RefCell::new(root));
        self.root = Some(Rc::clone(&root));
        // parent = root
        let mut parent_node_stack = vec![];
        parent_node_stack.push(Rc::clone(&root));
        for (_, tag) in htmlstream::tag_iter(html) {
            match tag.state {
                HTMLTagState::Opening => {
                    let node_type = NodeType::ElementNode;
                    let mut node = Node::new();
                    let tag_name = match tag.name.as_str() {
                        "" => None,
                        _ => Some(tag.name),
                    };
                    node.init(tag_name, node_type);
                    // attr
                    for (_, attr) in htmlstream::attr_iter(&tag.attributes) {
                        node.add_attr(attr.name, attr.value);
                    }
                    let node = Rc::new(RefCell::new(node));
                    {
                        let parent_node = parent_node_stack.pop().unwrap();
                        {
                            let mut parent_node_mut = parent_node.borrow_mut();
                            parent_node_mut.children.push(Rc::clone(&node));
                        }
                        parent_node_stack.push(parent_node);
                    }
                    parent_node_stack.push(Rc::clone(&node));
                }
                HTMLTagState::Closing => {
                    parent_node_stack.pop();
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
                    // TextNode set innerHtml
                    if node_type == NodeType::TextNode {
                        node.set_inner_html(tag.html)
                    }
                    let tag_name = match tag.name.as_str() {
                        "" => None,
                        _ => Some(tag.name),
                    };
                    node.init(tag_name, node_type);
                    let node = Rc::new(RefCell::new(node));
                    let parent_node = parent_node_stack.pop().unwrap();
                    {
                        let mut parent_node_mut = parent_node.borrow_mut();
                        parent_node_mut.children.push(Rc::clone(&node));
                    }
                    parent_node_stack.push(parent_node);
                }
            }
        }
    }
    fn get_vd(&mut self) -> Option<Rc<RefCell<Node>>> {
        let root = self.root.take();
        let r = match root {
            Some(rm) => {
                self.root = Some(Rc::clone(&rm));
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
