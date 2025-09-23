use dagviz::graph::Graph;
use dagviz::position::position;
use dagviz::types::{Edge, EdgeLabel, NodeLabel};

fn setup_test_graph() -> Graph {
    Graph::new()
}

#[test]
fn test_positions_single_node_at_origin() {
    let mut g = setup_test_graph();
    let a = g.add_node(NodeLabel {
        rank: Some(0),
        order: Some(0),
        width: 100.0,
        ..Default::default()
    });
    
    position(&mut g);
    
    if let Some(label) = g.node_label(a) {
        assert_eq!(label.x, Some(0.0));
    }
}

#[test]
fn test_positions_single_node_block_at_origin() {
    let mut g = setup_test_graph();
    let a = g.add_node(NodeLabel {
        rank: Some(0),
        order: Some(0),
        width: 100.0,
        ..Default::default()
    });
    let b = g.add_node(NodeLabel {
        rank: Some(1),
        order: Some(0),
        width: 100.0,
        ..Default::default()
    });
    
    g.add_edge(Edge::new(a, b), EdgeLabel::default());
    
    position(&mut g);
    
    if let (Some(a_label), Some(b_label)) = (g.node_label(a), g.node_label(b)) {
        assert_eq!(a_label.x, Some(0.0));
        assert_eq!(b_label.x, Some(0.0));
    }
}

#[test]
fn test_positions_single_node_block_at_origin_with_different_sizes() {
    let mut g = setup_test_graph();
    let a = g.add_node(NodeLabel {
        rank: Some(0),
        order: Some(0),
        width: 40.0,
        ..Default::default()
    });
    let b = g.add_node(NodeLabel {
        rank: Some(1),
        order: Some(0),
        width: 500.0,
        ..Default::default()
    });
    let c = g.add_node(NodeLabel {
        rank: Some(2),
        order: Some(0),
        width: 20.0,
        ..Default::default()
    });
    
    g.add_edge(Edge::new(a, b), EdgeLabel::default());
    g.add_edge(Edge::new(b, c), EdgeLabel::default());
    
    position(&mut g);
    
    if let (Some(a_label), Some(b_label), Some(c_label)) = 
        (g.node_label(a), g.node_label(b), g.node_label(c)) {
        assert_eq!(a_label.x, Some(0.0));
        assert_eq!(b_label.x, Some(0.0));
        assert_eq!(c_label.x, Some(0.0));
    }
}

#[test]
fn test_centers_node_if_predecessor_of_two_same_sized_nodes() {
    let mut g = setup_test_graph();
    let mut config = g.config().clone();
    config.node_sep = 10.0;
    g.set_config(config);
    
    let a = g.add_node(NodeLabel {
        rank: Some(0),
        order: Some(0),
        width: 20.0,
        ..Default::default()
    });
    let b = g.add_node(NodeLabel {
        rank: Some(1),
        order: Some(0),
        width: 50.0,
        ..Default::default()
    });
    let c = g.add_node(NodeLabel {
        rank: Some(1),
        order: Some(1),
        width: 50.0,
        ..Default::default()
    });
    
    g.add_edge(Edge::new(a, b), EdgeLabel::default());
    g.add_edge(Edge::new(a, c), EdgeLabel::default());
    
    position(&mut g);
    
    if let (Some(a_label), Some(b_label), Some(c_label)) = 
        (g.node_label(a), g.node_label(b), g.node_label(c)) {
        let a_x = a_label.x.unwrap_or(0.0);
        let b_x = b_label.x.unwrap_or(0.0);
        let c_x = c_label.x.unwrap_or(0.0);
        
        // b should be to the left of a, c should be to the right of a
        assert!(b_x < a_x);
        assert!(c_x > a_x);
    }
}

#[test]
fn test_handles_empty_graph() {
    let mut g = setup_test_graph();
    
    position(&mut g);
    
    // Should not panic
    assert!(g.is_empty());
}

#[test]
fn test_handles_single_node_without_rank() {
    let mut g = setup_test_graph();
    let a = g.add_node(NodeLabel {
        width: 100.0,
        ..Default::default()
    });
    
    position(&mut g);
    
    // Should not panic, even without rank
    assert!(g.has_node(a));
}

