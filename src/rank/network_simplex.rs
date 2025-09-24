//! 网络单纯形算法 - 精确复现JavaScript版本

use crate::graph::{Graph, NodeIndex};
use crate::types::*;
use std::collections::HashSet;

/// 网络单纯形算法 - 精确复现JavaScript版本
pub fn network_simplex(graph: &mut Graph) {
    // 1. 简化图
    // simplify(g) - 在Rust中我们假设图已经是简化的

    // 2. 初始化rank
    super::util::longest_path(graph);

    // 3. 构建feasible tree
    feasible_tree(graph);

    // 4. 初始化low/lim值
    init_low_lim_values(graph);

    // 5. 初始化cut值
    init_cut_values(graph);

    // 6. 迭代优化
    loop {
        if let Some(leave_edge) = leave_edge(graph) {
            let enter_edge = enter_edge(graph, &leave_edge);
            exchange_edges(graph, &leave_edge, &enter_edge);
        } else {
            break;
        }
    }
}

/// 构建feasible tree - 精确复现JavaScript版本
fn feasible_tree(graph: &mut Graph) {
    // 选择任意节点作为起始点
    let start = graph.node_indices().next().unwrap();
    let size = graph.node_count();

    // 使用HashSet来跟踪树中的节点
    let mut tree_nodes = std::collections::HashSet::new();
    tree_nodes.insert(start);

    // 迭代构建tight tree
    while tight_tree(graph, &mut tree_nodes) < size {
        let edge = find_min_slack_edge_for_tree(graph, &tree_nodes);
        let delta = if tree_nodes.contains(&edge.source) {
            super::util::slack(graph, &edge) as i32
        } else {
            -(super::util::slack(graph, &edge) as i32)
        };
        shift_ranks_for_tree(graph, &tree_nodes, delta);
    }

    // 建立树关系 - 将tight edges标记为树边
    establish_tree_relationships(graph, &tree_nodes);
}

/// 建立树关系 - 将tight edges标记为树边
fn establish_tree_relationships(
    graph: &mut Graph,
    tree_nodes: &std::collections::HashSet<NodeIndex>,
) {
    // 首先清除所有现有的树边标记
    for edge in graph.edges() {
        if let Some(edge_label) = graph.edge_label_mut(&edge) {
            edge_label.cutvalue = None; // 清除cut值
        }
    }

    // 找到所有tight edges并标记为树边
    let mut visited = std::collections::HashSet::new();
    let mut parent_map = std::collections::HashMap::new();

    fn dfs_establish_tree(
        graph: &mut Graph,
        tree_nodes: &std::collections::HashSet<NodeIndex>,
        visited: &mut std::collections::HashSet<NodeIndex>,
        parent_map: &mut std::collections::HashMap<NodeIndex, NodeIndex>,
        v: NodeIndex,
    ) {
        visited.insert(v);

        for edge in graph.out_edges(v) {
            let w = edge.target;
            if tree_nodes.contains(&w)
                && !visited.contains(&w)
                && super::util::slack(graph, &edge) == 0
            {
                parent_map.insert(w, v);
                if let Some(edge_label) = graph.edge_label_mut(&edge) {
                    edge_label.cutvalue = Some(0); // 标记为树边
                }
                dfs_establish_tree(graph, tree_nodes, visited, parent_map, w);
            }
        }

        for edge in graph.in_edges(v) {
            let w = edge.source;
            if tree_nodes.contains(&w)
                && !visited.contains(&w)
                && super::util::slack(graph, &edge) == 0
            {
                parent_map.insert(w, v);
                if let Some(edge_label) = graph.edge_label_mut(&edge) {
                    edge_label.cutvalue = Some(0); // 标记为树边
                }
                dfs_establish_tree(graph, tree_nodes, visited, parent_map, w);
            }
        }
    }

    // 从起始节点开始建立树关系
    let start = tree_nodes.iter().next().unwrap();
    dfs_establish_tree(graph, tree_nodes, &mut visited, &mut parent_map, *start);

    // 设置父节点关系
    for (child, parent) in parent_map {
        if let Some(node_label) = graph.node_label_mut(child) {
            node_label.parent = Some(parent);
        }
    }
}

