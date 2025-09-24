use dagviz::graph::Graph;
use dagviz::rank::network_simplex::network_simplex;
use dagviz::types::{Edge, EdgeLabel, NodeLabel};

fn main() {
    println!("Testing NetworkSimplex algorithm...");

    // 创建一个简单的测试图
    let mut graph = Graph::new();

    // 添加节点
    let a = graph.add_node(NodeLabel {
        rank: Some(0),
        ..Default::default()
    });
    let b = graph.add_node(NodeLabel {
        rank: Some(0),
        ..Default::default()
    });
    let c = graph.add_node(NodeLabel {
        rank: Some(0),
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
}
