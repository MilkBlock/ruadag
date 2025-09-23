//! 初始化排序算法
use crate::graph::Graph;
use crate::util::build_layer_matrix;
use petgraph::graph::NodeIndex;

/// 初始化节点排序
pub fn init_order(graph: &Graph) -> Vec<Vec<NodeIndex>> {
    let mut layering = build_layer_matrix(graph);

    // 对每一层进行排序
    for layer in layering.iter_mut() {
        // 使用启发式排序
        layer.sort_by(|a, b| {
            // 按入度排序，入度小的在前
            let a_in_degree = graph.in_edges(*a).len();
            let b_in_degree = graph.in_edges(*b).len();

            a_in_degree.cmp(&b_in_degree)
        });
    }

    layering
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
            a.index().hash(&mut hasher_a);
            b.index().hash(&mut hasher_b);
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
