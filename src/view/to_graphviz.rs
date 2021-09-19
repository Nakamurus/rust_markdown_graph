use petgraph::dot::{Config, Dot};
use petgraph::Graph;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

pub fn visualize(file_name: &str, connected: Vec<(&str, &str)>) -> std::io::Result<()> {
    let mut graph: Graph<&str, u32> = Graph::new();
    let mut added_node = BTreeMap::new();
    for (a, b) in connected.iter() {
        let x = if added_node.contains_key(a) {
            *added_node.get(a).unwrap()
        } else {
            graph.add_node(a)
        };
        let y = if added_node.contains_key(b) {
            *added_node.get(b).unwrap()
        } else {
            graph.add_node(b)
        };
        added_node.insert(a, x);
        added_node.insert(b, y);
        graph.update_edge(x, y, 0);
    }
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    let mut f = File::create(format!("{}.dot", file_name)).unwrap();
    let output = format!("{}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    f.write_all(&output.as_bytes())
}
