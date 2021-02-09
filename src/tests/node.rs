use crate::model::node::Node;

#[test]
fn test_node_new() {
    let n = Node::new();
    println!("{:?}", n);
}
