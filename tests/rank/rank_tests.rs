use dagviz::graph::Graph;
use dagviz::rank::rank;
use dagviz::types::{Edge, EdgeLabel, GraphConfig, NodeLabel, Ranker};
use dagviz::util::normalize_ranks;

fn setup_test_graph() -> Graph {
    let mut config = GraphConfig::default();
    config.ranker = Ranker::NetworkSimplex;
    Graph::with_config(config)
}

fn setup_test_graph_with_ranker(ranker: Ranker) -> Graph {
    let mut config = GraphConfig::default();
    config.ranker = ranker;
    Graph::with_config(config)
}

#[test]
fn test_respects_minlen_attribute_network_simplex() {
    let mut g = setup_test_graph_with_ranker(Ranker::NetworkSimplex);

    // Create path: a -> b -> c -> d -> h
    let a = g.add_node(NodeLabel::default());
    let b = g.add_node(NodeLabel::default());
    let c = g.add_node(NodeLabel::default());
    let d = g.add_node(NodeLabel::default());
    let h = g.add_node(NodeLabel::default());

    // Create path: a -> e -> g -> h
    let e = g.add_node(NodeLabel::default());
    let g_node = g.add_node(NodeLabel::default());

    // Create path: a -> f -> g
    let f = g.add_node(NodeLabel::default());

    // Add edges with minlen
    g.add_edge(
        Edge::new(a, b),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(b, c),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(c, d),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(d, h),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(a, e),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(e, g_node),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(g_node, h),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(a, f),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(f, g_node),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );

    rank(&mut g);

    // 标准化排名，确保所有rank都是非负数
    normalize_ranks(&mut g);

    // Check that all edges respect minlen
    for edge in g.edges() {
        if let (Some(v_label), Some(w_label)) =
            (g.node_label(edge.source), g.node_label(edge.target))
        {
            if let (Some(v_rank), Some(w_rank)) = (v_label.rank, w_label.rank) {
                if let Some(edge_label) = g.edge_label(&edge) {
                    let minlen = edge_label.minlen;
                    assert!(w_rank - v_rank >= minlen as i32);
                }
            }
        }
    }
}

#[test]
fn test_respects_minlen_attribute_feasible_tree() {
    let mut g = setup_test_graph_with_ranker(Ranker::FeasibleTree);

    // Create path: a -> b -> c -> d -> h
    let a = g.add_node(NodeLabel::default());
    let b = g.add_node(NodeLabel::default());
    let c = g.add_node(NodeLabel::default());
    let d = g.add_node(NodeLabel::default());
    let h = g.add_node(NodeLabel::default());

    // Create path: a -> e -> g -> h
    let e = g.add_node(NodeLabel::default());
    let g_node = g.add_node(NodeLabel::default());

    // Create path: a -> f -> g
    let f = g.add_node(NodeLabel::default());

    // Add edges with minlen
    g.add_edge(
        Edge::new(a, b),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(b, c),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(c, d),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(d, h),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(a, e),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(e, g_node),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(g_node, h),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(a, f),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(f, g_node),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );

    rank(&mut g);

    // 标准化排名，确保所有rank都是非负数
    normalize_ranks(&mut g);

    // Check that all edges respect minlen
    for edge in g.edges() {
        if let (Some(v_label), Some(w_label)) =
            (g.node_label(edge.source), g.node_label(edge.target))
        {
            if let (Some(v_rank), Some(w_rank)) = (v_label.rank, w_label.rank) {
                if let Some(edge_label) = g.edge_label(&edge) {
                    let minlen = edge_label.minlen;
                    assert!(w_rank - v_rank >= minlen as i32);
                }
            }
        }
    }
}

#[test]
fn test_respects_minlen_attribute_longest_path() {
    let mut g = setup_test_graph_with_ranker(Ranker::LongestPath);

    // Create path: a -> b -> c -> d -> h
    let a = g.add_node(NodeLabel::default());
    let b = g.add_node(NodeLabel::default());
    let c = g.add_node(NodeLabel::default());
    let d = g.add_node(NodeLabel::default());
    let h = g.add_node(NodeLabel::default());

    // Create path: a -> e -> g -> h
    let e = g.add_node(NodeLabel::default());
    let g_node = g.add_node(NodeLabel::default());

    // Create path: a -> f -> g
    let f = g.add_node(NodeLabel::default());

    // Add edges with minlen
    g.add_edge(
        Edge::new(a, b),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(b, c),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(c, d),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(d, h),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(a, e),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(e, g_node),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(g_node, h),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(a, f),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(f, g_node),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );

    rank(&mut g);

    // 标准化排名，确保所有rank都是非负数
    normalize_ranks(&mut g);

    // Check that all edges respect minlen
    for edge in g.edges() {
        if let (Some(v_label), Some(w_label)) =
            (g.node_label(edge.source), g.node_label(edge.target))
        {
            if let (Some(v_rank), Some(w_rank)) = (v_label.rank, w_label.rank) {
                if let Some(edge_label) = g.edge_label(&edge) {
                    let minlen = edge_label.minlen;
                    assert!(w_rank - v_rank >= minlen as i32);
                }
            }
        }
    }
}

#[test]
fn test_respects_minlen_attribute_tight_tree() {
    let mut g = setup_test_graph_with_ranker(Ranker::TightTree);

    // Create path: a -> b -> c -> d -> h
    let a = g.add_node(NodeLabel::default());
    let b = g.add_node(NodeLabel::default());
    let c = g.add_node(NodeLabel::default());
    let d = g.add_node(NodeLabel::default());
    let h = g.add_node(NodeLabel::default());

    // Create path: a -> e -> g -> h
    let e = g.add_node(NodeLabel::default());
    let g_node = g.add_node(NodeLabel::default());

    // Create path: a -> f -> g
    let f = g.add_node(NodeLabel::default());

    // Add edges with minlen
    g.add_edge(
        Edge::new(a, b),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(b, c),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(c, d),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(d, h),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(a, e),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(e, g_node),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(g_node, h),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(a, f),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );
    g.add_edge(
        Edge::new(f, g_node),
        EdgeLabel {
            minlen: 1,
            ..Default::default()
        },
    );

    rank(&mut g);

    // 标准化排名，确保所有rank都是非负数
    normalize_ranks(&mut g);

    // Check that all edges respect minlen
    for edge in g.edges() {
        if let (Some(v_label), Some(w_label)) =
            (g.node_label(edge.source), g.node_label(edge.target))
        {
            if let (Some(v_rank), Some(w_rank)) = (v_label.rank, w_label.rank) {
                if let Some(edge_label) = g.edge_label(&edge) {
                    let minlen = edge_label.minlen;
                    assert!(w_rank - v_rank >= minlen as i32);
                }
            }
        }
    }
}

#[test]
fn test_can_rank_single_node_graph() {
    let mut g = setup_test_graph();
    let a = g.add_node(NodeLabel::default());

    rank(&mut g);

    // 标准化排名，确保所有rank都是非负数
    normalize_ranks(&mut g);

    if let Some(label) = g.node_label(a) {
        assert_eq!(label.rank, Some(0));
    }
}
