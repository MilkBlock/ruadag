use dagviz::graph::Graph;
use dagviz::graph::NodeIndex;
use dagviz::types::*;
use dagviz::util;

fn main() {
    println!("=== Debug as_non_compound_graph ===");
    
    // 创建测试图
    let mut g = create_test_graph();
    
    print_ranks(&g, "原始图");
    
    // 执行rank计算
    dagviz::rank::rank(&mut g);
    print_ranks(&g, "rank计算后");
    
    // 执行normalize ranks
    util::normalize_ranks(&mut g);
    print_ranks(&g, "normalize后");
    
    // 执行as_non_compound_graph
    println!("\n执行 as_non_compound_graph...");
    let (simplified_graph, old_to_new) = util::as_non_compound_graph(&g);
    
    print_ranks(&simplified_graph, "简化图");
    
    // 检查映射
    println!("\n映射关系:");
    for (old_id, new_id) in &old_to_new {
        let old_rank = g.node_label(*old_id).and_then(|n| n.rank);
        let new_rank = simplified_graph.node_label(*new_id).and_then(|n| n.rank);
        println!("  {:?} -> {:?}: rank {} -> {}", old_id, new_id, 
                 old_rank.map_or("None".to_string(), |r| r.to_string()),
                 new_rank.map_or("None".to_string(), |r| r.to_string()));
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
    
    g.add_edge(edge_ab, EdgeLabel {
        minlen: 1,
        weight: 1.0,
        ..Default::default()
    });
    
    g.add_edge(edge_bc, EdgeLabel {
        minlen: 1,
        weight: 1.0,
        ..Default::default()
    });
    
    g.add_edge(edge_cd, EdgeLabel {
        minlen: 1,
        weight: 1.0,
        ..Default::default()
    });
    
    g.add_edge(edge_de, EdgeLabel {
        minlen: 1,
        weight: 1.0,
        ..Default::default()
    });
    
    g
}

fn print_ranks(g: &Graph, stage: &str) {
    println!("\n--- {} ---", stage);
    println!("节点rank信息:");
    for node_id in g.node_indices() {
        if let Some(node) = g.node_label(node_id) {
            let label = node.label.as_deref().unwrap_or("Unknown");
            println!("  {} ({:?}): rank={:?}", label, node_id, node.rank);
        }
    }
}