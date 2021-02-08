use htmlstream::HTMLTagState;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
pub struct Node {
    tag: Option<String>,
    inner_html: Option<String>,
    attributes: Vec<HashMap<String, String>>,
    node_type: NodeType,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Weak<RefCell<Node>>>,
    root: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug, Clone)]
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
    // TODO inner_html attrib children
    fn init(&mut self, tag: Option<String>, node_type: NodeType) {
        self.tag = tag;
        // self.inner_html = inner_html;
        self.node_type = node_type;
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            tag: None,
            inner_html: None,
            attributes: vec![],
            node_type: NodeType::None,
            children: vec![],
            parent: None,
            root: None,
        }
    }
}
#[derive(Debug)]
pub struct Parser {
    state: ParserState,
    current_node: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
pub enum ParserState {
    Closing,
    Opening,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            state: ParserState::Closing,
            current_node: None,
        }
    }
}

impl Parser {
    fn new() -> Self {
        Self { ..Self::default() }
    }
}

#[derive(Debug)]
pub struct VD {
    state: bool,
    parser: Rc<RefCell<Parser>>,
    pub root: Option<Rc<RefCell<Node>>>,
}

pub trait VirtualDom {
    fn new() -> Self;
    fn parse_html(&mut self, html: &str);
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
        // root_node.set_root =
        parser_mut.state = ParserState::Opening;
        for (pos, tag) in htmlstream::tag_iter(html) {
            let current_node = parser_mut.current_node.clone().unwrap();
            let mut current_node = current_node.borrow_mut();
            // let mut current_node = parser_mut.current_node.clone().unwrap().borrow_mut();
            match parser_mut.state {
                ParserState::Closing => {}
                ParserState::Opening => match tag.state {
                    HTMLTagState::Closing => {
                        parser_mut.current_node = current_node.parent.clone().unwrap().upgrade();
                    }
                    HTMLTagState::Opening => {
                        let node_type = NodeType::ElementNode;
                        let mut node = Node::new();
                        node.init(Some(tag.name), node_type);
                        node.parent =
                            Some(Rc::downgrade(&(parser_mut.current_node.clone().unwrap())));
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
                        node.init(Some(tag.name), node_type);
                        node.parent =
                            Some(Rc::downgrade(&(parser_mut.current_node.clone().unwrap())));
                        let node = Rc::new(RefCell::new(node));
                        current_node.children.push(node.clone());
                        parser_mut.state = ParserState::Closing;
                    }
                },
            }
            // tag.state
            // for (pos, attr) in htmlstream::attr_iter(&tag.attributes) {
            //         log(&format!("            {:?} {:?}", pos, attr));
            // }
        }
    }
}

impl Default for VD {
    fn default() -> Self {
        Self {
            state: false,
            parser: Rc::new(RefCell::new(Parser::new())),
            root: None,
        }
    }
}
