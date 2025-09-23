use dagviz::graph::Graph;
use dagviz::types::Edge;
use dagviz::types::EdgeLabel;
use dagviz::types::GraphConfig;
use dagviz::types::NodeLabel;

fn setup_test_graph() -> Graph {
    let config = GraphConfig::default();
    Graph::with_config(config)
}

#[test]
fn test_create_empty_graph() {
    let g = setup_test_graph();
    assert!(g.is_empty());
    assert_eq!(g.node_count(), 0);
    assert_eq!(g.edge_count(), 0);
}

#[test]
fn test_add_nodes_and_edges() {
    let mut g = setup_test_graph();

    // 添加节点
    let node1 = g.add_node(NodeLabel::default());
    let node2 = g.add_node(NodeLabel::default());

    // 添加边
    let edge = Edge::new(node1, node2);
    let edge_label = EdgeLabel::default();
    let _edge_index = g.add_edge(edge.clone(), edge_label);

    assert_eq!(g.node_count(), 2);
    assert_eq!(g.edge_count(), 1);
    assert!(g.has_edge(&edge));
}

#[test]
fn test_graph_config() {
    let g = setup_test_graph();
    let config = g.config().clone();
    // GraphType was removed in refactoring, just verify config exists
    assert!(config.node_sep > 0.0);
}

#[test]
fn test_node_operations() {
    let mut g = setup_test_graph();

    let node = g.add_node(NodeLabel::default());
    assert!(g.has_node(node));

    let node_label = g.node_label(node);
    assert!(node_label.is_some());
}

#[test]
fn test_edge_operations() {
    let mut g = setup_test_graph();

    let node1 = g.add_node(NodeLabel::default());
    let node2 = g.add_node(NodeLabel::default());
    let edge = Edge::new(node1, node2);
    let edge_label = EdgeLabel::default();

    let _edge_index = g.add_edge(edge.clone(), edge_label.clone());

    let retrieved_label = g.edge_label(&edge);
    assert!(retrieved_label.is_some());
}
