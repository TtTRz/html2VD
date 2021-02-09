use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

use crate::model::node::Node;

#[derive(Debug, Deserialize, Serialize)]
pub struct VD {
    pub root: Option<Rc<RefCell<Node>>>,
}

impl Default for VD {
    fn default() -> Self {
        VD {
            // parser: Rc::new(RefCell::new(Parser::new())),
            root: None,
        }
    }
}

impl VD {
    pub fn new() -> Self {
        VD { ..Self::default() }
    }
}
