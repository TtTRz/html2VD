use htmlstream::HTMLTagState;
use std::cell::RefCell;
use std::rc::Rc;

use crate::model::node::{Node, NodeType};
use crate::model::vd::VD;

pub trait VirtualDom {
    fn parse_html(&mut self, html: &str);
    fn get_vd(&mut self) -> Option<Rc<RefCell<Node>>>;
}

impl VirtualDom for VD {
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
                    let parent_node = parent_node_stack.pop().unwrap();
                    {
                        let mut parent_node_mut = parent_node.borrow_mut();
                        parent_node_mut.children.push(Rc::clone(&node));
                    }
                    parent_node_stack.push(parent_node);
                    parent_node_stack.push(node);
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
                    parent_node.borrow_mut().children.push(node);
                    parent_node_stack.push(parent_node);
                }
            }
        }
    }
    fn get_vd(&mut self) -> Option<Rc<RefCell<Node>>> {
        let root = self.root.take();
        root.map(|r| Rc::clone(&r))
    }
}
