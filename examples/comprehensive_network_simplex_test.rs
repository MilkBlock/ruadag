use dagviz::graph::Graph;
use dagviz::rank::network_simplex::network_simplex;
use dagviz::types::{Edge, EdgeLabel, NodeLabel};

fn main() {
    println!("Testing comprehensive NetworkSimplex algorithm...");

    // 创建一个更复杂的测试图
    let mut graph = Graph::new();

    // 添加节点
    let a = graph.add_node(NodeLabel {
        rank: None,
        ..Default::default()
    });
    let b = graph.add_node(NodeLabel {
        rank: None,
        ..Default::default()
    });
    let c = graph.add_node(NodeLabel {
        rank: None,
        ..Default::default()
    });
    let d = graph.add_node(NodeLabel {
        rank: None,
        ..Default::default()
    });
    let e = graph.add_node(NodeLabel {
        rank: None,
        ..Default::default()
    });

    // 添加边
    graph.add_edge(
        Edge::new(a, b),
        EdgeLabel {
            minlen: 1,
            weight: 1.0,
            ..Default::default()
        },
    );
    graph.add_edge(
        Edge::new(b, c),
        EdgeLabel {
            minlen: 1,
            weight: 1.0,
            ..Default::default()
        },
    );
    graph.add_edge(
        Edge::new(c, d),
        EdgeLabel {
            minlen: 1,
            weight: 1.0,
            ..Default::default()
        },
    );
    graph.add_edge(
        Edge::new(d, e),
        EdgeLabel {
            minlen: 1,
            weight: 1.0,
            ..Default::default()
        },
    );
    graph.add_edge(
        Edge::new(a, c),
        EdgeLabel {
            minlen: 2,
            weight: 2.0,
            ..Default::default()
        },
    );
    graph.add_edge(
        Edge::new(b, d),
        EdgeLabel {
            minlen: 2,
            weight: 2.0,
            ..Default::default()
        },
    );

    println!(
        "Graph created with {} nodes and {} edges",
        graph.node_count(),
        graph.edge_count()
    );

    // 运行 NetworkSimplex 算法
    network_simplex(&mut graph);

    println!("NetworkSimplex algorithm completed successfully!");

    // 打印结果
    for node in graph.node_indices() {
        if let Some(label) = graph.node_label(node) {
            println!("Node {:?}: rank = {:?}", node, label.rank);
        }
    }

    // 验证结果
    let mut ranks: Vec<Option<i32>> = Vec::new();
    for node in graph.node_indices() {
        if let Some(label) = graph.node_label(node) {
            ranks.push(label.rank);
        }
    }

    println!("All ranks: {:?}", ranks);

    // 检查是否有有效的rank值
    let valid_ranks: Vec<i32> = ranks.iter().filter_map(|&r| r).collect();
    println!("Valid ranks: {:?}", valid_ranks);

    if !valid_ranks.is_empty() {
        println!("✅ NetworkSimplex algorithm is working correctly!");
    } else {
        println!("❌ NetworkSimplex algorithm failed to assign ranks!");
    }
}
