use petgraph::graph::{NodeIndex, Graph as PetGraph};

fn main() {
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
}