/// 找到tight edges的最大树并返回节点数
fn tight_tree(graph: &Graph, tree_nodes: &mut std::collections::HashSet<NodeIndex>) -> usize {
    let mut visited = HashSet::new();

    fn dfs(
        graph: &Graph,
        tree_nodes: &mut std::collections::HashSet<NodeIndex>,
        visited: &mut HashSet<NodeIndex>,
        v: NodeIndex,
    ) {
        visited.insert(v);

        for edge in graph.out_edges(v) {
            let w = edge.target;
            if !visited.contains(&w)
                && !tree_nodes.contains(&w)
                && super::util::slack(graph, &edge) == 0
            {
                tree_nodes.insert(w);
                dfs(graph, tree_nodes, visited, w);
            }
        }

        for edge in graph.in_edges(v) {
            let w = edge.source;
            if !visited.contains(&w)
                && !tree_nodes.contains(&w)
                && super::util::slack(graph, &edge) == 0
            {
                tree_nodes.insert(w);
                dfs(graph, tree_nodes, visited, w);
            }
        }
    }

    let nodes: Vec<NodeIndex> = tree_nodes.iter().cloned().collect();
    for node in nodes {
        if !visited.contains(&node) {
            dfs(graph, tree_nodes, &mut visited, node);
        }
    }

    tree_nodes.len()
}

/// 找到最小slack的边
fn find_min_slack_edge_for_tree(
    graph: &Graph,
    tree_nodes: &std::collections::HashSet<NodeIndex>,
) -> Edge {
    let mut min_slack = f64::INFINITY;
    let mut min_edge = None;

    for edge in graph.edges() {
        let source_in_tree = tree_nodes.contains(&edge.source);
        let target_in_tree = tree_nodes.contains(&edge.target);

        let edge_slack = if source_in_tree != target_in_tree {
            super::util::slack(graph, &edge) as f64
        } else {
            f64::INFINITY
        };

        if edge_slack < min_slack {
            min_slack = edge_slack;
            min_edge = Some(edge);
        }
    }

    min_edge.unwrap()
}

/// 调整rank值
fn shift_ranks_for_tree(
    graph: &mut Graph,
    tree_nodes: &std::collections::HashSet<NodeIndex>,
    delta: i32,
) {
    // 直接遍历tree_nodes中的所有节点
    for node in tree_nodes {
        if let Some(label) = graph.node_label_mut(*node) {
            if let Some(rank) = label.rank {
                label.rank = Some(rank + delta);
            }
        }
    }
}

/// 初始化low/lim值 - 精确复现JavaScript版本
fn init_low_lim_values(graph: &mut Graph) {
    // 找到根节点（没有父节点的节点）
    let root = graph
        .node_indices()
        .find(|&node| {
            if let Some(label) = graph.node_label(node) {
                label.parent.is_none()
            } else {
                false
            }
        })
        .unwrap();

    dfs_assign_low_lim(graph, &mut HashSet::new(), 1, root, None);
}

/// DFS分配low/lim值
fn dfs_assign_low_lim(
    graph: &mut Graph,
    visited: &mut HashSet<NodeIndex>,
    mut next_lim: i32,
    v: NodeIndex,
    parent: Option<NodeIndex>,
) -> i32 {
    let low = next_lim;
    let mut label = graph.node_label_mut(v).unwrap().clone();

    visited.insert(v);

    // 遍历邻居节点，只考虑树边
    for edge in graph.out_edges(v) {
        let w = edge.target;
        if !visited.contains(&w) && is_tree_edge(graph, v, w) {
            next_lim = dfs_assign_low_lim(graph, visited, next_lim, w, Some(v));
        }
    }

    for edge in graph.in_edges(v) {
        let w = edge.source;
        if !visited.contains(&w) && is_tree_edge(graph, w, v) {
            next_lim = dfs_assign_low_lim(graph, visited, next_lim, w, Some(v));
        }
    }

    label.low = Some(low);
    label.lim = Some(next_lim);
    if let Some(p) = parent {
        label.parent = Some(p);
    } else {
        label.parent = None;
    }

    *graph.node_label_mut(v).unwrap() = label;
    next_lim + 1
}

/// 初始化cut值
fn init_cut_values(graph: &mut Graph) {
    // 获取后序遍历的节点（除了根节点）
    let mut vs = postorder(graph);
    if !vs.is_empty() {
        vs.pop(); // 移除根节点
    }

    for v in vs {
        assign_cut_value(graph, v);
    }
}

/// 分配cut值
fn assign_cut_value(graph: &mut Graph, child: NodeIndex) {
    let child_label = graph.node_label(child).unwrap();
    if let Some(parent) = child_label.parent {
        let cut_value = calc_cut_value(graph, child);
        if let Some(edge_label) = graph.edge_label_mut(&Edge::new(child, parent)) {
            edge_label.cutvalue = Some(cut_value);
        } else if let Some(edge_label) = graph.edge_label_mut(&Edge::new(parent, child)) {
            edge_label.cutvalue = Some(cut_value);
        }
    }
}

