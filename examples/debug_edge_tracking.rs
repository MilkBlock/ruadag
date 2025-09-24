use dagviz::graph::Graph;
use dagviz::graph::NodeIndex;
use dagviz::layout;
use dagviz::types::*;

fn main() {
    println!("=== 边追踪调试脚本 ===");

    // 创建测试图
    let mut g = create_test_graph();

    println!("\n=== 原始图信息 ===");
    print_graph_info(&g, "原始图");

    // 执行布局
    layout::layout(&mut g, Some(&LayoutOptions::default()));

    println!("\n=== 最终图信息 ===");
    print_graph_info(&g, "最终图");

    // 检查结果
    let ranks: Vec<i32> = g
        .node_indices()
        .filter_map(|node_id| g.node_label(node_id).and_then(|node| node.rank))
        .collect();
    let unique_ranks: std::collections::HashSet<i32> = ranks.iter().cloned().collect();

    println!("\n=== 结果分析 ===");
    println!("所有rank值: {:?}", ranks);
    println!("唯一rank值: {:?}", unique_ranks);
    println!("rank数量: {}", unique_ranks.len());

    if unique_ranks.len() == 1 {
        println!("❌ 问题: 所有节点都有相同的rank!");
    } else {
        println!("✅ 正常: 节点有不同的rank");
    }
}

fn create_test_graph() -> Graph {
    let mut g = Graph::new();

    // 设置图配置，使用NetworkSimplex
    let config = GraphConfig {
        ranker: Ranker::NetworkSimplex,
        ..Default::default()
    };
    g.set_config(config);

    // 添加节点
    let a = g.add_node(NodeLabel {
        label: Some("a".to_string()),
        width: 50.0,
        height: 50.0,
        ..Default::default()
    });

    let b = g.add_node(NodeLabel {
        label: Some("b".to_string()),
        width: 50.0,
        height: 50.0,
        ..Default::default()
    });

    let c = g.add_node(NodeLabel {
        label: Some("c".to_string()),
        width: 50.0,
        height: 50.0,
        ..Default::default()
    });

    let d = g.add_node(NodeLabel {
        label: Some("d".to_string()),
        width: 50.0,
        height: 50.0,
        ..Default::default()
    });

    let e = g.add_node(NodeLabel {
        label: Some("e".to_string()),
        width: 50.0,
        height: 50.0,
        ..Default::default()
    });

    // 添加边 - 注意这里我添加了更多的边来测试
    let edges = vec![
        Edge::new(a, b),
        Edge::new(b, c),
        Edge::new(c, d),
        Edge::new(d, e),
        Edge::new(a, c), // 跨层边
        Edge::new(c, e), // 跨层边
    ];

    println!("=== 添加边 ===");
    for (i, edge) in edges.iter().enumerate() {
        println!("添加边 {}: {:?} -> {:?}", i, edge.source, edge.target);
        g.add_edge(
            edge.clone(),
            EdgeLabel {
                minlen: 1,
                weight: 1.0,
                ..Default::default()
            },
        );
    }

    g
}

fn print_graph_info(g: &Graph, stage: &str) {
    println!("\n--- {} ---", stage);
    println!("节点数: {}", g.node_count());
    println!("边数: {}", g.edge_count());

    println!("节点信息:");
    for node_id in g.node_indices() {
        if let Some(node) = g.node_label(node_id) {
            println!(
                "  {:?}: rank={:?}, label={:?}",
                node_id, node.rank, node.label
            );
        }
    }

    println!("边信息:");
    for (i, edge) in g.edges().into_iter().enumerate() {
        if let Some(edge_label) = g.edge_label(&edge) {
            println!(
                "  边 {}: {:?} -> {:?}: minlen={}, weight={}",
                i, edge.source, edge.target, edge_label.minlen, edge_label.weight
            );
        }
    }
}
