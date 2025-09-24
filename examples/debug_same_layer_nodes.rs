use dagviz::graph::Graph;
use dagviz::types::NodeLabel;
use dagviz::position::bk::BrandesKoepf;

fn main() {
    println!("=== 调试同一层多个节点的情况 ===");
    
    // 创建测试图 - 所有节点在同一层
    let mut graph = Graph::new();
    
    // 添加节点，设置相同的 rank 值
    let nodes: Vec<_> = (0..5).map(|i| {
        let node_id = graph.add_node(NodeLabel::default());
        println!("添加节点 {}: {:?}", i, node_id);
        node_id
    }).collect();
    
    // 设置相同的 rank 值：所有节点都在 rank 0
    for (i, node_id) in nodes.iter().enumerate() {
        if let Some(label) = graph.node_label_mut(*node_id) {
            label.rank = Some(0); // 所有节点在同一层
            label.order = Some(i); // 设置顺序
        }
    }
    
    println!("\n=== 原图节点设置 ===");
    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            println!("节点 {:?}: rank={:?}, order={:?}", node_id, label.rank, label.order);
        }
    }
    
    println!("\n=== 创建 Brandes-Köpf 算法实例 ===");
    let mut bk = BrandesKoepf::new(graph);
    
    println!("\n=== Brandes-Köpf 内部 ranks 映射 ===");
    for (node_id, rank) in bk.get_ranks() {
        println!("节点 {:?}: rank={}", node_id, rank);
    }
    
    println!("\n=== 构建层级 ===");
    bk.build_layers();
    
    println!("\n=== 层级结构 ===");
    for (i, layer) in bk.get_layers().iter().enumerate() {
        println!("层 {}: {} 个节点", i, layer.len());
        for node_id in layer {
            println!("  - {:?}", node_id);
        }
    }
    
    println!("\n=== 运行算法 ===");
    let result = bk.run();
    
    println!("\n=== 结果位置 ===");
    for (node_id, position) in &result.positions {
        println!("节点 {:?}: position={}, rank={}", 
                 node_id, position.position, position.rank);
    }
}
