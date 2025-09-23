use dagviz::acyclic::Acyclic;
use dagviz::graph::Graph;
use dagviz::types::{Edge, EdgeLabel, NodeLabel};

fn main() {
    println!("Creating test graph...");

    let mut g = Graph::new();

    // Add nodes
    let a = g.add_node(NodeLabel::default());
    let b = g.add_node(NodeLabel::default());
    let c = g.add_node(NodeLabel::default());
    let d = g.add_node(NodeLabel::default());

    println!("Nodes: a={:?}, b={:?}, c={:?}, d={:?}", a, b, c, d);

    // Add edges to create a cycle: a->b->c->d->a
    let ab = g.add_edge(Edge::new(a, b), EdgeLabel::default());
    let bc = g.add_edge(Edge::new(b, c), EdgeLabel::default());
    let cd = g.add_edge(Edge::new(c, d), EdgeLabel::default());
    let da = g.add_edge(Edge::new(d, a), EdgeLabel::default());

    println!("Edges: ab={:?}, bc={:?}, cd={:?}, da={:?}", ab, bc, cd, da);

    println!("Running acyclic algorithm...");
    Acyclic::run(&mut g);

    println!("Acyclic algorithm completed successfully!");

    // Check if graph is now acyclic
    let is_acyclic = dagviz::acyclic::is_acyclic(&g);
    println!("Graph is acyclic: {}", is_acyclic);
}

