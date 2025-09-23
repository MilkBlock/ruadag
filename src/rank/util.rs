//! 排名工具函数

use crate::graph::Graph;
use crate::graph::NodeIndex;
use crate::types::*;
use indexmap::{IndexMap, IndexSet};
use std::collections::VecDeque;

/// 最长路径算法
/// 基于 JavaScript 版本的实现，使用深度优先搜索
pub fn longest_path(graph: &mut Graph) {
    let mut visited = indexmap::IndexSet::new();

    // 找到所有源节点（入度为0的节点），与JavaScript的sources()函数一致
    let sources: Vec<NodeIndex> = graph
        .node_indices()
        .filter(|&node_id| {
            // 跳过 _root 节点
            if let Some(label) = graph.node_label(node_id) {
                if label
                    .dummy
                    .as_ref()
                    .map_or(false, |d| matches!(d, Dummy::Edge))
                {
                    return false;
                }
            }

            // 只有入度为0的节点才是源节点
            graph.in_edges(node_id).is_empty()
        })
        .collect();

    // 从每个源节点开始DFS
    for source in sources {
        dfs_rank(graph, source, &mut visited);
    }

    // 如果还有未访问的节点（可能形成环或孤立节点），也要处理它们
    let unvisited_nodes: Vec<NodeIndex> = graph
        .node_indices()
        .filter(|&node_id| {
            !visited.contains(&node_id) && {
                if let Some(label) = graph.node_label(node_id) {
                    label
                        .dummy
                        .as_ref()
                        .map_or(true, |d| !matches!(d, Dummy::Edge))
                } else {
                    false
                }
            }
        })
        .collect();

    for node_id in unvisited_nodes {
        dfs_rank(graph, node_id, &mut visited);
    }
}

/// 深度优先搜索计算排名
fn dfs_rank(
    graph: &mut Graph,
    node_id: NodeIndex,
    visited: &mut indexmap::IndexSet<NodeIndex>,
) -> i32 {
    // 如果已经访问过，直接返回已计算的排名
    if visited.contains(&node_id) {
        return graph.node_label(node_id).and_then(|l| l.rank).unwrap_or(0);
    }

    visited.insert(node_id);

    // 获取所有出边的最小排名，与JavaScript版本一致
    let mut min_rank = std::i32::MAX;
    let mut has_out_edges = false;

    for edge in graph.out_edges(node_id) {
        // 忽略到 _root 节点的边
        if let Some(target_label) = graph.node_label(edge.target) {
            if target_label
                .dummy
                .as_ref()
                .map_or(false, |d| matches!(d, Dummy::Edge))
            {
                continue;
            }
        }

        has_out_edges = true;
        let target_rank = dfs_rank(graph, edge.target, visited);
        let min_len = graph
            .edge_label(&edge)
            .map(|l| l.minlen as i32)
            .unwrap_or(1);
        // JavaScript版本：dfs(e.w) - g.edge(e).minlen
        let required_rank = target_rank - min_len;

        if required_rank < min_rank {
            min_rank = required_rank;
        }
    }

    // 如果没有出边，排名为0；否则使用最小排名
    let rank = if !has_out_edges || min_rank == std::i32::MAX {
        0
    } else {
        min_rank
    };

    // 设置节点排名
    if let Some(label) = graph.node_label_mut(node_id) {
        label.rank = Some(rank);
    }

    println!(
        "  dfs_rank: NodeIndex({:?}) -> rank={}, has_out_edges={}, min_rank={}",
        node_id, rank, has_out_edges, min_rank
    );

    rank
}

/// 拓扑排序
fn topological_sort(graph: &Graph) -> Vec<NodeIndex> {
    let mut in_degree = IndexMap::new();
    let mut result = Vec::new();
    let mut queue = VecDeque::new();

    // 计算入度
    for node_id in graph.node_indices() {
        in_degree.insert(node_id, graph.in_edges(node_id).len());
        if graph.in_edges(node_id).is_empty() {
            queue.push_back(node_id);
        }
    }

    // 拓扑排序
    while let Some(node_id) = queue.pop_front() {
        result.push(node_id);

        for succ_id in graph.successors(node_id) {
            if let Some(degree) = in_degree.get_mut(&succ_id) {
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(succ_id);
                }
            }
        }
    }

    result
}

