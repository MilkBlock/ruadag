//! 构建层级图

use indexmap::IndexMap;

use crate::graph::Graph;
use crate::graph::NodeIndex;
use crate::types::Edge;

/// 层级图结构，包含层级图和节点映射
pub struct LayerGraph {
    pub graph: Graph,
    pub node_mapping: IndexMap<NodeIndex, NodeIndex>, // 原始节点ID -> 层级图节点ID
}

/// 构建指定排名的层级图
pub fn build_layer_graph(graph: &Graph, rank: i32, relationship: &str) -> LayerGraph {
    let mut layer_graph = Graph::new();
    let mut node_mapping = IndexMap::default();

    // 添加该层级的所有节点
    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            if let Some(node_rank) = label.rank {
                if node_rank == rank {
                    let new_node_id = layer_graph.add_node(label.clone());
                    node_mapping.insert(node_id, new_node_id);
                }
            }
        }
    }

    // 根据关系类型添加边
    match relationship {
        "in_edges" => add_in_edges_to_layer_graph(graph, &mut layer_graph, &node_mapping, rank),
        "out_edges" => add_out_edges_to_layer_graph(graph, &mut layer_graph, &node_mapping, rank),
        _ => {
            // 默认添加所有相关边
            add_all_edges_to_layer_graph(graph, &mut layer_graph, &node_mapping, rank);
        }
    }

    LayerGraph {
        graph: layer_graph,
        node_mapping,
    }
}

/// 添加入边到层级图
fn add_in_edges_to_layer_graph(
    graph: &Graph,
    layer_graph: &mut Graph,
    node_mapping: &IndexMap<NodeIndex, NodeIndex>,
    rank: i32,
) {
    // 收集所有需要添加的边，避免借用冲突
    let mut edges_to_add = Vec::new();

    // 遍历层级图中的每个节点
    for (original_node_id, layer_node_id) in node_mapping {
        // 在原始图中查找该节点的入边
        for edge in graph.in_edges(*original_node_id) {
            if let Some(source_label) = graph.node_label(edge.source) {
                if let Some(source_rank) = source_label.rank {
                    if source_rank == rank - 1 {
                        // 检查源节点是否也在层级图中
                        if let Some(&source_layer_id) = node_mapping.get(&edge.source) {
                            if let Some(edge_label) = graph.edge_label(&edge) {
                                // 创建新的边，使用层级图中的节点ID
                                let new_edge = Edge::new(source_layer_id, *layer_node_id);
                                edges_to_add.push((new_edge, edge_label.clone()));
                            }
                        }
                    }
                }
            }
        }
    }

    // 添加收集到的边
    for (edge, edge_label) in edges_to_add {
        let _ = layer_graph.add_edge(edge, edge_label);
    }
}

/// 添加出边到层级图
fn add_out_edges_to_layer_graph(
    graph: &Graph,
    layer_graph: &mut Graph,
    node_mapping: &IndexMap<NodeIndex, NodeIndex>,
    rank: i32,
) {
    // 收集所有需要添加的边，避免借用冲突
    let mut edges_to_add = Vec::new();

    // 遍历层级图中的每个节点
    for (original_node_id, layer_node_id) in node_mapping {
        // 在原始图中查找该节点的出边
        for edge in graph.out_edges(*original_node_id) {
            if let Some(target_label) = graph.node_label(edge.target) {
                if let Some(target_rank) = target_label.rank {
                    if target_rank == rank + 1 {
                        // 检查目标节点是否也在层级图中
                        if let Some(&target_layer_id) = node_mapping.get(&edge.target) {
                            if let Some(edge_label) = graph.edge_label(&edge) {
                                // 创建新的边，使用层级图中的节点ID
                                let new_edge = Edge::new(*layer_node_id, target_layer_id);
                                edges_to_add.push((new_edge, edge_label.clone()));
                            }
                        }
                    }
                }
            }
        }
    }

    // 添加收集到的边
    for (edge, edge_label) in edges_to_add {
        let _ = layer_graph.add_edge(edge, edge_label);
    }
}

/// 添加所有相关边到层级图
fn add_all_edges_to_layer_graph(
    graph: &Graph,
    layer_graph: &mut Graph,
    node_mapping: &IndexMap<NodeIndex, NodeIndex>,
    rank: i32,
) {
    // 收集所有需要添加的边，避免借用冲突
    let mut edges_to_add = Vec::new();

    // 遍历层级图中的每个节点
    for (original_node_id, layer_node_id) in node_mapping {
        // 添加入边
        for edge in graph.in_edges(*original_node_id) {
            if let Some(source_label) = graph.node_label(edge.source) {
                if let Some(source_rank) = source_label.rank {
                    if source_rank == rank - 1 {
                        // 检查源节点是否也在层级图中
                        if let Some(&source_layer_id) = node_mapping.get(&edge.source) {
                            if let Some(edge_label) = graph.edge_label(&edge) {
                                // 创建新的边，使用层级图中的节点ID
                                let new_edge = Edge::new(source_layer_id, *layer_node_id);
                                edges_to_add.push((new_edge, edge_label.clone()));
                            }
                        }
                    }
                }
            }
        }

        // 添加出边
        for edge in graph.out_edges(*original_node_id) {
            if let Some(target_label) = graph.node_label(edge.target) {
                if let Some(target_rank) = target_label.rank {
                    if target_rank == rank + 1 {
                        // 检查目标节点是否也在层级图中
                        if let Some(&target_layer_id) = node_mapping.get(&edge.target) {
                            if let Some(edge_label) = graph.edge_label(&edge) {
                                // 创建新的边，使用层级图中的节点ID
                                let new_edge = Edge::new(*layer_node_id, target_layer_id);
                                edges_to_add.push((new_edge, edge_label.clone()));
                            }
                        }
                    }
                }
            }
        }
    }

    // 添加收集到的边
    for (edge, edge_label) in edges_to_add {
        let _ = layer_graph.add_edge(edge, edge_label);
    }
}

