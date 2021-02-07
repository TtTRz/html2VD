use htmlstream::HTMLTagState;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Node {
    tag: Option<String>,
    inner_html: Option<String>,
    attributes: Vec<HashMap<String, String>>,
    node_type: NodeType,
    children: Vec<NodeType>,
    parent: Option<Rc<RefCell<Node>>>,
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
        Self { ..Node::default() }
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
    parser: Parser,
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
        let root_node = Node::new();
        // root_node.set_root =
        for (pos, tag) in htmlstream::tag_iter(html) {
            match self.parser.state {
                ParserState::Closing => {}
                ParserState::Opening => {
                    let mut node = Node::new();
                    let node_type = match tag.state {
                        HTMLTagState::Text => NodeType::TextNode,
                        _ => NodeType::ElementNode,
                    };
                    node.init(Some(tag.name), node_type)
                }
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
            parser: Parser::new(),
        }
    }
}
