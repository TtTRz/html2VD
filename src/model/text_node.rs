use super::node::{Node, NodeType};
use serde::Serialize;

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
