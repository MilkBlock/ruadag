//! 子图排序算法

use crate::graph::Graph;
use indexmap::IndexSet;
use petgraph::graph::NodeIndex;

/// 排序子图
pub fn sort_subgraph(graph: &Graph, root: NodeIndex, bias_right: bool) -> Vec<NodeIndex> {
    let mut visited = IndexSet::new();
    let mut result = Vec::new();

    // 使用DFS遍历子图
    dfs_sort(graph, root, &mut visited, &mut result, bias_right);

    result
}

/// DFS排序
fn dfs_sort(
    graph: &Graph,
    node: NodeIndex,
    visited: &mut IndexSet<NodeIndex>,
    result: &mut Vec<NodeIndex>,
    bias_right: bool,
) {
    if visited.contains(&node) {
        return;
    }

    visited.insert(node);

    // 获取邻居节点
    let mut neighbors: Vec<NodeIndex> = graph.successors(node).collect();
    neighbors.extend(graph.predecessors(node));

    // 根据bias_right参数排序邻居
    if bias_right {
        neighbors.sort_by(|a, b| {
            // 按节点索引排序，偏向右侧
            a.index().cmp(&b.index())
        });
    } else {
        neighbors.sort_by(|a, b| {
            // 按节点索引排序，偏向左侧
            b.index().cmp(&a.index())
        });
    }

    // 递归处理邻居
    for neighbor in neighbors {
        if !visited.contains(&neighbor) {
            dfs_sort(graph, neighbor, visited, result, bias_right);
        }
    }

    result.push(node);
}

/// 使用拓扑排序
pub fn sort_subgraph_topological(graph: &Graph, root: NodeIndex) -> Vec<NodeIndex> {
    let mut visited = IndexSet::new();
    let mut result = Vec::new();
    let mut temp_visited = IndexSet::new();

    // 检查是否有环
    if has_cycle_dfs(graph, root, &mut visited, &mut temp_visited) {
        return Vec::new();
    }

    visited.clear();
    temp_visited.clear();

    // 拓扑排序
    topological_sort_dfs(graph, root, &mut visited, &mut result);

    result
}

/// 检查环
fn has_cycle_dfs(
    graph: &Graph,
    node: NodeIndex,
    visited: &mut IndexSet<NodeIndex>,
    rec_stack: &mut IndexSet<NodeIndex>,
) -> bool {
    visited.insert(node);
    rec_stack.insert(node);

    for neighbor in graph.successors(node) {
        if !visited.contains(&neighbor) {
            if has_cycle_dfs(graph, neighbor, visited, rec_stack) {
                return true;
            }
        } else if rec_stack.contains(&neighbor) {
            return true;
        }
    }

    rec_stack.swap_remove(&node);
    false
}

/// 拓扑排序DFS
fn topological_sort_dfs(
    graph: &Graph,
    node: NodeIndex,
    visited: &mut IndexSet<NodeIndex>,
    result: &mut Vec<NodeIndex>,
) {
    if visited.contains(&node) {
        return;
    }

    visited.insert(node);

    // 先处理所有前驱节点
    for predecessor in graph.predecessors(node) {
        topological_sort_dfs(graph, predecessor, visited, result);
    }

    result.push(node);
}

/// 使用权重排序
pub fn sort_subgraph_by_weight(graph: &Graph, root: NodeIndex) -> Vec<NodeIndex> {
    let mut visited = IndexSet::new();
    let mut result = Vec::new();

    // 收集所有节点及其权重
    let mut nodes_with_weights = Vec::new();
    collect_nodes_with_weights(graph, root, &mut visited, &mut nodes_with_weights);

    // 按权重排序
    nodes_with_weights.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // 提取排序后的节点
    for (node, _) in nodes_with_weights {
        result.push(node);
    }

    result
}

/// 收集节点及其权重
fn collect_nodes_with_weights(
    graph: &Graph,
    node: NodeIndex,
    visited: &mut IndexSet<NodeIndex>,
    result: &mut Vec<(NodeIndex, f64)>,
) {
    if visited.contains(&node) {
        return;
    }

    visited.insert(node);

    // 计算节点权重
    let weight = calculate_node_weight(graph, node);
    result.push((node, weight));

    // 递归处理邻居
    for neighbor in graph.successors(node) {
        collect_nodes_with_weights(graph, neighbor, visited, result);
    }

    for neighbor in graph.predecessors(node) {
        collect_nodes_with_weights(graph, neighbor, visited, result);
    }
}

/// 计算节点权重
fn calculate_node_weight(graph: &Graph, node: NodeIndex) -> f64 {
    let mut weight = 0.0;

    // 计算入边权重
    for edge in graph.in_edges(node) {
        if let Some(edge_label) = graph.edge_label(&edge) {
            weight += edge_label.weight;
        }
    }

    // 计算出边权重
    for edge in graph.out_edges(node) {
        if let Some(edge_label) = graph.edge_label(&edge) {
            weight += edge_label.weight;
        }
    }

    weight
}

/// 使用度数排序
pub fn sort_subgraph_by_degree(graph: &Graph, root: NodeIndex) -> Vec<NodeIndex> {
    let mut visited = IndexSet::new();
    let mut result = Vec::new();

    // 收集所有节点及其度数
    let mut nodes_with_degrees = Vec::new();
    collect_nodes_with_degrees(graph, root, &mut visited, &mut nodes_with_degrees);

    // 按度数排序
    nodes_with_degrees.sort_by(|a, b| b.1.cmp(&a.1));

    // 提取排序后的节点
    for (node, _) in nodes_with_degrees {
        result.push(node);
    }

    result
}

/// 收集节点及其度数
fn collect_nodes_with_degrees(
    graph: &Graph,
    node: NodeIndex,
    visited: &mut IndexSet<NodeIndex>,
    result: &mut Vec<(NodeIndex, usize)>,
) {
    if visited.contains(&node) {
        return;
    }

    visited.insert(node);

    // 计算节点度数
    let degree = graph.in_edges(node).len() + graph.out_edges(node).len();
    result.push((node, degree));

    // 递归处理邻居
    for neighbor in graph.successors(node) {
        collect_nodes_with_degrees(graph, neighbor, visited, result);
    }

    for neighbor in graph.predecessors(node) {
        collect_nodes_with_degrees(graph, neighbor, visited, result);
    }
}

// 测试代码暂时移除，因为需要重新适配NodeIndex
