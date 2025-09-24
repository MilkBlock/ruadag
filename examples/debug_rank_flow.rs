use dagviz::graph::Graph;
use dagviz::graph::NodeIndex;
use dagviz::layout;
use dagviz::types::*;
use dagviz::util;

fn main() {
    println!("=== Rust Rank Flow Debug 脚本 ===");

    // 创建测试图
    let mut g = create_test_graph();

    print_ranks(&g, "初始状态");

    // 手动执行各个步骤
    println!("\n1. 执行无环化...");
    dagviz::acyclic::Acyclic::run(&mut g);
    print_ranks(&g, "无环化后");

    println!("\n2. 执行rank...");
    dagviz::rank::rank(&mut g);
    print_ranks(&g, "rank后");

    println!("\n3. 执行normalize ranks...");
    util::normalize_ranks(&mut g);
    print_ranks(&g, "normalize后");

    println!("\n4. 执行remove empty ranks...");
    util::remove_empty_ranks(&mut g);
    print_ranks(&g, "remove empty ranks后");

    // 检查结果
    let ranks: Vec<i32> = g
        .node_indices()
        .filter_map(|node_id| g.node_label(node_id).and_then(|node| node.rank))
        .collect();
    let unique_ranks: std::collections::HashSet<i32> = ranks.iter().cloned().collect();

    println!("\n=== 最终结果分析 ===");
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
    let edge_ab = Edge::new(a, b);
    let edge_bc = Edge::new(b, c);
    let edge_cd = Edge::new(c, d);
    let edge_de = Edge::new(d, e);

    g.add_edge(
        edge_ab,
        EdgeLabel {
            minlen: 1,
            weight: 1.0,
            ..Default::default()
        },
    );

    g.add_edge(
        edge_bc,
        EdgeLabel {
            minlen: 1,
            weight: 1.0,
            ..Default::default()
        },
    );

    g.add_edge(
        edge_cd,
        EdgeLabel {
            minlen: 1,
            weight: 1.0,
            ..Default::default()
        },
    );

    g.add_edge(
        edge_de,
        EdgeLabel {
            minlen: 1,
            weight: 1.0,
            ..Default::default()
        },
    );

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
}
