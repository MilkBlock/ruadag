//! 调试petgraph行为

use crate::graph::Graph;
use crate::types::*;
use petgraph::graph::NodeIndex;

#[test]
fn debug_petgraph_behavior() {
    let mut graph = Graph::new();
    
    // 添加节点
    let a = graph.add_node(NodeLabel::default());
    let b = graph.add_node(NodeLabel::default());
    let c = graph.add_node(NodeLabel::default());
    
    println!("添加节点后:");
    println!("  a={:?}, has_node={}", a, graph.has_node(a));
    println!("  b={:?}, has_node={}", b, graph.has_node(b));
    println!("  c={:?}, has_node={}", c, graph.has_node(c));
    
    // 检查node_weight
    println!("node_weight检查:");
    println!("  a: {:?}", graph.petgraph().node_weight(a));
    println!("  b: {:?}", graph.petgraph().node_weight(b));
    println!("  c: {:?}", graph.petgraph().node_weight(c));
    
    // 删除节点b
    let removed = graph.remove_node(b);
    println!("删除节点b后:");
    println!("  removed: {:?}", removed);
    println!("  a={:?}, has_node={}", a, graph.has_node(a));
    println!("  b={:?}, has_node={}", b, graph.has_node(b));
    println!("  c={:?}, has_node={}", c, graph.has_node(c));
    
    // 再次检查node_weight
    println!("删除后node_weight检查:");
    println!("  a: {:?}", graph.petgraph().node_weight(a));
    println!("  b: {:?}", graph.petgraph().node_weight(b));
    println!("  c: {:?}", graph.petgraph().node_weight(c));
    
    // 检查节点数量
    println!("节点数量: {}", graph.node_count());
    
    // 检查所有节点索引
    println!("所有节点索引: {:?}", graph.node_indices().collect::<Vec<_>>());
}