/// 构建层级图的简化版本
pub fn build_simple_layer_graph(graph: &Graph, rank: i32) -> Graph {
    let mut layer_graph = Graph::new();

    // 只添加节点，不添加边
    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            if let Some(node_rank) = label.rank {
                if node_rank == rank {
                    layer_graph.add_node(label.clone());
                }
            }
        }
    }

    layer_graph
}

/// 构建层级图的连通分量
pub fn build_layer_graph_components(graph: &Graph, rank: i32) -> Vec<Graph> {
    let layer_graph = build_layer_graph(graph, rank, "all");
    let components = find_connected_components(&layer_graph.graph);

    let mut component_graphs = Vec::new();

    for component in components {
        let mut comp_graph = Graph::new();

        for node_id in component {
            if let Some(label) = layer_graph.graph.node_label(node_id) {
                comp_graph.add_node(label.clone());
            }
        }

        // 添加组件内的边
        for edge in layer_graph.graph.edges() {
            if comp_graph.has_node(edge.source) && comp_graph.has_node(edge.target) {
                if let Some(edge_label) = layer_graph.graph.edge_label(&edge) {
                    let _ = comp_graph.add_edge(edge, edge_label.clone());
                }
            }
        }

        component_graphs.push(comp_graph);
    }

    component_graphs
}

/// 查找连通分量
fn find_connected_components(graph: &Graph) -> Vec<Vec<NodeIndex>> {
    let mut visited = indexmap::IndexSet::new();
    let mut components = Vec::new();

    for node_id in graph.node_indices() {
        if !visited.contains(&node_id) {
            let mut component = Vec::new();
            dfs_component(graph, node_id, &mut visited, &mut component);
            if !component.is_empty() {
                components.push(component);
            }
        }
    }

    components
}

/// DFS查找连通分量
fn dfs_component(
    graph: &Graph,
    node: NodeIndex,
    visited: &mut indexmap::IndexSet<NodeIndex>,
    component: &mut Vec<NodeIndex>,
) {
    if visited.contains(&node) {
        return;
    }

    visited.insert(node);
    component.push(node);

    for neighbor in graph.successors(node) {
        dfs_component(graph, neighbor, visited, component);
    }

    for neighbor in graph.predecessors(node) {
        dfs_component(graph, neighbor, visited, component);
    }
}

/// 构建层级图的子图
pub fn build_layer_subgraph(
    graph: &Graph,
    rank: i32,
    node_filter: &dyn Fn(NodeIndex) -> bool,
) -> Graph {
    let mut layer_graph = Graph::new();

    // 添加过滤后的节点
    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            if let Some(node_rank) = label.rank {
                if node_rank == rank && node_filter(node_id) {
                    layer_graph.add_node(label.clone());
                }
            }
        }
    }

    // 收集所有需要添加的边，避免借用冲突
    let mut edges_to_add = Vec::new();

    for node_id in layer_graph.node_indices() {
        for edge in graph.in_edges(node_id) {
            if layer_graph.has_node(edge.source) {
                if let Some(edge_label) = graph.edge_label(&edge) {
                    edges_to_add.push((edge.clone(), edge_label.clone()));
                }
            }
        }

        for edge in graph.out_edges(node_id) {
            if layer_graph.has_node(edge.target) {
                if let Some(edge_label) = graph.edge_label(&edge) {
                    edges_to_add.push((edge.clone(), edge_label.clone()));
                }
            }
        }
    }

    // 添加收集到的边
    for (edge, edge_label) in edges_to_add {
        let _ = layer_graph.add_edge(edge, edge_label);
    }

    layer_graph
}

#[cfg(test)]
mod tests {
    use crate::{Edge, EdgeLabel, NodeLabel};

    use super::*;

    #[test]
    fn test_build_layer_graph() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeLabel::default());
        let b = graph.add_node(NodeLabel::default());
        let c = graph.add_node(NodeLabel::default());

        // 设置排名
        if let Some(label_a) = graph.node_label_mut(a) {
            label_a.rank = Some(0);
        }
        if let Some(label_b) = graph.node_label_mut(b) {
            label_b.rank = Some(1);
        }
        if let Some(label_c) = graph.node_label_mut(c) {
            label_c.rank = Some(1);
        }

        // 添加边
        let edge_ab = Edge::new(a, b);
        let edge_ac = Edge::new(a, c);
        let _ = graph.add_edge(edge_ab, EdgeLabel::default());
        let _ = graph.add_edge(edge_ac, EdgeLabel::default());

        let layer_graph = build_layer_graph(&graph, 1, "in_edges");

        assert_eq!(layer_graph.graph.node_count(), 2);
        // 检查节点映射中是否包含原始节点
        assert!(layer_graph.node_mapping.contains_key(&b));
        assert!(layer_graph.node_mapping.contains_key(&c));
    }

    #[test]
    fn test_build_simple_layer_graph() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeLabel::default());
        let b = graph.add_node(NodeLabel::default());

        // 设置排名
        if let Some(label_a) = graph.node_label_mut(a) {
            label_a.rank = Some(0);
        }
        if let Some(label_b) = graph.node_label_mut(b) {
            label_b.rank = Some(1);
        }

        let layer_graph = build_simple_layer_graph(&graph, 1);

        assert_eq!(layer_graph.node_count(), 1);
        // 对于简单层级图，我们检查是否有任何节点（因为节点ID会不同）
        assert!(layer_graph.node_count() > 0);
    }
}
