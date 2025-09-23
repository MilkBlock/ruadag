use dagviz::graph::Graph;
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

    let layering = build_layer_matrix(&g);
    let conflicts = find_type1_conflicts(&g);

    assert!(!has_conflict(&g, a, c));
    assert!(!has_conflict(&g, b, d));
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

    let layering = build_layer_matrix(&g);
    let conflicts = find_type1_conflicts(&g);

    assert!(!has_conflict(&g, a, d));
    assert!(!has_conflict(&g, b, c));
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

    let layering = build_layer_matrix(&g);
    let conflicts = find_type1_conflicts(&g);

    assert!(!has_conflict(&g, a, d));
    assert!(!has_conflict(&g, b, c));
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

    let layering = build_layer_matrix(&g);
    let conflicts = find_type1_conflicts(&g);

    assert!(has_conflict(&g, a, d));
    assert!(!has_conflict(&g, b, c));
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

    let layering = build_layer_matrix(&g);
    let conflicts = find_type1_conflicts(&g);

    assert!(!has_conflict(&g, a, d));
    assert!(!has_conflict(&g, b, c));
}

#[test]
fn test_find_type2_conflicts() {
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

    let layering = build_layer_matrix(&g);
    let conflicts = find_type2_conflicts(&g);

    // Type 2 conflicts should be detected
    assert!(has_conflict(&g, a, d));
    assert!(has_conflict(&g, b, c));
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
    let conflicts = IndexMap::new(); // No conflicts for this simple case
    let alignment = vertical_alignment(&g, &layering, &conflicts, "ul");

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
    let conflicts = IndexMap::new();
    let alignment = vertical_alignment(&g, &layering, &conflicts, "ul");
    let xs = horizontal_compaction(&g, &layering, &alignment, false);

    // Check that coordinates are assigned
    assert!(!xs.is_empty());
    for node_id in g.node_indices() {
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

    align_coordinates(&mut xss, "ul");

    // Check that coordinates are aligned
    let ul_coords = xss.get("ul").unwrap();
    let ur_coords = xss.get("ur").unwrap();

    assert!(ul_coords.get(&a).unwrap() < ur_coords.get(&a).unwrap());
    assert!(ul_coords.get(&b).unwrap() < ur_coords.get(&b).unwrap());
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

    let balanced = balance(&xss, None);

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

    let alignment = find_smallest_width_alignment(&xss);
    assert_eq!(alignment, Some("ur".to_string()));
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

    let xs = position_x(&g);

    // Check that all nodes have coordinates
    for node_id in g.node_indices() {
        assert!(xs.iter().any(|(id, _)| *id == node_id));
    }

    // Check that coordinates are reasonable
    for (_, x) in &xs {
        assert!(x.is_finite());
    }
}
