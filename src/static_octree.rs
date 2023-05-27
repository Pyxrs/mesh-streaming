struct StaticOctree {
    size: u32, // Size of root node in meters
    root_node: StaticNode,
}

impl StaticOctree {
    fn new(size: u32) -> Self {
        Self {
            size,
            root_node: StaticNode::default(),
        }
    }
}

#[derive(Default)]
struct StaticNode {
    leaf: bool,
    id: u32,
    nodes: Vec<StaticNode>,
}