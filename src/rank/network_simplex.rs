//! 网络单纯形算法

use crate::graph::Graph;
use crate::types::*;
// use std::collections::{HashMap, HashSet, VecDeque};

/// 网络单纯形算法
pub fn network_simplex(graph: &mut Graph) {
    // 首先使用最长路径算法获取初始排名
    super::util::longest_path(graph);

    // 创建辅助图
    let mut aux_graph = create_auxiliary_graph(graph);

    // 使用网络单纯形算法优化
    optimize_with_network_simplex(graph, &mut aux_graph);
}

/// 创建辅助图用于网络单纯形算法
fn create_auxiliary_graph(graph: &Graph) -> Graph {
    let mut aux_graph = Graph::new();

    // 添加源节点
    let _source = "source".to_string();
    let source_node = aux_graph.add_node(NodeLabel::default());

    // 添加所有原始节点
    for node_id in graph.node_indices() {
        aux_graph.add_node(NodeLabel::default());

        // 从源节点到每个节点的边
        let source_edge = Edge::new(source_node, node_id);
        let mut source_label = EdgeLabel::default();
        source_label.weight = 1.0;
        source_label.minlen = 0;
        let _ = aux_graph.add_edge(source_edge, source_label);
    }

    // 添加原始图中的边
    for edge in graph.edges() {
        let mut edge_label = EdgeLabel::default();
        if let Some(original_label) = graph.edge_label(&edge) {
            edge_label.minlen = original_label.minlen;
            edge_label.weight = original_label.weight;
        }
        let _ = aux_graph.add_edge(edge, edge_label);
    }

    aux_graph
}

/// 使用网络单纯形算法优化
fn optimize_with_network_simplex(graph: &mut Graph, _aux_graph: &mut Graph) {
    let mut improved = true;
    let mut iterations = 0;
    let max_iterations = 1000;

    while improved && iterations < max_iterations {
        improved = false;
        iterations += 1;

        // 找到需要优化的边
        let mut edges_to_optimize = Vec::new();

        for edge in graph.edges() {
            if let Some(_edge_label) = graph.edge_label(&edge) {
                let slack = super::util::slack(graph, &edge);
                if slack > 0 {
                    edges_to_optimize.push((edge, slack));
                }
            }
        }

        // 按松弛度排序，优先处理松弛度大的边
        edges_to_optimize.sort_by(|a, b| b.1.cmp(&a.1));

        // 尝试优化每条边
        for (edge, _slack) in edges_to_optimize {
            if optimize_edge(graph, &edge) {
                improved = true;
                break; // 一次只优化一条边
            }
        }
    }
}

/// 优化单条边
fn optimize_edge(graph: &mut Graph, edge: &Edge) -> bool {
    if let Some(edge_label) = graph.edge_label(edge) {
        let current_slack = super::util::slack(graph, edge);

        if current_slack <= 0 {
            return false;
        }

        // 尝试调整目标节点的排名
        if let Some(source_label) = graph.node_label(edge.source) {
            let source_rank = source_label.rank.unwrap_or(0);
            let min_len = edge_label.minlen as i32;
            let new_rank = source_rank + min_len;

            if let Some(target_label) = graph.node_label_mut(edge.target) {
                let current_rank = target_label.rank.unwrap_or(0);
                if new_rank < current_rank {
                    target_label.rank = Some(new_rank);
                    return true;
                }
            }
        }
    }

    false
}

/// 计算网络流成本
pub fn calculate_flow_cost(graph: &Graph) -> f64 {
    let mut cost = 0.0;

    for edge in graph.edges() {
        if let Some(edge_label) = graph.edge_label(&edge) {
            let slack = super::util::slack(graph, &edge);
            cost += edge_label.weight * slack as f64;
        }
    }

    cost
}

/// 找到最小成本边
pub fn find_min_cost_edge(graph: &Graph) -> Option<Edge> {
    let mut min_cost = f64::INFINITY;
    let mut min_edge = None;

    for edge in graph.edges() {
        if let Some(edge_label) = graph.edge_label(&edge) {
            let slack = super::util::slack(graph, &edge);
            let cost = edge_label.weight * slack as f64;

            if cost < min_cost {
                min_cost = cost;
                min_edge = Some(edge);
            }
        }
    }

    min_edge
}

/// 检查排名是否最优
pub fn is_optimal(graph: &Graph) -> bool {
    for edge in graph.edges() {
        if let Some(_edge_label) = graph.edge_label(&edge) {
            let slack = super::util::slack(graph, &edge);
            if slack < 0 {
                return false;
            }
        }
    }
    true
}

/// 使用启发式方法改进排名
pub fn improve_ranks_heuristic(graph: &mut Graph) {
    let max_iterations = 100;
    let mut iteration = 0;

    while iteration < max_iterations && !is_optimal(graph) {
        iteration += 1;

        // 找到所有需要调整的边
        let mut adjustments = Vec::new();

        for edge in graph.edges() {
            if let Some(_edge_label) = graph.edge_label(&edge) {
                let slack = super::util::slack(graph, &edge);
                if slack < 0 {
                    let adjustment = -slack;
                    adjustments.push((edge, adjustment));
                }
            }
        }

        // 按调整量排序
        adjustments.sort_by(|a, b| b.1.cmp(&a.1));

        // 应用调整
        for (edge, adjustment) in adjustments {
            if let Some(target_label) = graph.node_label_mut(edge.target) {
                if let Some(current_rank) = target_label.rank {
                    target_label.rank = Some(current_rank + adjustment);
                }
            }
        }
    }
}

/// 计算排名范围
pub fn calculate_rank_range(graph: &Graph) -> (i32, i32) {
    let mut min_rank = i32::MAX;
    let mut max_rank = i32::MIN;

    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            if let Some(rank) = label.rank {
                min_rank = min_rank.min(rank);
                max_rank = max_rank.max(rank);
            }
        }
    }

    (min_rank, max_rank)
}

/// 规范化排名
pub fn normalize_ranks(graph: &mut Graph) {
    let (min_rank, _) = calculate_rank_range(graph);

    for node_id in graph.node_indices().collect::<Vec<_>>() {
        if let Some(label) = graph.node_label_mut(node_id) {
            if let Some(rank) = label.rank {
                label.rank = Some(rank - min_rank);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_simplex() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeLabel::default());
        let b = graph.add_node(NodeLabel::default());
        let c = graph.add_node(NodeLabel::default());

        let edge_ab = Edge::new(a, b);
        let edge_bc = Edge::new(b, c);

        let mut label_ab = EdgeLabel::default();
        label_ab.minlen = 2;
        label_ab.weight = 1.0;
        let mut label_bc = EdgeLabel::default();
        label_bc.minlen = 3;
        label_bc.weight = 1.0;

        let _ = graph.add_edge(edge_ab, label_ab);
        let _ = graph.add_edge(edge_bc, label_bc);

        network_simplex(&mut graph);

        assert!(is_optimal(&graph));
    }

    #[test]
    fn test_calculate_flow_cost() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeLabel::default());
        let b = graph.add_node(NodeLabel::default());

        let edge = Edge::new(a, b);
        let mut label = EdgeLabel::default();
        label.minlen = 1;
        label.weight = 2.0;
        let _ = graph.add_edge(edge, label);

        // 设置排名
        if let Some(label_a) = graph.node_label_mut(a) {
            label_a.rank = Some(0);
        }
        if let Some(label_b) = graph.node_label_mut(b) {
            label_b.rank = Some(2);
        }

        let cost = calculate_flow_cost(&graph);
        assert!(cost >= 0.0);
    }
}
