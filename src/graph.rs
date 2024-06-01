pub struct Node<T> {
    value: T,
}

pub struct Edge<T> {
    from: Node<T>,
    to: Node<T>,
    condition: Box<dyn Fn() -> bool>,
}
pub trait Graph<T> {

    fn set_entry_point(&mut self, node: Node<T>);
    fn add_node(&mut self, node: Node<T>);
    fn add_edge(&mut self, edge: Edge<T>);
    fn add_conditional_edge(&mut self, from: Node<T>, to: Node<T>, condition: Box<dyn Fn() -> bool>);
    fn traverse(&self) -> Vec<&T>;
}
pub struct DirectedGraph<T> {
    start: Node<T>,
    nodes: Vec<Node<T>>,
    edges: Vec<Edge<T>>,
}

impl<T: PartialEq + Clone> Graph<T> for DirectedGraph<T> {
    fn set_entry_point(&mut self, node: Node<T>) {
        self.start = node;
    }
    fn add_node(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, edge: Edge<T>) {
        self.edges.push(edge);
    }

    fn add_conditional_edge(&mut self, from: Node<T>, to: Node<T>, condition: Box<dyn Fn() -> bool>) {
        let edge = Edge { from, to, condition };
        self.edges.push(edge);
    }

    fn traverse(&self) -> Vec<&T> {
        let mut visited: Vec<&T> = Vec::new();
        let mut stack: Vec<&T> = Vec::new();

        stack.push(&self.start);

        while let Some(node) = stack.pop() {
            if !visited.contains(node) {
                visited.push(node);
                for edge in self.edges.iter() {
                    if edge.from == *node {
                        stack.push(&edge.to);
                    }
                }
            }
        }
        visited
    }
}