struct Octree {
    size: u32, // Size of root node in meters
    root_node: Node,
}

impl Octree {
    fn new(size: u32) -> Self {
        Self {
            size,
            root_node: Node::default(),
        }
    }
}

#[derive(Default)]
struct Node {
    leaf: bool,
    id: u16,
    nodes: Vec<Node>,
}