use dagviz::graph::Graph;
use dagviz::layout;
use dagviz::types::*;
use dagviz::util::{build_layer_matrix, max_rank, min_rank};

fn main() {
    println!("=== longest_path 算法调试 ===");

    // 创建测试图
    let mut g = create_test_graph();

    println!("\n=== 执行layout前 ===");
    print_graph_info(&g, "原始图");

    // 执行布局
    layout::layout(&mut g, Some(&LayoutOptions::default()));

    println!("\n=== 执行layout后 ===");
    print_graph_info(&g, "最终图");
}

fn create_test_graph() -> Graph {
    let mut g = Graph::new();

    // 设置图配置，使用LongestPath算法
    let config = GraphConfig {
        ranker: Ranker::LongestPath,
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
    let edges = vec![
        Edge::new(a, b),
        Edge::new(b, c),
        Edge::new(c, d),
        Edge::new(d, e),
        Edge::new(a, c), // 跨层边
        Edge::new(c, e), // 跨层边
    ];

    for edge in edges {
        g.add_edge(
            edge,
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

    // 分析层级结构
    let min_rank_val = min_rank(g);
    let max_rank_val = max_rank(g);
    println!("Rank范围: {} 到 {}", min_rank_val, max_rank_val);
    
    let layers = build_layer_matrix(g);
    println!("层级结构:");
    for (i, layer) in layers.iter().enumerate() {
        let actual_rank = min_rank_val + i as i32;
        println!(
            "  层级 {} (rank={}): {} 个节点",
            i,
            actual_rank,
            layer.len()
        );
        for node_id in layer {
            if let Some(node) = g.node_label(*node_id) {
                println!("    {:?}: label={:?}", node_id, node.label);
            }
        }
    }
}
