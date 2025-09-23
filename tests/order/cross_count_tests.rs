use dagviz::graph::Graph;
use dagviz::order::cross_count::cross_count;
use dagviz::types::{Edge, EdgeLabel, NodeLabel};

fn setup_test_graph() -> Graph {
    Graph::new()
}

#[test]
fn test_returns_zero_for_empty_layering() {
    let g = setup_test_graph();
    let layering: Vec<Vec<dagviz::graph::NodeIndex>> = vec![];
    assert_eq!(cross_count(&g, &layering), 0);
}

#[test]
fn test_returns_zero_for_layering_with_no_crossings() {
    let mut g = setup_test_graph();
    
    let a1 = g.add_node(NodeLabel::default());
    let a2 = g.add_node(NodeLabel::default());
    let b1 = g.add_node(NodeLabel::default());
    let b2 = g.add_node(NodeLabel::default());
    
    g.add_edge(Edge::new(a1, b1), EdgeLabel::default());
    g.add_edge(Edge::new(a2, b2), EdgeLabel::default());
    
    let layering = vec![vec![a1, a2], vec![b1, b2]];
    assert_eq!(cross_count(&g, &layering), 0);
}

#[test]
fn test_returns_one_for_layering_with_one_crossing() {
    let mut g = setup_test_graph();
    
    let a1 = g.add_node(NodeLabel::default());
    let a2 = g.add_node(NodeLabel::default());
    let b1 = g.add_node(NodeLabel::default());
    let b2 = g.add_node(NodeLabel::default());
    
    g.add_edge(Edge::new(a1, b1), EdgeLabel::default());
    g.add_edge(Edge::new(a2, b2), EdgeLabel::default());
    
    let layering = vec![vec![a1, a2], vec![b2, b1]];
    assert_eq!(cross_count(&g, &layering), 1);
}

#[test]
fn test_returns_weighted_crossing_count_for_layering_with_one_crossing() {
    let mut g = setup_test_graph();
    
    let a1 = g.add_node(NodeLabel::default());
    let a2 = g.add_node(NodeLabel::default());
    let b1 = g.add_node(NodeLabel::default());
    let b2 = g.add_node(NodeLabel::default());
    
    g.add_edge(Edge::new(a1, b1), EdgeLabel {
        weight: 2.0,
        ..Default::default()
    });
    g.add_edge(Edge::new(a2, b2), EdgeLabel {
        weight: 3.0,
        ..Default::default()
    });
    
    let layering = vec![vec![a1, a2], vec![b2, b1]];
    assert_eq!(cross_count(&g, &layering), 6);
}

#[test]
fn test_calculates_crossings_across_layers() {
    let mut g = setup_test_graph();
    
    let a1 = g.add_node(NodeLabel::default());
    let a2 = g.add_node(NodeLabel::default());
    let b1 = g.add_node(NodeLabel::default());
    let b2 = g.add_node(NodeLabel::default());
    let c1 = g.add_node(NodeLabel::default());
    let c2 = g.add_node(NodeLabel::default());
    
    g.add_edge(Edge::new(a1, b1), EdgeLabel::default());
    g.add_edge(Edge::new(b1, c1), EdgeLabel::default());
    g.add_edge(Edge::new(a2, b2), EdgeLabel::default());
    g.add_edge(Edge::new(b2, c2), EdgeLabel::default());
    
    let layering = vec![vec![a1, a2], vec![b2, b1], vec![c1, c2]];
    assert_eq!(cross_count(&g, &layering), 2);
}

#[test]
fn test_works_for_graph_1() {
    let mut g = setup_test_graph();
    
    let a = g.add_node(NodeLabel::default());
    let b = g.add_node(NodeLabel::default());
    let c = g.add_node(NodeLabel::default());
    let d = g.add_node(NodeLabel::default());
    let e = g.add_node(NodeLabel::default());
    let f = g.add_node(NodeLabel::default());
    let i = g.add_node(NodeLabel::default());
    
    // Path: a -> b -> c
    g.add_edge(Edge::new(a, b), EdgeLabel::default());
    g.add_edge(Edge::new(b, c), EdgeLabel::default());
    
    // Path: d -> e -> c
    g.add_edge(Edge::new(d, e), EdgeLabel::default());
    g.add_edge(Edge::new(e, c), EdgeLabel::default());
    
    // Path: a -> f -> i
    g.add_edge(Edge::new(a, f), EdgeLabel::default());
    g.add_edge(Edge::new(f, i), EdgeLabel::default());
    
    // Edge: a -> e
    g.add_edge(Edge::new(a, e), EdgeLabel::default());
    
    let layering1 = vec![vec![a, d], vec![b, e, f], vec![c, i]];
    assert_eq!(cross_count(&g, &layering1), 1);
    
    let layering2 = vec![vec![d, a], vec![e, b, f], vec![c, i]];
    assert_eq!(cross_count(&g, &layering2), 0);
}

