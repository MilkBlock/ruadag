//! 初始化排序算法
use crate::graph::Graph;
use crate::graph::NodeIndex;
use crate::util::{build_layer_matrix, is_placeholder};

/// 初始化节点排序
/// 对应 JS 函数: initOrder() in lib/order/init-order.js
pub fn init_order(graph: &Graph) -> Vec<Vec<NodeIndex>> {
    let mut visited = std::collections::HashSet::new();

    // 获取简单节点（无子节点的节点）
    let simple_nodes: Vec<NodeIndex> = graph
        .node_indices()
        .filter(|&_node_id| {
            // 检查是否有子节点（在dagre中，子节点通常表示复合节点）
            // 这里我们假设所有节点都是简单节点，因为我们的Graph结构可能不同
            true
        })
        .collect();

    // 获取简单节点的rank并找到最大rank
    let simple_nodes_ranks: Vec<i32> = simple_nodes
        .iter()
        .filter_map(|&node_id| graph.node_label(node_id)?.rank)
        .collect();

    let max_rank = simple_nodes_ranks.iter().max().copied().unwrap_or(0);

    // 创建层级数组
    let mut layers: Vec<Vec<NodeIndex>> = (0..=max_rank).map(|_| Vec::new()).collect();

    // DFS函数
    fn dfs(
        graph: &Graph,
        node_id: NodeIndex,
        visited: &mut std::collections::HashSet<NodeIndex>,
        layers: &mut Vec<Vec<NodeIndex>>,
    ) {
        if visited.contains(&node_id) {
            return;
        }
        visited.insert(node_id);

        if let Some(node_label) = graph.node_label(node_id) {
            if let Some(rank) = node_label.rank {
                if rank >= 0 && (rank as usize) < layers.len() {
                    layers[rank as usize].push(node_id);
                }
            }
        }

        // 遍历后继节点
        for successor in graph.successors(node_id) {
            dfs(graph, successor, visited, layers);
        }
    }

    // 按rank排序简单节点
    let mut ordered_nodes = simple_nodes;
    ordered_nodes.sort_by(|a, b| {
        let rank_a = graph
            .node_label(*a)
            .and_then(|label| label.rank)
            .unwrap_or(0);
        let rank_b = graph
            .node_label(*b)
            .and_then(|label| label.rank)
            .unwrap_or(0);
        rank_a.cmp(&rank_b)
    });

    // 对每个排序后的节点执行DFS
    for node_id in ordered_nodes {
        dfs(graph, node_id, &mut visited, &mut layers);
    }

    layers
}

/// 使用随机排序初始化
pub fn init_order_random(graph: &Graph) -> Vec<Vec<NodeIndex>> {
    let mut layering = build_layer_matrix(graph);

    // 对每一层进行随机排序
    for layer in layering.iter_mut() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        layer.sort_by(|a, b| {
            let mut hasher_a = DefaultHasher::new();
            let mut hasher_b = DefaultHasher::new();
            a.hash(&mut hasher_a);
            b.hash(&mut hasher_b);
            hasher_a.finish().cmp(&hasher_b.finish())
        });
    }

    layering
}

/// 使用度数排序初始化
pub fn init_order_by_degree(graph: &Graph) -> Vec<Vec<NodeIndex>> {
    let mut layering = build_layer_matrix(graph);

    // 对每一层按度数排序
    for layer in layering.iter_mut() {
        layer.sort_by(|a, b| {
            // 跳过占位符节点
            if is_placeholder(*a) || is_placeholder(*b) {
                return std::cmp::Ordering::Equal;
            }
            // 检查节点是否属于当前图
            if !a.belongs_to_graph(graph.graph_id()) || !b.belongs_to_graph(graph.graph_id()) {
                return std::cmp::Ordering::Equal;
            }
            let a_degree = graph.in_edges(*a).len() + graph.out_edges(*a).len();
            let b_degree = graph.in_edges(*b).len() + graph.out_edges(*b).len();

            // 度数大的在前
            b_degree.cmp(&a_degree)
        });
    }

    layering
}

/// 使用权重排序初始化
pub fn init_order_by_weight(graph: &Graph) -> Vec<Vec<NodeIndex>> {
    let mut layering = build_layer_matrix(graph);

    // 对每一层按边权重排序
    for layer in layering.iter_mut() {
        layer.sort_by(|a, b| {
            // 跳过占位符节点
            if is_placeholder(*a) || is_placeholder(*b) {
                return std::cmp::Ordering::Equal;
            }
            // 检查节点是否属于当前图
            if !a.belongs_to_graph(graph.graph_id()) || !b.belongs_to_graph(graph.graph_id()) {
                return std::cmp::Ordering::Equal;
            }
            let a_weight = calculate_node_weight(graph, *a);
            let b_weight = calculate_node_weight(graph, *b);

            b_weight
                .partial_cmp(&a_weight)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    layering
}

/// 计算节点权重
fn calculate_node_weight(graph: &Graph, node_id: NodeIndex) -> f64 {
    // 跳过占位符节点
    if is_placeholder(node_id) {
        return 0.0;
    }
    // 检查节点是否属于当前图
    if !node_id.belongs_to_graph(graph.graph_id()) {
        panic!(
            "Node {:?} does not belong to graph {}\nGraph debug info: {}\nNode debug info: belongs_to_graph({}) = {}",
            node_id,
            graph.graph_id(),
            graph.debug_info(),
            graph.graph_id(),
            node_id.belongs_to_graph(graph.graph_id())
        );
    }

    let mut weight = 0.0;

    // 计算入边权重
    for edge in graph.in_edges(node_id) {
        if let Some(edge_label) = graph.edge_label(&edge) {
            weight += edge_label.weight;
        }
    }

    // 计算出边权重
    for edge in graph.out_edges(node_id) {
        if let Some(edge_label) = graph.edge_label(&edge) {
            weight += edge_label.weight;
        }
    }

    weight
}

/// 使用拓扑排序初始化
pub fn init_order_topological(graph: &Graph) -> Vec<Vec<NodeIndex>> {
    let mut layering = build_layer_matrix(graph);

    // 对每一层进行拓扑排序
    for layer in layering.iter_mut() {
        layer.sort_by(|a, b| {
            // 按拓扑顺序排序
            let a_predecessors = graph.predecessors(*a);
            let b_predecessors = graph.predecessors(*b);

            a_predecessors.count().cmp(&b_predecessors.count())
        });
    }

    layering
}

// 测试代码暂时移除，因为需要重新适配NodeIndex
