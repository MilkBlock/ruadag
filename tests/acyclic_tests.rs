use dagviz::acyclic::{Acyclic, find_cycles};
use dagviz::graph::Graph;
use dagviz::types::{Edge, EdgeLabel, GraphConfig, NodeLabel};

fn setup_test_graph() -> Graph {
    let config = GraphConfig::default();
    Graph::with_config(config)
}

fn setup_test_graph_with_acyclicer(acyclicer: &str) -> Graph {
    let mut config = GraphConfig::default();
    config.acyclicer = acyclicer.to_string();
    Graph::with_config(config)
}

// Helper functions to match JavaScript test structure
fn strip_label(edge: Edge) -> Edge {
    Edge::new(edge.source, edge.target)
}

fn sort_edges(mut edges: Vec<Edge>) -> Vec<Edge> {
    edges.sort_by(|a, b| {
        a.source.index().cmp(&b.source.index()).then(
            a.target.index().cmp(&b.target.index())
        )
    });
    edges
}

#[test]
fn test_does_not_change_already_acyclic_graph_greedy() {
    let mut g = setup_test_graph_with_acyclicer("greedy");
    
    // Create path: a -> b -> d and a -> c -> d
    let a = g.add_node(NodeLabel::default());
    let b = g.add_node(NodeLabel::default());
    let c = g.add_node(NodeLabel::default());
    let d = g.add_node(NodeLabel::default());
    
    g.add_edge(Edge::new(a, b), EdgeLabel::default());
    g.add_edge(Edge::new(a, c), EdgeLabel::default());
    g.add_edge(Edge::new(b, d), EdgeLabel::default());
    g.add_edge(Edge::new(c, d), EdgeLabel::default());
    
    Acyclic::run(&mut g);
    
    let mut results: Vec<Edge> = g.edges().into_iter().map(strip_label).collect();
    results = sort_edges(results);
    
    let expected = vec![
        Edge::new(a, b),
        Edge::new(a, c),
        Edge::new(b, d),
        Edge::new(c, d),
    ];
    
    assert_eq!(results, expected);
}

#[test]
fn test_does_not_change_already_acyclic_graph_dfs() {
    let mut g = setup_test_graph_with_acyclicer("dfs");
    
    // Create path: a -> b -> d and a -> c -> d
    let a = g.add_node(NodeLabel::default());
    let b = g.add_node(NodeLabel::default());
    let c = g.add_node(NodeLabel::default());
    let d = g.add_node(NodeLabel::default());
    
    g.add_edge(Edge::new(a, b), EdgeLabel::default());
    g.add_edge(Edge::new(a, c), EdgeLabel::default());
    g.add_edge(Edge::new(b, d), EdgeLabel::default());
    g.add_edge(Edge::new(c, d), EdgeLabel::default());
    
    Acyclic::run(&mut g);
    
    let mut results: Vec<Edge> = g.edges().into_iter().map(strip_label).collect();
    results = sort_edges(results);
    
    let expected = vec![
        Edge::new(a, b),
        Edge::new(a, c),
        Edge::new(b, d),
        Edge::new(c, d),
    ];
    
    assert_eq!(results, expected);
}

#[test]
fn test_breaks_cycles_in_input_graph() {
    let mut g = setup_test_graph_with_acyclicer("greedy");
    
    // Create cycle: a -> b -> c -> d -> a
    let a = g.add_node(NodeLabel::default());
    let b = g.add_node(NodeLabel::default());
    let c = g.add_node(NodeLabel::default());
    let d = g.add_node(NodeLabel::default());
    
    g.add_edge(Edge::new(a, b), EdgeLabel::default());
    g.add_edge(Edge::new(b, c), EdgeLabel::default());
    g.add_edge(Edge::new(c, d), EdgeLabel::default());
    g.add_edge(Edge::new(d, a), EdgeLabel::default());
    
    Acyclic::run(&mut g);
    
    let cycles = find_cycles(&g);
    assert!(cycles.is_empty());
}

#[test]
fn test_creates_multi_edge_where_necessary() {
    let mut g = setup_test_graph_with_acyclicer("greedy");
    
    // Create cycle: a -> b -> a
    let a = g.add_node(NodeLabel::default());
    let b = g.add_node(NodeLabel::default());
    
    g.add_edge(Edge::new(a, b), EdgeLabel::default());
    g.add_edge(Edge::new(b, a), EdgeLabel::default());
    
    Acyclic::run(&mut g);
    
    let cycles = find_cycles(&g);
    assert!(cycles.is_empty());
    assert_eq!(g.edge_count(), 2);
}