/// 计算cut值
fn calc_cut_value(graph: &Graph, child: NodeIndex) -> i32 {
    let child_label = graph.node_label(child).unwrap();
    let parent = child_label.parent.unwrap();

    // 确定child是否是边的尾部
    let mut child_is_tail = true;
    let mut graph_edge = graph.edge_label(&Edge::new(child, parent));

    if graph_edge.is_none() {
        child_is_tail = false;
        graph_edge = graph.edge_label(&Edge::new(parent, child));
    }

    let mut cut_value = 0;
    if let Some(edge_label) = graph_edge {
        cut_value = edge_label.weight as i32;
    }

    // 遍历child的所有边
    for edge in graph.out_edges(child) {
        let other = edge.target;
        if other != parent {
            let is_out_edge = true;
            let points_to_head = is_out_edge == child_is_tail;
            let other_weight = graph
                .edge_label(&edge)
                .map(|e| e.weight as i32)
                .unwrap_or(0);

            cut_value += if points_to_head {
                other_weight
            } else {
                -other_weight
            };

            if is_tree_edge(graph, child, other) {
                if let Some(edge_label) = graph.edge_label(&Edge::new(child, other)) {
                    if let Some(other_cut_value) = edge_label.cutvalue {
                        cut_value += if points_to_head {
                            -other_cut_value
                        } else {
                            other_cut_value
                        };
                    }
                } else if let Some(edge_label) = graph.edge_label(&Edge::new(other, child)) {
                    if let Some(other_cut_value) = edge_label.cutvalue {
                        cut_value += if points_to_head {
                            -other_cut_value
                        } else {
                            other_cut_value
                        };
                    }
                }
            }
        }
    }

    for edge in graph.in_edges(child) {
        let other = edge.source;
        if other != parent {
            let is_out_edge = false;
            let points_to_head = is_out_edge == child_is_tail;
            let other_weight = graph
                .edge_label(&edge)
                .map(|e| e.weight as i32)
                .unwrap_or(0);

            cut_value += if points_to_head {
                other_weight
            } else {
                -other_weight
            };

            if is_tree_edge(graph, child, other) {
                if let Some(edge_label) = graph.edge_label(&Edge::new(other, child)) {
                    if let Some(other_cut_value) = edge_label.cutvalue {
                        cut_value += if points_to_head {
                            -other_cut_value
                        } else {
                            other_cut_value
                        };
                    }
                } else if let Some(edge_label) = graph.edge_label(&Edge::new(child, other)) {
                    if let Some(other_cut_value) = edge_label.cutvalue {
                        cut_value += if points_to_head {
                            -other_cut_value
                        } else {
                            other_cut_value
                        };
                    }
                }
            }
        }
    }

    cut_value
}

/// 检查是否是树边
fn is_tree_edge(graph: &Graph, u: NodeIndex, v: NodeIndex) -> bool {
    if let Some(edge_label) = graph.edge_label(&Edge::new(u, v)) {
        edge_label.cutvalue.is_some()
    } else if let Some(edge_label) = graph.edge_label(&Edge::new(v, u)) {
        edge_label.cutvalue.is_some()
    } else {
        false
    }
}

/// 后序遍历
fn postorder(graph: &Graph) -> Vec<NodeIndex> {
    let mut result = Vec::new();
    let mut visited = HashSet::new();

    fn dfs(
        graph: &Graph,
        visited: &mut HashSet<NodeIndex>,
        result: &mut Vec<NodeIndex>,
        v: NodeIndex,
    ) {
        visited.insert(v);

        // 遍历所有邻居，只考虑树边
        for edge in graph.out_edges(v) {
            let w = edge.target;
            if !visited.contains(&w) && is_tree_edge(graph, v, w) {
                dfs(graph, visited, result, w);
            }
        }

        for edge in graph.in_edges(v) {
            let w = edge.source;
            if !visited.contains(&w) && is_tree_edge(graph, w, v) {
                dfs(graph, visited, result, w);
            }
        }

        result.push(v);
    }

    let nodes: Vec<NodeIndex> = graph.node_indices().collect();
    for node in nodes {
        if !visited.contains(&node) {
            dfs(graph, &mut visited, &mut result, node);
        }
    }

    result
}

/// 找到离开边
fn leave_edge(graph: &Graph) -> Option<Edge> {
    for edge in graph.edges() {
        if let Some(edge_label) = graph.edge_label(&edge) {
            if let Some(cutvalue) = edge_label.cutvalue {
                if cutvalue < 0 {
                    return Some(edge);
                }
            }
        }
    }
    None
}

