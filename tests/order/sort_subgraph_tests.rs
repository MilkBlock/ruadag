//! 子图排序功能测试
//! 对应JavaScript版本的 test/order/sort-subgraph-test.js

use dagviz::graph::Graph;
use dagviz::order::constraint_graph::ConstraintGraph;
use dagviz::order::sort_subgraph::sort_subgraph;
use dagviz::types::{Edge, EdgeLabel, NodeLabel};

#[test]
fn test_sort_simple_subgraph() {
    // 测试简单子图排序
    let mut g = Graph::new();
    let cg = ConstraintGraph::new();

    // 添加节点
    let mut root_label = NodeLabel::default();
    root_label.label = Some("root".to_string());
    let mut node1_label = NodeLabel::default();
    node1_label.label = Some("node1".to_string());
    let mut node2_label = NodeLabel::default();
    node2_label.label = Some("node2".to_string());
    let root = g.add_node(root_label);
    let node1 = g.add_node(node1_label);
    let node2 = g.add_node(node2_label);

    // 设置父子关系
    g.set_parent(node1, root);
    g.set_parent(node2, root);

    // 添加边
    g.add_edge(Edge::new(node1, node2), EdgeLabel::default());

    let result = sort_subgraph(&g, root, &cg, false);
    assert!(!result.is_empty());
}

#[test]
fn test_sort_empty_subgraph() {
    // 测试空子图排序
    let mut g = Graph::new();
    let cg = ConstraintGraph::new();

    let mut root_label = NodeLabel::default();
    root_label.label = Some("root".to_string());
    let root = g.add_node(root_label);

    let result = sort_subgraph(&g, root, &cg, false);
    assert!(result.is_empty());
}

#[test]
fn test_sort_with_bias() {
    // 测试带偏置的排序
    let mut g = Graph::new();
    let cg = ConstraintGraph::new();

    // 添加节点
    let mut root_label = NodeLabel::default();
    root_label.label = Some("root".to_string());
    let mut node1_label = NodeLabel::default();
    node1_label.label = Some("node1".to_string());
    let mut node2_label = NodeLabel::default();
    node2_label.label = Some("node2".to_string());
    let root = g.add_node(root_label);
    let node1 = g.add_node(node1_label);
    let node2 = g.add_node(node2_label);

    // 设置父子关系
    g.set_parent(node1, root);
    g.set_parent(node2, root);

    // 添加边
    g.add_edge(Edge::new(node1, node2), EdgeLabel::default());

    let result_left = sort_subgraph(&g, root, &cg, false);
    let result_right = sort_subgraph(&g, root, &cg, true);

    assert!(!result_left.is_empty());
    assert!(!result_right.is_empty());
}
