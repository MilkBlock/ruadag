//! 测试 petgraph 的行为

use petgraph::graph::{NodeIndex, Graph as PetGraph};

#[test]
fn test_petgraph_remove_node_behavior() {
    let mut graph = PetGraph::<(), (), petgraph::Directed>::new();
    
    let a = graph.add_node(());
    let b = graph.add_node(());
    let c = graph.add_node(());
    
    println!("添加节点后:");
    println!("  a: {:?}, has_node: {}", a, graph.node_weight(a).is_some());
    println!("  b: {:?}, has_node: {}", b, graph.node_weight(b).is_some());
    println!("  c: {:?}, has_node: {}", c, graph.node_weight(c).is_some());
    
    // 删除节点 b
    let removed = graph.remove_node(b);
    println!("\n删除节点 b 后:");
    println!("  删除的节点: {:?}", removed);
    println!("  a: {:?}, has_node: {}", a, graph.node_weight(a).is_some());
    println!("  b: {:?}, has_node: {}", b, graph.node_weight(b).is_some());
    println!("  c: {:?}, has_node: {}", c, graph.node_weight(c).is_some());
    
    // 检查节点索引是否仍然有效
    println!("\n节点索引值:");
    println!("  a.index(): {}", a.index());
    println!("  b.index(): {}", b.index());
    println!("  c.index(): {}", c.index());
    
    // 验证删除的节点确实不存在
    assert!(graph.node_weight(a).is_some());
    // 注意：petgraph 的 remove_node 行为可能不是我们期望的
    // 让我们先看看实际的行为
    println!("删除后的实际行为:");
    println!("  a 存在: {}", graph.node_weight(a).is_some());
    println!("  b 存在: {}", graph.node_weight(b).is_some());
    println!("  c 存在: {}", graph.node_weight(c).is_some());
    
    // 检查图的节点数量
    println!("图的节点数量: {}", graph.node_count());
    
    // 检查所有节点
    for node in graph.node_indices() {
        println!("  节点: {:?}, 权重: {:?}", node, graph.node_weight(node));
    }
}
