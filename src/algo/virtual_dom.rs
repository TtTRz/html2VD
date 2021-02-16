use htmlstream::HTMLTagState;
use std::cell::RefCell;
use std::rc::Rc;

use crate::model::element_node::ElementNode;
use crate::model::fragment_node::FragmentNode;
use crate::model::node::{Node, NodeType};
use crate::model::text_node::TextNode;

use crate::model::vd::VD;

pub trait VirtualDom {
    fn parse_html(&mut self, _html: &str) {}
    fn get_vd(&mut self) -> Option<Rc<RefCell<Box<dyn Node>>>>;
}

impl VirtualDom for VD {
    fn parse_html(&mut self, html: &str) {
        let root: Rc<RefCell<Box<dyn Node>>> = Rc::new(RefCell::new(Box::new(FragmentNode::new())));
        self.root = Some(Rc::clone(&root) as _);
        // parent = root
        let mut parent_node_stack: Vec<Rc<RefCell<Box<dyn Node>>>> = vec![];
        parent_node_stack.push(Rc::clone(&root) as _);
        for (_, tag) in htmlstream::tag_iter(html) {
            match tag.state {
                HTMLTagState::Opening => {
                    let tag_name = match tag.name.as_str() {
                        "" => None, // TODO should be FragmentNode
                        _ => Some(tag.name),
                    };
                    let node: Rc<RefCell<Box<dyn Node>>> =
                        Rc::new(RefCell::new(Box::new(ElementNode::new(tag_name))));
                    // attr
                    for (_, attr) in htmlstream::attr_iter(&tag.attributes) {
                        node.borrow_mut().add_attr(attr.name, attr.value);
                    }
                    let parent_node = parent_node_stack.pop().unwrap();
                    parent_node.borrow_mut().add_child(Rc::clone(&node) as _);
                    parent_node_stack.push(parent_node);
                    parent_node_stack.push(node);
                }
                HTMLTagState::Closing => {
                    parent_node_stack.pop();
                }
                _ => {
                    let node_type: NodeType;
                    let tag_name = match tag.name.as_str() {
                        "" => None, // TODO should be FragmentNode
                        _ => Some(tag.name),
                    };
                    let node: Rc<RefCell<Box<dyn Node>>> = match tag.state {
                        HTMLTagState::Text => {
                            node_type = NodeType::TextNode;
                            Rc::new(RefCell::new(Box::new(TextNode::new())))
                        }
                        _ => {
                            node_type = NodeType::ElementNode;
                            Rc::new(RefCell::new(Box::new(ElementNode::new(tag_name))))
                        }
                    };
                    for (_, attr) in htmlstream::attr_iter(&tag.attributes) {
                        node.borrow_mut().add_attr(attr.name, attr.value);
                    }
                    // TextNode set innerHtml
                    if node_type == NodeType::TextNode {
                        node.borrow_mut().set_inner_html(tag.html)
                    }
                    let parent_node = parent_node_stack.pop().unwrap();
                    parent_node.borrow_mut().add_child(node);
                    parent_node_stack.push(parent_node);
                }
            }
        }
    }
    fn get_vd(&mut self) -> Option<Rc<RefCell<Box<dyn Node>>>> {
        let root = self.root.take();
        root.map(|r| Rc::clone(&r))
    }
}
