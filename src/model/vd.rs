use std::cell::RefCell;
use std::rc::Rc;

use crate::model::node::Node;

#[derive(Debug)]
pub struct VD {
    pub root: Option<Rc<RefCell<Box<dyn Node>>>>,
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
