use dagviz::*;

fn main() {
    // 创建一个简单的图
    let mut graph = Graph::new();

    // 添加节点
    let a = graph.add_node(NodeLabel {
        width: 50.0,
        height: 100.0,
        label: Some("Node A".to_string()),
        ..Default::default()
    });

    let b = graph.add_node(NodeLabel {
        width: 75.0,
        height: 200.0,
        label: Some("Node B".to_string()),
        ..Default::default()
    });

    let c = graph.add_node(NodeLabel {
        width: 60.0,
        height: 80.0,
        label: Some("Node C".to_string()),
        ..Default::default()
    });

    // 添加边
    let edge_ab = Edge::new(a, b);
    let edge_bc = Edge::new(b, c);
    let edge_ac = Edge::new(a, c);

    let _ = graph.add_edge(
        edge_ab,
        EdgeLabel {
            labelpos: LabelPosition::Center,
            ..Default::default()
        },
    );

    let _ = graph.add_edge(
        edge_bc,
        EdgeLabel {
            labelpos: LabelPosition::Right,
            ..Default::default()
        },
    );

    let _ = graph.add_edge(
        edge_ac,
        EdgeLabel {
            labelpos: LabelPosition::Left,
            ..Default::default()
        },
    );

    // 执行布局
    println!("执行图布局...");
    layout(&mut graph, None);

    // 输出结果
    println!("布局完成！");
    println!(
        "图尺寸: {}x{}",
        graph.config().width.unwrap_or(0.0),
        graph.config().height.unwrap_or(0.0)
    );

    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            println!(
                "节点 {:?}: 位置({:.2}, {:.2}), 尺寸({:.2}x{:.2})",
                node_id,
                label.x.unwrap_or(0.0),
                label.y.unwrap_or(0.0),
                label.width,
                label.height
            );
        }
    }

    for edge in graph.edges() {
        if let Some(edge_label) = graph.edge_label(&edge) {
            println!(
                "边 {:?}->{:?}: {} 个路径点",
                edge.source,
                edge.target,
                edge_label.points.len()
            );
        }
    }
}
