//! 交叉计数算法

use crate::graph::Graph;
use crate::graph::NodeIndex;
use crate::util::is_placeholder;
use indexmap::IndexMap;

/// 计算层级中的边交叉数
pub fn cross_count(graph: &Graph, layering: &Vec<Vec<NodeIndex>>) -> usize {
    let mut crossings = 0;

    // 计算相邻层级之间的交叉
    for rank in 0..layering.len().saturating_sub(1) {
        if let Some(current_layer) = layering.get(rank) {
            if let Some(next_layer) = layering.get(rank + 1) {
                crossings += count_crossings_between_layers(graph, current_layer, next_layer);
            }
        }
    }

    crossings
}

/// 计算两个层级之间的交叉数
fn count_crossings_between_layers(
    graph: &Graph,
    upper_layer: &[NodeIndex],
    lower_layer: &[NodeIndex],
) -> usize {
    // 为每个节点分配位置索引
    let lower_positions: IndexMap<NodeIndex, usize> = lower_layer
        .iter()
        .enumerate()
        .map(|(i, &node)| (node, i))
        .collect();

    // 收集所有边及其权重
    let mut south_entries = Vec::new();

    for &upper_node in upper_layer {
        // 跳过占位符节点
        if is_placeholder(upper_node) {
            continue;
        }
        
        for edge in graph.out_edges(upper_node) {
            if let Some(&pos) = lower_positions.get(&edge.target) {
                let weight = graph.edge_label(&edge)
                    .map(|label| label.weight as usize)
                    .unwrap_or(1);
                south_entries.push((pos, weight));
            }
        }
    }

    // 按位置排序
    south_entries.sort_by_key(|&(pos, _)| pos);

    // 构建累加器树
    let first_index = if lower_layer.is_empty() {
        1
    } else {
        let mut fi = 1;
        while fi < lower_layer.len() {
            fi <<= 1;
        }
        fi
    };
    let tree_size = 2 * first_index - 1;
    let first_index = first_index - 1;
    let mut tree = vec![0; tree_size];

    // 计算加权交叉数
    let mut cc = 0;
    for (pos, weight) in south_entries {
        let mut index = pos + first_index;
        tree[index] += weight;
        let mut weight_sum = 0;
        
        while index > 0 {
            if index % 2 == 1 {
                weight_sum += tree[index + 1];
            }
            index = (index - 1) >> 1;
            tree[index] += weight;
        }
        cc += weight * weight_sum;
    }

    cc
}

/// 检查两条边是否交叉
fn edges_cross(edge1: &(usize, usize), edge2: &(usize, usize)) -> bool {
    let (u1, l1) = edge1;
    let (u2, l2) = edge2;

    // 边交叉的条件：一条边的上端点在上层中位置更靠前，
    // 但下端点在下层中位置更靠后
    (u1 < u2 && l1 > l2) || (u1 > u2 && l1 < l2)
}

/// 获取最大排名

/// 计算层级内的交叉数（用于自环边）
pub fn count_crossings_within_layer(graph: &Graph, layer: &[NodeIndex]) -> usize {
    let mut crossings = 0;

    // 检查同一层级内的边交叉
    for i in 0..layer.len() {
        for j in (i + 1)..layer.len() {
            let node1 = layer[i];
            let node2 = layer[j];

            // 检查是否有边连接这两个节点
            let edge1 = crate::types::Edge::new(node1, node2);
            let edge2 = crate::types::Edge::new(node2, node1);

            if graph.has_edge(&edge1) || graph.has_edge(&edge2) {
                // 检查这些边是否与其他边交叉
                for k in 0..layer.len() {
                    if k != i && k != j {
                        let node3 = layer[k];
                        crossings +=
                            count_crossings_with_node(graph, &node1, &node2, &node3, i, j, k);
                    }
                }
            }
        }
    }

    crossings
}

/// 计算与特定节点的交叉数
fn count_crossings_with_node(
    graph: &Graph,
    _node1: &NodeIndex,
    _node2: &NodeIndex,
    node3: &NodeIndex,
    pos1: usize,
    pos2: usize,
    pos3: usize,
) -> usize {
    let mut crossings = 0;

    // 检查从node3出发的边是否与node1-node2的边交叉
    for edge in graph.out_edges(*node3) {
        if let Some(target_pos) = find_node_position_in_layer(graph, edge.target) {
            if target_pos != pos1 && target_pos != pos2 {
                // 检查边是否交叉
                if edges_cross(&(pos1, pos2), &(pos3, target_pos)) {
                    crossings += 1;
                }
            }
        }
    }

    crossings
}

