// 测试JavaScript和Rust节点配置的语义等价性
use dagviz::graph::Graph;
use dagviz::layout::layout;
use dagviz::types::{Edge, EdgeLabel, GraphConfig, LabelPosition, NodeLabel};

fn main() {
    println!("=== JavaScript和Rust节点配置对比测试 ===\n");

    // 测试1: 单节点布局
    println!("测试1: 单节点布局");
    let mut g1 = Graph::new();
    let node_a = g1.add_node(NodeLabel {
        width: 50.0,
        height: 100.0,
        ..Default::default()
    });

    layout(&mut g1, None);

    let node_label = g1.node_label(node_a).unwrap();
    println!(
        "Rust结果: x={:?}, y={:?}, width={}, height={}",
        node_label.x, node_label.y, node_label.width, node_label.height
    );

    // 测试2: 两个节点在同一层级
    println!("\n测试2: 两个节点在同一层级");
    let mut g2 = Graph::new();
    let mut config = GraphConfig::default();
    config.node_sep = 200.0;
    g2.set_config(config);

    let node_a2 = g2.add_node(NodeLabel {
        width: 50.0,
        height: 100.0,
        ..Default::default()
    });
    let node_b2 = g2.add_node(NodeLabel {
        width: 75.0,
        height: 200.0,
        ..Default::default()
    });

    layout(&mut g2, None);

    let node_a2_label = g2.node_label(node_a2).unwrap();
    let node_b2_label = g2.node_label(node_b2).unwrap();
    println!("节点a: x={:?}, y={:?}", node_a2_label.x, node_a2_label.y);
    println!("节点b: x={:?}, y={:?}", node_b2_label.x, node_b2_label.y);

    // 测试3: 两个节点通过边连接
    println!("\n测试3: 两个节点通过边连接");
    let mut g3 = Graph::new();
    let mut config = GraphConfig::default();
    config.rank_sep = 300.0;
    g3.set_config(config);

    let node_a3 = g3.add_node(NodeLabel {
        width: 50.0,
        height: 100.0,
        ..Default::default()
    });
    let node_b3 = g3.add_node(NodeLabel {
        width: 75.0,
        height: 200.0,
        ..Default::default()
    });
    g3.add_edge(Edge::new(node_a3, node_b3), EdgeLabel::default());

    layout(&mut g3, None);

    let node_a3_label = g3.node_label(node_a3).unwrap();
    let node_b3_label = g3.node_label(node_b3).unwrap();
    println!("节点a: x={:?}, y={:?}", node_a3_label.x, node_a3_label.y);
    println!("节点b: x={:?}, y={:?}", node_b3_label.x, node_b3_label.y);

    // 测试4: 带标签的边
    println!("\n测试4: 带标签的边");
    let mut g4 = Graph::new();
    let mut config = GraphConfig::default();
    config.rank_sep = 300.0;
    g4.set_config(config);

    let node_a4 = g4.add_node(NodeLabel {
        width: 50.0,
        height: 100.0,
        ..Default::default()
    });
    let node_b4 = g4.add_node(NodeLabel {
        width: 75.0,
        height: 200.0,
        ..Default::default()
    });
    g4.add_edge(
        Edge::new(node_a4, node_b4),
        EdgeLabel {
            width: 60.0,
            height: 70.0,
            labelpos: LabelPosition::Center,
            ..Default::default()
        },
    );

    layout(&mut g4, None);

    let node_a4_label = g4.node_label(node_a4).unwrap();
    let node_b4_label = g4.node_label(node_b4).unwrap();
    println!("节点a: x={:?}, y={:?}", node_a4_label.x, node_a4_label.y);
    println!("节点b: x={:?}, y={:?}", node_b4_label.x, node_b4_label.y);

    // 测试5: 默认节点配置
    println!("\n测试5: 默认节点配置");
    let mut g5 = Graph::new();
    let node_a5 = g5.add_node(NodeLabel::default()); // 使用默认配置

    layout(&mut g5, None);

    let node_a5_label = g5.node_label(node_a5).unwrap();
    println!(
        "默认节点a: x={:?}, y={:?}, width={}, height={}",
        node_a5_label.x, node_a5_label.y, node_a5_label.width, node_a5_label.height
    );

    println!("\n=== Rust测试完成 ===");
}
