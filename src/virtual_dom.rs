use htmlstream::HTMLTagState;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
    tag: Option<String>,
    // inner_html: Option<String>,
    attributes: Vec<TagAttr>,
    node_type: NodeType,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Weak<RefCell<Node>>>,
    // root: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
pub struct TagAttr {
    name: String,
    value: String,
}

#[derive(Debug)]
pub enum NodeType {
    ElementNode,
    FragmentNode,
    TextNode,
    None,
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
        self.attributes.push(TagAttr { name, value });
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            tag: None,
            // inner_html: None,
            attributes: vec![],
            node_type: NodeType::None,
            children: vec![],
            parent: None,
            // root: None,
        }
    }
}
#[derive(Debug)]
pub struct Parser {
    current_node: Option<Rc<RefCell<Node>>>,
}

impl Default for Parser {
    fn default() -> Self {
        Self { current_node: None }
    }
}

impl Parser {
    fn new() -> Self {
        Self { ..Self::default() }
    }
}

#[derive(Debug)]
pub struct VD {
    parser: Rc<RefCell<Parser>>,
    root: Option<Rc<RefCell<Node>>>,
}

pub trait VirtualDom {
    fn new() -> Self;
    fn parse_html(&mut self, html: &str);
    fn get_vd(&self) -> Option<Rc<RefCell<Node>>>;
}

impl VirtualDom for VD {
    fn new() -> Self {
        Self { ..Self::default() }
    }
    fn parse_html(&mut self, html: &str) {
        let mut root_node = Node::new();
        root_node.init(None, NodeType::FragmentNode);
        let root_node = Rc::new(RefCell::new(root_node));
        self.root = Some(root_node.clone());
        let mut parser_mut = self.parser.borrow_mut();
        parser_mut.current_node = Some(root_node.clone());
        for (_, tag) in htmlstream::tag_iter(html) {
            let current_node = parser_mut.current_node.clone().unwrap();
            let mut current_node = current_node.borrow_mut();
            match tag.state {
                HTMLTagState::Closing => {
                    parser_mut.current_node = current_node.parent.clone().unwrap().upgrade();
                }
                HTMLTagState::Opening => {
                    let node_type = NodeType::ElementNode;
                    let mut node = Node::new();
                    node.init(Some(tag.name), node_type);
                    // attr
                    for (_, attr) in htmlstream::attr_iter(&tag.attributes) {
                        node.add_attr(attr.name, attr.value);
                    }
                    node.parent = Some(Rc::downgrade(&(parser_mut.current_node.clone().unwrap())));
                    let node = Rc::new(RefCell::new(node));
                    current_node.children.push(node.clone());
                    parser_mut.current_node = Some(node.clone());
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
                    node.parent = Some(Rc::downgrade(&(parser_mut.current_node.clone().unwrap())));
                    let node = Rc::new(RefCell::new(node));
                    current_node.children.push(node.clone());
                }
            }
        }
    }
    fn get_vd(&self) -> Option<Rc<RefCell<Node>>> {
        self.root.clone()
    }
}

impl Default for VD {
    fn default() -> Self {
        Self {
            parser: Rc::new(RefCell::new(Parser::new())),
            root: None,
        }
    }
}