/// 计算边的松弛度
pub fn slack(graph: &Graph, edge: &Edge) -> i32 {
    let source_rank = graph
        .node_label(edge.source)
        .and_then(|l| l.rank)
        .unwrap_or(0);
    let target_rank = graph
        .node_label(edge.target)
        .and_then(|l| l.rank)
        .unwrap_or(0);
    let min_len = graph.edge_label(edge).map(|l| l.minlen as i32).unwrap_or(1);

    target_rank - source_rank - min_len
}

/// 检查图是否为DAG
pub fn is_acyclic(graph: &Graph) -> bool {
    let mut visited = IndexSet::new();
    let mut rec_stack = IndexSet::new();

    for node_id in graph.node_indices() {
        if !visited.contains(&node_id) {
            if has_cycle_dfs(graph, node_id, &mut visited, &mut rec_stack) {
                return false;
            }
        }
    }

    true
}

/// DFS检查环
fn has_cycle_dfs(
    graph: &Graph,
    node_id: NodeIndex,
    visited: &mut IndexSet<NodeIndex>,
    rec_stack: &mut IndexSet<NodeIndex>,
) -> bool {
    visited.insert(node_id);
    rec_stack.insert(node_id);

    for succ_id in graph.successors(node_id) {
        if !visited.contains(&succ_id) {
            if has_cycle_dfs(graph, succ_id, visited, rec_stack) {
                return true;
            }
        } else if rec_stack.contains(&succ_id) {
            return true;
        }
    }

    rec_stack.remove(&node_id);
    false
}

/// 获取图的强连通分量
pub fn strongly_connected_components(graph: &Graph) -> Vec<Vec<NodeIndex>> {
    let mut components = Vec::new();
    let mut visited = IndexSet::new();
    let mut stack = Vec::new();

    // 第一次DFS获取完成时间
    for node_id in graph.node_indices() {
        if !visited.contains(&node_id) {
            dfs1(graph, node_id, &mut visited, &mut stack);
        }
    }

    // 反转图
    let reversed_graph = reverse_graph(graph);

    // 第二次DFS按完成时间逆序处理
    visited.clear();
    while let Some(node_id) = stack.pop() {
        if !visited.contains(&node_id) {
            let mut component = Vec::new();
            dfs2(&reversed_graph, node_id, &mut visited, &mut component);
            if !component.is_empty() {
                components.push(component);
            }
        }
    }

    components
}

/// 第一次DFS
fn dfs1(
    graph: &Graph,
    node_id: NodeIndex,
    visited: &mut IndexSet<NodeIndex>,
    stack: &mut Vec<NodeIndex>,
) {
    visited.insert(node_id);

    for succ_id in graph.successors(node_id) {
        if !visited.contains(&succ_id) {
            dfs1(graph, succ_id, visited, stack);
        }
    }

    stack.push(node_id);
}

/// 第二次DFS
fn dfs2(
    graph: &Graph,
    node_id: NodeIndex,
    visited: &mut IndexSet<NodeIndex>,
    component: &mut Vec<NodeIndex>,
) {
    visited.insert(node_id);
    component.push(node_id);

    for succ_id in graph.successors(node_id) {
        if !visited.contains(&succ_id) {
            dfs2(graph, succ_id, visited, component);
        }
    }
}

/// 反转图
fn reverse_graph(graph: &Graph) -> Graph {
    let mut reversed = Graph::new();
    let mut node_mapping = std::collections::HashMap::new();

    // 添加所有节点
    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            let new_node_id = reversed.add_node(label.clone());
            node_mapping.insert(node_id, new_node_id);
        }
    }

    // 添加反转的边
    for edge in graph.edges() {
        if let (Some(&new_source), Some(&new_target)) = (
            node_mapping.get(&edge.target),
            node_mapping.get(&edge.source),
        ) {
            let reversed_edge = Edge::new(new_source, new_target);
            if let Some(label) = graph.edge_label(&edge) {
                let _ = reversed.add_edge(reversed_edge, label.clone());
            }
        }
    }

    reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_path() {
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

        longest_path(&mut graph);

        // 标准化排名，与JavaScript版本一致
        crate::util::normalize_ranks(&mut graph);

        assert_eq!(graph.node_label(a).unwrap().rank, Some(0));
        assert_eq!(graph.node_label(b).unwrap().rank, Some(2));
        assert_eq!(graph.node_label(c).unwrap().rank, Some(5));
    }

    #[test]
    fn test_is_acyclic() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeLabel::default());
        let b = graph.add_node(NodeLabel::default());

        let edge = Edge::new(a, b);
        let _ = graph.add_edge(edge, EdgeLabel::default());

        assert!(is_acyclic(&graph));
    }
}
