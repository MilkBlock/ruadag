use dagviz::graph::Graph;
use dagviz::graph::NodeIndex;
use dagviz::position::bk::*;
use dagviz::types::{Dummy, Edge, EdgeLabel, NodeLabel};
use dagviz::util::build_layer_matrix;
use indexmap::IndexMap;

#[test]
fn test_find_type1_conflicts_does_not_mark_edges_with_no_conflict() {
    let mut g = Graph::new();
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

    g.add_edge(Edge::new(a, c), EdgeLabel::default());
    g.add_edge(Edge::new(b, d), EdgeLabel::default());

    let mut bk = BrandesKoepf::new(g);
    bk.build_layers();
    bk.find_type1_conflicts();

    assert!(!bk.has_conflict(a, c));
    assert!(!bk.has_conflict(b, d));
}

#[test]
fn test_find_type1_conflicts_does_not_mark_type0_conflicts() {
    let mut g = Graph::new();
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

    // Set up crossing
    g.add_edge(Edge::new(a, d), EdgeLabel::default());
    g.add_edge(Edge::new(b, c), EdgeLabel::default());

    let mut bk = BrandesKoepf::new(g);
    bk.build_layers();
    bk.find_type1_conflicts();

    assert!(!bk.has_conflict(a, d));
    assert!(!bk.has_conflict(b, c));
}

