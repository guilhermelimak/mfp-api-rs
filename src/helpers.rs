pub mod selector_helpers {
    use select::node::Node;

    pub fn get_macro_value(n: Node) -> Option<&str> {
        n.children()
            .filter(is_not_text)
            .next()
            .unwrap()
            .children()
            .next()
            .unwrap()
            .as_text()
    }

    pub fn get_calories(n: Node) -> Option<&str> {
        n.children().next().unwrap().as_text()
    }

    pub fn is_not_text(node: &Node) -> bool {
        node.name().is_some()
    }
}
