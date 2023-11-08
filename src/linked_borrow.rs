pub struct Node {
    pub key: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}
fn min_value_node<'a>(node: Option<&'a mut Node>) -> Option<&'a mut Node> {
    let mut current = node?;
    while let Some(mut left) = current.left.as_deref_mut() {
        current = left;
    }
    Some(current)
}

mod test {
    // add code here
    #[test]
    fn test() {}
}
