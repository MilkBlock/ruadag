use dagviz::graph::Graph;
use dagviz::order::cross_count::cross_count;
use dagviz::order::order;
use dagviz::types::{Edge, EdgeLabel, NodeLabel};
use dagviz::util::build_layer_matrix;

#[test]
fn test_does_not_add_crossings_to_tree_structure() {
    let mut g = Graph::new();

    // 添加节点
    let a = g.add_node(NodeLabel {
        rank: Some(1),
        ..Default::default()
    });
    let b = g.add_node(NodeLabel {
        rank: Some(2),
        ..Default::default()
    });
    let e = g.add_node(NodeLabel {
        rank: Some(2),
        ..Default::default()
    });
    let c = g.add_node(NodeLabel {
        rank: Some(3),
        ..Default::default()
    });
    let d = g.add_node(NodeLabel {
        rank: Some(3),
        ..Default::default()
    });
    let f = g.add_node(NodeLabel {
        rank: Some(3),
        ..Default::default()
    });

    // 添加边
    g.add_edge(
        Edge::new(a, b),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(b, c),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(b, d),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(a, e),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(e, f),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );

    order(&mut g, None);
    let layering = build_layer_matrix(&g);
    assert_eq!(cross_count(&g, &layering), 0);
}

#[test]
fn test_can_solve_simple_graph() {
    let mut g = Graph::new();

    // 添加节点
    let a = g.add_node(NodeLabel {
        rank: Some(1),
        ..Default::default()
    });
    let d = g.add_node(NodeLabel {
        rank: Some(1),
        ..Default::default()
    });
    let b = g.add_node(NodeLabel {
        rank: Some(2),
        ..Default::default()
    });
    let c = g.add_node(NodeLabel {
        rank: Some(3),
        ..Default::default()
    });

    // 添加边
    g.add_edge(
        Edge::new(a, b),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(b, c),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(d, c),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );

    order(&mut g, None);
    let layering = build_layer_matrix(&g);
    assert_eq!(cross_count(&g, &layering), 0);
}

#[test]
fn test_can_solve_complex_graph() {
    let mut g = Graph::new();

    // 添加节点
    let a = g.add_node(NodeLabel {
        rank: Some(1),
        ..Default::default()
    });
    let b = g.add_node(NodeLabel {
        rank: Some(2),
        ..Default::default()
    });
    let c = g.add_node(NodeLabel {
        rank: Some(2),
        ..Default::default()
    });
    let d = g.add_node(NodeLabel {
        rank: Some(3),
        ..Default::default()
    });

    // 添加边
    g.add_edge(
        Edge::new(a, b),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(a, c),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(b, d),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(c, d),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );

    order(&mut g, None);
    let layering = build_layer_matrix(&g);
    assert_eq!(cross_count(&g, &layering), 0);
}

#[test]
fn test_can_solve_multi_layer_graph() {
    let mut g = Graph::new();

    // 添加节点
    let a = g.add_node(NodeLabel {
        rank: Some(1),
        ..Default::default()
    });
    let b = g.add_node(NodeLabel {
        rank: Some(2),
        ..Default::default()
    });
    let c = g.add_node(NodeLabel {
        rank: Some(2),
        ..Default::default()
    });
    let d = g.add_node(NodeLabel {
        rank: Some(2),
        ..Default::default()
    });
    let e = g.add_node(NodeLabel {
        rank: Some(3),
        ..Default::default()
    });
    let f = g.add_node(NodeLabel {
        rank: Some(3),
        ..Default::default()
    });
    let g_node = g.add_node(NodeLabel {
        rank: Some(3),
        ..Default::default()
    });
    let h = g.add_node(NodeLabel {
        rank: Some(3),
        ..Default::default()
    });
    let i = g.add_node(NodeLabel {
        rank: Some(4),
        ..Default::default()
    });

    // 添加边
    g.add_edge(
        Edge::new(a, b),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(a, c),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(a, d),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(b, e),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(b, f),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(c, g_node),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(c, h),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(d, h),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(e, i),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(f, i),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(g_node, i),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(h, i),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );

    order(&mut g, None);
    let layering = build_layer_matrix(&g);
    assert_eq!(cross_count(&g, &layering), 0);
}

#[test]
fn test_can_solve_dense_graph() {
    let mut g = Graph::new();

    // 添加节点
    let a = g.add_node(NodeLabel {
        rank: Some(1),
        ..Default::default()
    });
    let b = g.add_node(NodeLabel {
        rank: Some(2),
        ..Default::default()
    });
    let c = g.add_node(NodeLabel {
        rank: Some(2),
        ..Default::default()
    });
    let d = g.add_node(NodeLabel {
        rank: Some(2),
        ..Default::default()
    });
    let e = g.add_node(NodeLabel {
        rank: Some(3),
        ..Default::default()
    });
    let f = g.add_node(NodeLabel {
        rank: Some(3),
        ..Default::default()
    });
    let g_node = g.add_node(NodeLabel {
        rank: Some(3),
        ..Default::default()
    });
    let h = g.add_node(NodeLabel {
        rank: Some(3),
        ..Default::default()
    });
    let i = g.add_node(NodeLabel {
        rank: Some(4),
        ..Default::default()
    });

    // 添加边
    g.add_edge(
        Edge::new(a, b),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(a, c),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(a, d),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(b, e),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(b, f),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(b, g_node),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(b, h),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(c, e),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(c, f),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(c, g_node),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(c, h),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(d, e),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(d, f),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(d, g_node),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(d, h),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(e, i),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(f, i),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(g_node, i),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(h, i),
        EdgeLabel {
            weight: 1.0,
            ..Default::default()
        },
    );

    order(&mut g, None);
    let layering = build_layer_matrix(&g);
    assert_eq!(cross_count(&g, &layering), 0);
}

#[test]
fn test_can_solve_single_node() {
    let mut g = Graph::new();

    let a = g.add_node(NodeLabel {
        rank: Some(1),
        ..Default::default()
    });

    order(&mut g, None);
    let layering = build_layer_matrix(&g);
    assert_eq!(cross_count(&g, &layering), 0);
}

#[test]
fn test_can_solve_empty_graph() {
    let mut g = Graph::new();

    order(&mut g, None);
    let layering = build_layer_matrix(&g);
    assert_eq!(cross_count(&g, &layering), 0);
}