#[test]
fn test_undo_does_not_change_edges_where_original_was_acyclic() {
    let mut g = setup_test_graph_with_acyclicer("greedy");
    
    let a = g.add_node(NodeLabel::default());
    let b = g.add_node(NodeLabel::default());
    
    let mut edge_label = EdgeLabel::default();
    edge_label.minlen = 2;
    edge_label.weight = 3.0;
    
    g.add_edge(Edge::new(a, b), edge_label.clone());
    
    Acyclic::run(&mut g);
    Acyclic::undo(&mut g);
    
    let retrieved_label = g.edge_label(&Edge::new(a, b));
    assert!(retrieved_label.is_some());
    let retrieved = retrieved_label.unwrap();
    assert_eq!(retrieved.minlen, 2);
    assert_eq!(retrieved.weight, 3.0);
    assert_eq!(g.edge_count(), 1);
}

#[test]
fn test_undo_can_restore_previously_reversed_edges() {
    let mut g = setup_test_graph_with_acyclicer("greedy");
    
    let a = g.add_node(NodeLabel::default());
    let b = g.add_node(NodeLabel::default());
    
    let mut edge1_label = EdgeLabel::default();
    edge1_label.minlen = 2;
    edge1_label.weight = 3.0;
    
    let mut edge2_label = EdgeLabel::default();
    edge2_label.minlen = 3;
    edge2_label.weight = 4.0;
    
    g.add_edge(Edge::new(a, b), edge1_label);
    g.add_edge(Edge::new(b, a), edge2_label);
    
    Acyclic::run(&mut g);
    Acyclic::undo(&mut g);
    
    let retrieved1 = g.edge_label(&Edge::new(a, b));
    let retrieved2 = g.edge_label(&Edge::new(b, a));
    
    assert!(retrieved1.is_some());
    assert!(retrieved2.is_some());
    
    let label1 = retrieved1.unwrap();
    let label2 = retrieved2.unwrap();
    
    assert_eq!(label1.minlen, 2);
    assert_eq!(label1.weight, 3.0);
    assert_eq!(label2.minlen, 3);
    assert_eq!(label2.weight, 4.0);
    assert_eq!(g.edge_count(), 2);
}

#[test]
fn test_greedy_prefers_to_break_cycles_at_low_weight_edges() {
    let mut g = setup_test_graph_with_acyclicer("greedy");
    
    // Create cycle: a -> b -> c -> d -> a
    let a = g.add_node(NodeLabel::default());
    let b = g.add_node(NodeLabel::default());
    let c = g.add_node(NodeLabel::default());
    let d = g.add_node(NodeLabel::default());
    
    let mut default_label = EdgeLabel::default();
    default_label.weight = 2.0;
    
    let mut low_weight_label = EdgeLabel::default();
    low_weight_label.weight = 1.0;
    
    g.add_edge(Edge::new(a, b), default_label.clone());
    g.add_edge(Edge::new(b, c), default_label.clone());
    g.add_edge(Edge::new(c, d), low_weight_label);
    g.add_edge(Edge::new(d, a), default_label);
    
    Acyclic::run(&mut g);
    
    let cycles = find_cycles(&g);
    assert!(cycles.is_empty());
    
    // The low-weight edge (c, d) should be removed
    assert!(!g.has_edge(&Edge::new(c, d)));
}

#[test]
fn test_unknown_acyclicer_still_works() {
    let mut g = setup_test_graph_with_acyclicer("unknown-should-still-work");
    
    // Create path: a -> b -> d and a -> c -> d
    let a = g.add_node(NodeLabel::default());
    let b = g.add_node(NodeLabel::default());
    let c = g.add_node(NodeLabel::default());
    let d = g.add_node(NodeLabel::default());
    
    g.add_edge(Edge::new(a, b), EdgeLabel::default());
    g.add_edge(Edge::new(a, c), EdgeLabel::default());
    g.add_edge(Edge::new(b, d), EdgeLabel::default());
    g.add_edge(Edge::new(c, d), EdgeLabel::default());
    
    Acyclic::run(&mut g);
    
    let cycles = find_cycles(&g);
    assert!(cycles.is_empty());
}
