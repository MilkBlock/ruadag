use dagviz::graph::Graph;
use dagviz::graph::NodeIndex;
use dagviz::layout;
use dagviz::types::*;

fn main() {
    println!("=== Rust Network Simplex Debug 脚本 ===");

    // 创建测试图
    let mut g = create_test_graph();

    println!("\n=== 测试 Network Simplex 算法 ===");
    println!("原始图配置:");
    println!("  ranker: {:?}", g.config().ranker);
    println!("  rankdir: {:?}", g.config().rankdir);

    print_ranks(&g, "执行layout前");

    // 执行布局
    layout::layout(&mut g, Some(&LayoutOptions::default()));

    print_ranks(&g, "执行layout后");

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

    // 添加边
    [
        Edge::new(a, b),
        Edge::new(b, c),
        Edge::new(c, d),
        Edge::new(d, e),
        Edge::new(a, c),
        Edge::new(c, e),
    ]
    .into_iter()
    .for_each(|edge| add_edge(&mut g, edge));
    fn add_edge(g: &mut Graph, edge: Edge) {
        {
            g.add_edge(
                edge,
                EdgeLabel {
                    minlen: 1,
                    weight: 1.0,
                    ..Default::default()
                },
            );
        }
    }

    g
}

fn print_ranks(g: &Graph, stage: &str) {
    println!("\n--- {} ---", stage);
    println!("节点rank信息:");
    for node_id in g.node_indices() {
        if let Some(node) = g.node_label(node_id) {
            println!("  {:?}: rank={:?}", node_id, node.rank);
        }
    }

    println!("边信息:");
    for edge in g.edges() {
        if let Some(edge_label) = g.edge_label(&edge) {
            println!(
                "  {:?} -> {:?}: minlen={}, weight={}",
                edge.source, edge.target, edge_label.minlen, edge_label.weight
            );
        }
    }
}
