extern crate html2VD;

Some(RefCell { 
  value: Node { 
    tag: None, 
    attributes: [], 
    node_type: FragmentNode, 
    children: [RefCell { 
      value: Node { 
        tag: Some("div"), 
        attributes: [TagAttr { 
          name: "class", 
          value: "name" 
        }], 
        node_type: ElementNode, 
        children: [RefCell { 
          value: Node { 
            tag: Some("span"), 
            attributes: [], 
            node_type: ElementNode, 
            children: [], 
            parent: None 
          } 
        }, RefCell { 
          value: Node { 
            tag: Some("span"), 
            attributes: [], 
            node_type: ElementNode, 
            children: [], 
            parent: None 
          } 
        }], 
        parent: None 
      } 
    }], 
    parent: None 
  } 
})