use dagviz::graph::Graph;
use dagviz::types::{Edge, EdgeLabel, NodeLabel};
use dagviz::util::{build_layer_matrix, normalize_ranks, time};

fn setup_test_graph() -> Graph {
    Graph::new()
}

#[test]
fn test_build_layer_matrix() {
    let mut g = setup_test_graph();

    let a = g.add_node(NodeLabel {
        rank: Some(0),
        order: Some(0),
        ..Default::default()
    });
    let b = g.add_node(NodeLabel {
        rank: Some(0),
        order: Some(1),
        ..Default::default()
    });
    let c = g.add_node(NodeLabel {
        rank: Some(1),
        order: Some(0),
        ..Default::default()
    });
    let d = g.add_node(NodeLabel {
        rank: Some(1),
        order: Some(1),
        ..Default::default()
    });
    let e = g.add_node(NodeLabel {
        rank: Some(2),
        order: Some(0),
        ..Default::default()
    });

    let matrix = build_layer_matrix(&g);

    // 验证矩阵结构
    assert_eq!(matrix.len(), 3); // 3个rank层
    assert_eq!(matrix[0].len(), 2); // rank 0有2个节点
    assert_eq!(matrix[1].len(), 2); // rank 1有2个节点
    assert_eq!(matrix[2].len(), 1); // rank 2有1个节点

    // 验证节点在正确的层中
    assert!(matrix[0].contains(&a));
    assert!(matrix[0].contains(&b));
    assert!(matrix[1].contains(&c));
    assert!(matrix[1].contains(&d));
    assert!(matrix[2].contains(&e));
}

#[test]
fn test_normalize_ranks_positive() {
    let mut g = setup_test_graph();

    let a = g.add_node(NodeLabel {
        rank: Some(3),
        ..Default::default()
    });
    let b = g.add_node(NodeLabel {
        rank: Some(2),
        ..Default::default()
    });
    let c = g.add_node(NodeLabel {
        rank: Some(4),
        ..Default::default()
    });

    normalize_ranks(&mut g);

    // 验证标准化后的rank值
    if let Some(a_label) = g.node_label(a) {
        assert_eq!(a_label.rank, Some(1));
    }
    if let Some(b_label) = g.node_label(b) {
        assert_eq!(b_label.rank, Some(0));
    }
    if let Some(c_label) = g.node_label(c) {
        assert_eq!(c_label.rank, Some(2));
    }
}

#[test]
fn test_normalize_ranks_negative() {
    let mut g = setup_test_graph();

    let a = g.add_node(NodeLabel {
        rank: Some(-3),
        ..Default::default()
    });
    let b = g.add_node(NodeLabel {
        rank: Some(-2),
        ..Default::default()
    });

    normalize_ranks(&mut g);

    // 验证标准化后的rank值
    if let Some(a_label) = g.node_label(a) {
        assert_eq!(a_label.rank, Some(0));
    }
    if let Some(b_label) = g.node_label(b) {
        assert_eq!(b_label.rank, Some(1));
    }
}

#[test]
fn test_normalize_ranks_single_node() {
    let mut g = setup_test_graph();

    let a = g.add_node(NodeLabel {
        rank: Some(5),
        ..Default::default()
    });

    normalize_ranks(&mut g);

    // 单个节点应该被标准化为0
    if let Some(a_label) = g.node_label(a) {
        assert_eq!(a_label.rank, Some(0));
    }
}

#[test]
fn test_normalize_ranks_empty_graph() {
    let mut g = setup_test_graph();

    normalize_ranks(&mut g);

    // 空图应该不会panic
    assert!(g.is_empty());
}

#[test]
fn test_time_function() {
    let result = time("test_operation", || {
        // 模拟一些工作
        std::thread::sleep(std::time::Duration::from_millis(1));
        "test_result"
    });

    assert_eq!(result, "test_result");
}

#[test]
fn test_time_with_closure() {
    let value = 42;
    let result = time("test_closure", || value * 2);

    assert_eq!(result, 84);
}

#[test]
fn test_build_layer_matrix_empty() {
    let g = setup_test_graph();
    let matrix = build_layer_matrix(&g);

    // 空图应该返回包含一个空向量的矩阵
    assert_eq!(matrix.len(), 1);
    assert!(matrix[0].is_empty());
}

#[test]
fn test_build_layer_matrix_single_rank() {
    let mut g = setup_test_graph();

    let a = g.add_node(NodeLabel {
        rank: Some(0),
        order: Some(0),
        ..Default::default()
    });
    let b = g.add_node(NodeLabel {
        rank: Some(0),
        order: Some(1),
        ..Default::default()
    });

    let matrix = build_layer_matrix(&g);

    assert_eq!(matrix.len(), 1);
    assert_eq!(matrix[0].len(), 2);
    assert!(matrix[0].contains(&a));
    assert!(matrix[0].contains(&b));
}

#[test]
fn test_build_layer_matrix_with_gaps() {
    let mut g = setup_test_graph();

    let a = g.add_node(NodeLabel {
        rank: Some(0),
        order: Some(0),
        ..Default::default()
    });
    let b = g.add_node(NodeLabel {
        rank: Some(2), // 跳过rank 1
        order: Some(0),
        ..Default::default()
    });

    let matrix = build_layer_matrix(&g);

    // 当有rank间隙时，矩阵应该包含所有rank层
    assert_eq!(matrix.len(), 3); // rank 0, 1, 2
    assert_eq!(matrix[0].len(), 1); // rank 0
    assert_eq!(matrix[1].len(), 0); // rank 1 (空)
    assert_eq!(matrix[2].len(), 1); // rank 2
    assert!(matrix[0].contains(&a));
    assert!(matrix[2].contains(&b));
}