#[test]
fn test_find_type1_conflicts_does_not_mark_type0_conflicts_with_dummy() {
    let mut g = Graph::new();
    let a = g.add_node(NodeLabel {
        rank: Some(0),
        order: Some(0),
        dummy: Some(Dummy::Border),
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

    // Set up crossing
    g.add_edge(Edge::new(a, d), EdgeLabel::default());
    g.add_edge(Edge::new(b, c), EdgeLabel::default());

    let mut bk = BrandesKoepf::new(g);
    bk.build_layers();
    bk.find_type1_conflicts();

    assert!(!bk.has_conflict(a, d));
    assert!(!bk.has_conflict(b, c));
}

#[test]
fn test_find_type1_conflicts_does_mark_type1_conflicts() {
    let mut g = Graph::new();
    let a = g.add_node(NodeLabel {
        rank: Some(0),
        order: Some(0),
        ..Default::default()
    });
    let b = g.add_node(NodeLabel {
        rank: Some(0),
        order: Some(1),
        dummy: Some(Dummy::Border),
        ..Default::default()
    });
    let c = g.add_node(NodeLabel {
        rank: Some(1),
        order: Some(0),
        dummy: Some(Dummy::Border),
        ..Default::default()
    });
    let d = g.add_node(NodeLabel {
        rank: Some(1),
        order: Some(1),
        dummy: Some(Dummy::Border),
        ..Default::default()
    });

    // Set up crossing
    g.add_edge(Edge::new(a, d), EdgeLabel::default());
    g.add_edge(Edge::new(b, c), EdgeLabel::default());

    let mut bk = BrandesKoepf::new(g);
    bk.build_layers();
    bk.find_type1_conflicts();

    assert!(bk.has_conflict(a, d));
    assert!(!bk.has_conflict(b, c));
}

#[test]
fn test_find_type1_conflicts_does_not_mark_type2_conflicts() {
    let mut g = Graph::new();
    let a = g.add_node(NodeLabel {
        rank: Some(0),
        order: Some(0),
        dummy: Some(Dummy::Border),
        ..Default::default()
    });
    let b = g.add_node(NodeLabel {
        rank: Some(0),
        order: Some(1),
        dummy: Some(Dummy::Border),
        ..Default::default()
    });
    let c = g.add_node(NodeLabel {
        rank: Some(1),
        order: Some(0),
        dummy: Some(Dummy::Border),
        ..Default::default()
    });
    let d = g.add_node(NodeLabel {
        rank: Some(1),
        order: Some(1),
        dummy: Some(Dummy::Border),
        ..Default::default()
    });

    // Set up crossing
    g.add_edge(Edge::new(a, d), EdgeLabel::default());
    g.add_edge(Edge::new(b, c), EdgeLabel::default());

    let mut bk = BrandesKoepf::new(g);
    bk.build_layers();
    bk.find_type1_conflicts();

    assert!(!bk.has_conflict(a, d));
    assert!(!bk.has_conflict(b, c));
}

#[test]
fn test_find_type2_conflicts() {
    let mut g = Graph::new();
    let a = g.add_node(NodeLabel {
        rank: Some(0),
        order: Some(0),
        dummy: Some(Dummy::Border),
        ..Default::default()
    });
    let b = g.add_node(NodeLabel {
        rank: Some(0),
        order: Some(1),
        dummy: Some(Dummy::Border),
        ..Default::default()
    });
    let c = g.add_node(NodeLabel {
        rank: Some(1),
        order: Some(0),
        dummy: Some(Dummy::Border),
        ..Default::default()
    });
    let d = g.add_node(NodeLabel {
        rank: Some(1),
        order: Some(1),
        dummy: Some(Dummy::Border),
        ..Default::default()
    });

    // Set up crossing
    g.add_edge(Edge::new(a, d), EdgeLabel::default());
    g.add_edge(Edge::new(b, c), EdgeLabel::default());

    let mut bk = BrandesKoepf::new(g);
    bk.build_layers();
    bk.find_type2_conflicts();

    // Type 2 conflicts should be detected for dummy nodes
    assert!(bk.has_conflict(a, d));
    assert!(bk.has_conflict(b, c));
}

#[test]
fn test_vertical_alignment() {
    let mut g = Graph::new();
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

    g.add_edge(Edge::new(a, c), EdgeLabel::default());
    g.add_edge(Edge::new(b, d), EdgeLabel::default());

    let layering = build_layer_matrix(&g);
    let mut bk = BrandesKoepf::new(g);
    bk.build_layers();

    let neighbor_fn = |g: &Graph, v: NodeIndex| g.predecessors(v).collect::<Vec<_>>();
    let alignment = bk.vertical_alignment(&layering, &neighbor_fn);

    // Check that alignment is created
    assert!(!alignment.root.is_empty());
    assert!(!alignment.align.is_empty());
}

#[test]
fn test_horizontal_compaction() {
    let mut g = Graph::new();
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

    g.add_edge(Edge::new(a, c), EdgeLabel::default());
    g.add_edge(Edge::new(b, d), EdgeLabel::default());

    let layering = build_layer_matrix(&g);
    let mut bk = BrandesKoepf::new(g);
    bk.build_layers();

    let neighbor_fn = |g: &Graph, v: NodeIndex| g.predecessors(v).collect::<Vec<_>>();
    let alignment = bk.vertical_alignment(&layering, &neighbor_fn);
    let xs = bk.horizontal_compaction(&layering, &alignment, false);

    // Check that coordinates are assigned
    assert!(!xs.is_empty());
    for node_id in bk.graph().node_indices() {
        assert!(xs.contains_key(&node_id));
    }
}

#[test]
fn test_align_coordinates() {
    let mut g = Graph::new();
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

    let mut xss = IndexMap::new();
    xss.insert("ul".to_string(), IndexMap::from([(a, 0.0), (b, 100.0)]));
    xss.insert("ur".to_string(), IndexMap::from([(a, 200.0), (b, 300.0)]));

    let bk = BrandesKoepf::new(Graph::new());
    let align_to = xss.get("ul").unwrap().clone();
    bk.align_coordinates(&mut xss, &align_to);

    // Check that coordinates are aligned
    let ul_coords = xss.get("ul").unwrap();
    let ur_coords = xss.get("ur").unwrap();

    // After alignment, both should have the same minimum coordinate
    let ul_min = ul_coords.values().fold(f64::INFINITY, |a, &b| a.min(b));
    let ur_min = ur_coords.values().fold(f64::INFINITY, |a, &b| a.min(b));
    assert!((ul_min - ur_min).abs() < 1e-6);
}

#[test]
fn test_balance() {
    let mut g = Graph::new();
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

    let mut xss = IndexMap::new();
    xss.insert("ul".to_string(), IndexMap::from([(a, 0.0), (b, 100.0)]));
    xss.insert("ur".to_string(), IndexMap::from([(a, 200.0), (b, 300.0)]));

    let bk = BrandesKoepf::new(Graph::new());
    let balanced = bk.balance(&xss, None);

    // Check that balanced coordinates are between ul and ur
    assert!(balanced.get(&a).unwrap() > xss.get("ul").unwrap().get(&a).unwrap());
    assert!(balanced.get(&a).unwrap() < xss.get("ur").unwrap().get(&a).unwrap());
    assert!(balanced.get(&b).unwrap() > xss.get("ul").unwrap().get(&b).unwrap());
    assert!(balanced.get(&b).unwrap() < xss.get("ur").unwrap().get(&b).unwrap());
}

#[test]
fn test_find_smallest_width_alignment() {
    let mut g = Graph::new();
    let a = g.add_node(NodeLabel::default());
    let b = g.add_node(NodeLabel::default());

    let mut xss = IndexMap::new();
    xss.insert("ul".to_string(), IndexMap::from([(a, 0.0), (b, 100.0)]));
    xss.insert(
        "ur".to_string(),
        IndexMap::from([
            (a, 0.0),
            (b, 50.0), // Smaller width
        ]),
    );

    let bk = BrandesKoepf::new(g);
    let alignment = bk.find_smallest_width_alignment(&xss);
    assert!(alignment.is_some());
    // Check that the returned alignment has the smaller width
    let returned_alignment = alignment.unwrap();
    assert_eq!(returned_alignment.get(&b).unwrap(), &50.0);
}

#[test]
fn test_position_x() {
    let mut g = Graph::new();
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

    g.add_edge(Edge::new(a, c), EdgeLabel::default());
    g.add_edge(Edge::new(b, d), EdgeLabel::default());

    let result = g.compute_bk_positions();
    let xs: IndexMap<NodeIndex, f64> = result
        .positions
        .iter()
        .map(|(node, pos)| (*node, pos.position))
        .collect();

    // Check that all nodes have coordinates
    for node_id in g.node_indices() {
        assert!(xs.iter().any(|(id, _)| *id == node_id));
    }

    // Check that coordinates are reasonable
    for (_, x) in &xs {
        assert!(x.is_finite());
    }
}
