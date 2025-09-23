//! 可行树算法

use crate::graph::Graph;
use crate::types::*;
// use std::collections::{HashMap, HashSet, VecDeque};

/// 构建可行树
pub fn feasible_tree(graph: &mut Graph) {
    // 首先使用最长路径算法获取初始排名
    super::util::longest_path(graph);

    // 创建辅助图用于网络流算法
    let mut aux_graph = create_auxiliary_graph(graph);

    // 使用网络流算法优化排名
    optimize_ranks_with_network_flow(graph, &mut aux_graph);
}

/// 创建辅助图
fn create_auxiliary_graph(graph: &Graph) -> Graph {
    let mut aux_graph = Graph::new();
    let mut node_mapping = std::collections::HashMap::new();

    // 添加源节点和汇节点
    let _source = "source".to_string();
    let _sink = "sink".to_string();

    let source_node = aux_graph.add_node(NodeLabel::default());
    let sink_node = aux_graph.add_node(NodeLabel::default());

    // 添加所有原始节点
    for node_id in graph.node_indices() {
        let new_node_id = aux_graph.add_node(NodeLabel::default());
        node_mapping.insert(node_id, new_node_id);

        // 从源节点到每个节点的边
        let source_edge = Edge::new(source_node, new_node_id);
        let mut source_label = EdgeLabel::default();
        source_label.weight = 1.0;
        let _ = aux_graph.add_edge(source_edge, source_label);

        // 从每个节点到汇节点的边
        let sink_edge = Edge::new(new_node_id, sink_node);
        let mut sink_label = EdgeLabel::default();
        sink_label.weight = 1.0;
        let _ = aux_graph.add_edge(sink_edge, sink_label);
    }

    // 添加原始图中的边
    for edge in graph.edges() {
        if let (Some(&new_source), Some(&new_target)) = (
            node_mapping.get(&edge.source),
            node_mapping.get(&edge.target)
        ) {
            let mut edge_label = EdgeLabel::default();
            if let Some(original_label) = graph.edge_label(&edge) {
                edge_label.minlen = original_label.minlen;
                edge_label.weight = original_label.weight;
            }
            let new_edge = Edge::new(new_source, new_target);
            let _ = aux_graph.add_edge(new_edge, edge_label);
        }
    }

    aux_graph
}

/// 使用网络流算法优化排名
fn optimize_ranks_with_network_flow(graph: &mut Graph, _aux_graph: &mut Graph) {
    // 简化的网络流算法实现
    // 实际实现应该使用更复杂的最大流算法

    let mut improved = true;
    let mut iterations = 0;
    let max_iterations = 100;

    while improved && iterations < max_iterations {
        improved = false;
        iterations += 1;

        // 尝试优化每条边
        let mut edge_updates = Vec::new();
        for edge in graph.edges() {
            if let Some(edge_label) = graph.edge_label(&edge) {
                let current_slack = super::util::slack(graph, &edge);

                if current_slack > 0 {
                    if let (Some(source_label), Some(target_label)) =
                        (graph.node_label(edge.source), graph.node_label(edge.target))
                    {
                        let source_rank = source_label.rank.unwrap_or(0);
                        let target_rank = target_label.rank.unwrap_or(0);
                        let min_len = edge_label.minlen as i32;

                        // 调整目标节点的排名
                        let new_target_rank = source_rank + min_len;
                        if new_target_rank < target_rank {
                            edge_updates.push((edge.target, new_target_rank));
                            improved = true;
                        }
                    }
                }
            }
        }

        for (node_id, new_rank) in edge_updates {
            if let Some(label) = graph.node_label_mut(node_id) {
                label.rank = Some(new_rank);
            }
        }
    }
}

/// 检查排名是否可行
pub fn is_feasible(graph: &Graph) -> bool {
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

/// 计算总松弛度
pub fn total_slack(graph: &Graph) -> i32 {
    graph
        .edges()
        .iter()
        .map(|edge| super::util::slack(graph, edge))
        .sum()
}

/// 找到最小松弛边
pub fn find_min_slack_edge(graph: &Graph) -> Option<Edge> {
    let mut min_slack = i32::MAX;
    let mut min_edge = None;

    for edge in graph.edges() {
        let slack = super::util::slack(graph, &edge);
        if slack < min_slack {
            min_slack = slack;
            min_edge = Some(edge);
        }
    }

    min_edge
}

/// 调整排名以满足最小松弛约束
pub fn adjust_ranks_for_min_slack(graph: &mut Graph) {
    while let Some(edge) = find_min_slack_edge(graph) {
        let slack = super::util::slack(graph, &edge);
        if slack >= 0 {
            break;
        }

        // 调整目标节点的排名
        if let (Some(source_label), Some(edge_label)) =
            (graph.node_label(edge.source), graph.edge_label(&edge))
        {
            let source_rank = source_label.rank.unwrap_or(0);
            let min_len = edge_label.minlen as i32;
            let new_rank = source_rank + min_len;

            if let Some(target_label) = graph.node_label_mut(edge.target) {
                if let Some(current_rank) = target_label.rank {
                    if new_rank > current_rank {
                        target_label.rank = Some(new_rank);
                    }
                } else {
                    target_label.rank = Some(new_rank);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feasible_tree() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeLabel::default());
        let b = graph.add_node(NodeLabel::default());
        let c = graph.add_node(NodeLabel::default());

        let edge_ab = Edge::new(a, b);
        let edge_bc = Edge::new(b, c);

        let mut label_ab = EdgeLabel::default();
        label_ab.minlen = 2;
        let mut label_bc = EdgeLabel::default();
        label_bc.minlen = 3;

        let _ = graph.add_edge(edge_ab, label_ab);
        let _ = graph.add_edge(edge_bc, label_bc);

        feasible_tree(&mut graph);

        assert!(is_feasible(&graph));
    }

    #[test]
    fn test_is_feasible() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeLabel::default());
        let b = graph.add_node(NodeLabel::default());

        let edge = Edge::new(a, b);
        let mut label = EdgeLabel::default();
        label.minlen = 1;
        let _ = graph.add_edge(edge, label);

        // 设置不可行的排名
        if let Some(label_a) = graph.node_label_mut(a) {
            label_a.rank = Some(5);
        }
        if let Some(label_b) = graph.node_label_mut(b) {
            label_b.rank = Some(3);
        }

        assert!(!is_feasible(&graph));
    }
}