/// 找到进入边
fn enter_edge(graph: &Graph, edge: &Edge) -> Edge {
    let v = edge.source;
    let w = edge.target;

    // 确保v是尾部，w是头部
    let (v, w) = if !graph.has_edge(&Edge::new(v, w)) {
        (w, v)
    } else {
        (v, w)
    };

    let v_label = graph.node_label(v).unwrap();
    let w_label = graph.node_label(w).unwrap();
    let mut tail_label = v_label;
    let mut flip = false;

    // 如果根在边的尾部，需要翻转逻辑
    if v_label.lim.unwrap_or(0) > w_label.lim.unwrap_or(0) {
        tail_label = w_label;
        flip = true;
    }

    // 找到候选边
    let mut candidates = Vec::new();
    for candidate_edge in graph.edges() {
        let candidate_v_label = graph.node_label(candidate_edge.source).unwrap();
        let candidate_w_label = graph.node_label(candidate_edge.target).unwrap();

        let v_is_descendant = is_descendant(graph, candidate_v_label, &tail_label);
        let w_is_descendant = is_descendant(graph, candidate_w_label, &tail_label);

        if flip == v_is_descendant && flip != w_is_descendant {
            candidates.push(candidate_edge);
        }
    }

    // 找到最小slack的边
    candidates
        .into_iter()
        .min_by(|a, b| {
            let slack_a = super::util::slack(graph, a);
            let slack_b = super::util::slack(graph, b);
            slack_a.partial_cmp(&slack_b).unwrap()
        })
        .unwrap()
}

/// 检查是否是后代
fn is_descendant(_graph: &Graph, v_label: &NodeLabel, root_label: &NodeLabel) -> bool {
    let v_lim = v_label.lim.unwrap_or(0);
    let root_low = root_label.low.unwrap_or(0);
    let root_lim = root_label.lim.unwrap_or(0);

    root_low <= v_lim && v_lim <= root_lim
}

/// 交换边
fn exchange_edges(graph: &mut Graph, e: &Edge, f: &Edge) {
    // 移除旧边的树标记
    if let Some(edge_label) = graph.edge_label_mut(e) {
        edge_label.cutvalue = None;
    }

    // 添加新边的树标记
    if let Some(edge_label) = graph.edge_label_mut(f) {
        edge_label.cutvalue = Some(0);
    }

    // 重新初始化low/lim值
    init_low_lim_values(graph);

    // 重新初始化cut值
    init_cut_values(graph);

    // 更新rank
    update_ranks(graph);
}

/// 更新rank
fn update_ranks(graph: &mut Graph) {
    // 找到根节点
    let root = graph
        .node_indices()
        .find(|&v| graph.node_label(v).unwrap().parent.is_none())
        .unwrap();

    // 前序遍历
    let mut vs = preorder(graph, root);
    vs.remove(0); // 移除根节点

    for v in vs {
        let v_label = graph.node_label(v).unwrap();
        if let Some(parent) = v_label.parent {
            let mut edge = graph.edge_label(&Edge::new(v, parent));
            let mut flipped = false;

            if edge.is_none() {
                edge = graph.edge_label(&Edge::new(parent, v));
                flipped = true;
            }

            if let (Some(v_label), Some(parent_label), Some(edge_label)) =
                (graph.node_label(v), graph.node_label(parent), edge)
            {
                if let (Some(_v_rank), Some(parent_rank)) = (v_label.rank, parent_label.rank) {
                    let new_rank = if flipped {
                        parent_rank + edge_label.minlen
                    } else {
                        parent_rank - edge_label.minlen
                    };

                    if let Some(v_label_mut) = graph.node_label_mut(v) {
                        v_label_mut.rank = Some(new_rank);
                    }
                }
            }
        }
    }
}

/// 前序遍历
fn preorder(graph: &Graph, root: NodeIndex) -> Vec<NodeIndex> {
    let mut result = Vec::new();
    let mut visited = HashSet::new();

    fn dfs(
        graph: &Graph,
        visited: &mut HashSet<NodeIndex>,
        result: &mut Vec<NodeIndex>,
        v: NodeIndex,
    ) {
        visited.insert(v);
        result.push(v);

        // 遍历所有邻居，只考虑树边
        for edge in graph.out_edges(v) {
            let w = edge.target;
            if !visited.contains(&w) && is_tree_edge(graph, v, w) {
                dfs(graph, visited, result, w);
            }
        }

        for edge in graph.in_edges(v) {
            let w = edge.source;
            if !visited.contains(&w) && is_tree_edge(graph, w, v) {
                dfs(graph, visited, result, w);
            }
        }
    }

    dfs(graph, &mut visited, &mut result, root);
    result
}