/// 在层级中查找节点位置
fn find_node_position_in_layer(graph: &Graph, node_id: NodeIndex) -> Option<usize> {
    // 查找节点在层级中的位置
    if let Some(node_label) = graph.node_label(node_id) {
        if let Some(rank) = node_label.rank {
            // 找到同一层级的所有节点
            let mut layer_nodes: Vec<NodeIndex> = graph
                .node_indices()
                .filter(|&n| {
                    graph
                        .node_label(n)
                        .and_then(|label| label.rank)
                        .map_or(false, |r| r == rank)
                })
                .collect();

            // 按order排序
            layer_nodes.sort_by_key(|&n| {
                graph
                    .node_label(n)
                    .and_then(|label| label.order)
                    .unwrap_or(0)
            });

            // 返回节点在层级中的位置
            layer_nodes.iter().position(|&n| n == node_id)
        } else {
            None
        }
    } else {
        None
    }
}

/// 使用更高效的算法计算交叉数
pub fn cross_count_efficient(graph: &Graph, layering: &Vec<Vec<NodeIndex>>) -> usize {
    let mut total_crossings = 0;

    for rank in 0..layering.len() {
        if let Some(current_layer) = layering.get(rank) {
            if let Some(next_layer) = layering.get(rank + 1) {
                total_crossings += count_crossings_efficient(graph, current_layer, next_layer);
            }
        }
    }

    total_crossings
}

/// 高效计算两个层级间的交叉数
fn count_crossings_efficient(
    graph: &Graph,
    upper_layer: &[NodeIndex],
    lower_layer: &[NodeIndex],
) -> usize {
    // 构建边列表
    let mut edges = Vec::new();

    for (upper_idx, &upper_node) in upper_layer.iter().enumerate() {
        for (lower_idx, &lower_node) in lower_layer.iter().enumerate() {
            let edge = crate::types::Edge::new(upper_node, lower_node);
            if graph.has_edge(&edge) {
                edges.push((upper_idx, lower_idx));
            }
        }
    }

    // 使用排序算法计算交叉数
    count_crossings_by_sorting(&edges)
}

/// 使用排序算法计算交叉数
fn count_crossings_by_sorting(edges: &[(usize, usize)]) -> usize {
    if edges.len() <= 1 {
        return 0;
    }

    // 按上层位置排序
    let mut sorted_edges = edges.to_vec();
    sorted_edges.sort_by_key(|&(upper, _)| upper);

    // 计算逆序数
    let mut crossings = 0;
    let mut lower_positions: Vec<usize> = sorted_edges.iter().map(|&(_, lower)| lower).collect();

    // 使用归并排序计算逆序数
    crossings += count_inversions(&mut lower_positions);

    crossings
}

/// 计算逆序数
fn count_inversions(arr: &mut [usize]) -> usize {
    if arr.len() <= 1 {
        return 0;
    }

    let mid = arr.len() / 2;
    let left_inversions = count_inversions(&mut arr[..mid]);
    let right_inversions = count_inversions(&mut arr[mid..]);
    let merge_inversions = merge_and_count_inversions(arr, mid);

    left_inversions + right_inversions + merge_inversions
}

/// 归并并计算逆序数
fn merge_and_count_inversions(arr: &mut [usize], mid: usize) -> usize {
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut inversions = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
            inversions += left.len() - i;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }

    inversions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    #[test]
    fn test_cross_count() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeLabel::default());
        let b = graph.add_node(NodeLabel::default());
        let c = graph.add_node(NodeLabel::default());
        let d = graph.add_node(NodeLabel::default());

        // 添加边
        let edge_ac = Edge::new(a, c);
        let edge_bd = Edge::new(b, d);
        let _ = graph.add_edge(edge_ac, EdgeLabel::default());
        let _ = graph.add_edge(edge_bd, EdgeLabel::default());

        // 设置排名
        if let Some(label_a) = graph.node_label_mut(a) {
            label_a.rank = Some(0);
        }
        if let Some(label_b) = graph.node_label_mut(b) {
            label_b.rank = Some(0);
        }
        if let Some(label_c) = graph.node_label_mut(c) {
            label_c.rank = Some(1);
        }
        if let Some(label_d) = graph.node_label_mut(d) {
            label_d.rank = Some(1);
        }

        let layering = vec![vec![a, b], vec![c, d]];

        let crossings = cross_count(&graph, &layering);
        assert_eq!(crossings, 0); // 没有交叉
    }

    #[test]
    fn test_edges_cross() {
        assert!(edges_cross(&(0, 1), &(1, 0))); // 交叉
        assert!(!edges_cross(&(0, 0), &(1, 1))); // 不交叉
        assert!(!edges_cross(&(0, 1), &(1, 1))); // 不交叉
    }
}
