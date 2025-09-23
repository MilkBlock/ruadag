//! 子图排序算法

use crate::graph::Graph;
use crate::graph::NodeIndex;
use crate::order::constraint_graph::ConstraintGraph;
use std::collections::HashMap;

/// 排序子图
pub fn sort_subgraph(
    graph: &Graph,
    root: NodeIndex,
    constraint_graph: &ConstraintGraph,
    bias_right: bool,
) -> SortResult {
    // 验证根节点是否属于当前图
    if !root.belongs_to_graph(graph.graph_id()) {
        panic!(
            "Root node {:?} does not belong to graph {}\nGraph debug info: {}\nNode debug info: belongs_to_graph({}) = {}",
            root,
            graph.graph_id(),
            graph.debug_info(),
            graph.graph_id(),
            root.belongs_to_graph(graph.graph_id())
        );
    }

    // 获取可移动的节点（子节点）
    let movable = graph.children(root);

    // 检查是否有边框节点
    let node_label = graph.node_label(root);
    let filtered_movable = if let Some(label) = node_label {
        if !label.border_left.is_empty() && !label.border_right.is_empty() {
            let bl_node = label.border_left[0];
            let br_node = label.border_right[0];
            movable
                .into_iter()
                .filter(|&w| w != bl_node && w != br_node)
                .collect()
        } else {
            movable
        }
    } else {
        movable
    };

    // 计算重心
    let barycenters = barycenter(graph, &filtered_movable);

    // 递归处理子图
    let mut subgraphs = HashMap::new();
    for entry in &barycenters {
        if !graph.children(entry.v).is_empty() {
            let subgraph_result = sort_subgraph(graph, entry.v, constraint_graph, bias_right);
            subgraphs.insert(entry.v, subgraph_result);
        }
    }

    // 合并子图重心
    let mut merged_barycenters = barycenters;
    for entry in &mut merged_barycenters {
        if let Some(subgraph_result) = subgraphs.get(&entry.v) {
            merge_barycenters(entry, subgraph_result);
        }
    }

    // 解决冲突
    let resolved_entries = resolve_conflicts(&merged_barycenters, constraint_graph);

    // 展开子图
    let expanded_entries = expand_subgraphs(resolved_entries, &subgraphs);

    // 排序
    let result = sort_entries(&expanded_entries, bias_right);

    // 处理边框节点
    if let Some(label) = node_label {
        if !label.border_left.is_empty() && !label.border_right.is_empty() {
            let bl_node = label.border_left[0];
            let br_node = label.border_right[0];
            let mut final_vs = vec![bl_node];
            final_vs.extend(result.vs);
            final_vs.push(br_node);

            // 计算边框节点的重心
            let mut final_barycenter = result.barycenter;
            let mut final_weight = result.weight;

            if !graph.in_edges(bl_node).is_empty() {
                let bl_pred = graph.in_edges(bl_node)[0].source;
                let br_pred = graph.in_edges(br_node)[0].source;

                if let (Some(bl_pred_label), Some(br_pred_label)) =
                    (graph.node_label(bl_pred), graph.node_label(br_pred))
                {
                    let bl_order = bl_pred_label.order.unwrap_or(0) as f64;
                    let br_order = br_pred_label.order.unwrap_or(0) as f64;

                    if final_barycenter.is_none() {
                        final_barycenter = Some(0.0);
                        final_weight = Some(0.0);
                    }

                    if let (Some(bc), Some(w)) = (final_barycenter, final_weight) {
                        final_barycenter = Some((bc * w + bl_order + br_order) / (w + 2.0));
                        final_weight = Some(w + 2.0);
                    }
                }
            }

            SortResult {
                vs: final_vs,
                barycenter: final_barycenter,
                weight: final_weight,
            }
        } else {
            result
        }
    } else {
        result
    }
}

/// 重心条目
#[derive(Debug, Clone)]
struct BarycenterEntry {
    v: NodeIndex,
    barycenter: Option<f64>,
    weight: Option<f64>,
}

/// 映射条目 - 用于冲突解决
#[derive(Debug, Clone)]
struct MappedEntry {
    indegree: usize,
    in_edges: Vec<usize>, // 存储索引而不是引用
    out_edges: Vec<usize>,
    vs: Vec<NodeIndex>,
    i: usize,
    barycenter: Option<f64>,
    weight: Option<f64>,
    merged: bool,
}

/// 解决冲突后的条目
#[derive(Debug, Clone)]
struct ResolvedEntry {
    vs: Vec<NodeIndex>,
    i: usize,
    barycenter: Option<f64>,
    weight: Option<f64>,
}

/// 排序结果
#[derive(Debug, Clone)]
pub struct SortResult {
    pub vs: Vec<NodeIndex>,
    pub barycenter: Option<f64>,
    pub weight: Option<f64>,
}

impl SortResult {
    /// 检查结果是否为空
    pub fn is_empty(&self) -> bool {
        self.vs.is_empty()
    }
}

/// 计算重心
fn barycenter(graph: &Graph, movable: &[NodeIndex]) -> Vec<BarycenterEntry> {
    movable
        .iter()
        .map(|&v| {
            let in_edges = graph.in_edges(v);

            if in_edges.is_empty() {
                BarycenterEntry {
                    v,
                    barycenter: None,
                    weight: None,
                }
            } else {
                let mut sum = 0.0;
                let mut weight = 0.0;

                for edge in in_edges {
                    let edge_label = graph.edge_label(&edge).unwrap();
                    let source_label = graph.node_label(edge.source).unwrap();
                    let edge_weight = edge_label.weight;
                    let source_order = source_label.order.unwrap_or(0) as f64;

                    sum += edge_weight * source_order;
                    weight += edge_weight;
                }

                BarycenterEntry {
                    v,
                    barycenter: Some(sum / weight),
                    weight: Some(weight),
                }
            }
        })
        .collect()
}

/// 解决冲突
fn resolve_conflicts(
    entries: &[BarycenterEntry],
    constraint_graph: &ConstraintGraph,
) -> Vec<ResolvedEntry> {
    // 创建映射条目
    let mut mapped_entries: Vec<MappedEntry> = entries
        .iter()
        .enumerate()
        .map(|(i, entry)| MappedEntry {
            indegree: 0,
            in_edges: Vec::new(),
            out_edges: Vec::new(),
            vs: vec![entry.v],
            i,
            barycenter: entry.barycenter,
            weight: entry.weight,
            merged: false,
        })
        .collect();

    // 创建节点到索引的映射
    let mut node_to_index: HashMap<NodeIndex, usize> = HashMap::new();
    for (i, entry) in entries.iter().enumerate() {
        node_to_index.insert(entry.v, i);
    }

    // 处理约束边
    for (from, targets) in constraint_graph.get_all_constraints() {
        if let Some(&from_idx) = node_to_index.get(from) {
            for &to in targets {
                if let Some(&to_idx) = node_to_index.get(&to) {
                    mapped_entries[to_idx].indegree += 1;
                    mapped_entries[from_idx].out_edges.push(to_idx);
                }
            }
        }
    }

    // 找到源节点（入度为0的节点）
    let mut source_set: Vec<usize> = mapped_entries
        .iter()
        .enumerate()
        .filter(|(_, entry)| entry.indegree == 0)
        .map(|(i, _)| i)
        .collect();

    // 解决冲突
    do_resolve_conflicts(&mut mapped_entries, &mut source_set)
}

/// 执行冲突解决的核心逻辑
fn do_resolve_conflicts(
    mapped_entries: &mut [MappedEntry],
    source_set: &mut Vec<usize>,
) -> Vec<ResolvedEntry> {
    let mut result = Vec::new();

    while let Some(entry_idx) = source_set.pop() {
        let entry = &mapped_entries[entry_idx].clone();
        result.push(entry_idx);

        // 处理入边
        let in_edges = entry.in_edges.clone();
        for &u_idx in in_edges.iter().rev() {
            if mapped_entries[u_idx].merged {
                continue;
            }

            let u_entry = &mapped_entries[u_idx];
            let v_entry = &mapped_entries[entry_idx];

            if should_merge(u_entry, v_entry) {
                merge_entries(mapped_entries, entry_idx, u_idx);
            }
        }

        // 处理出边
        let out_edges = entry.out_edges.clone();
        for w_idx in out_edges {
            mapped_entries[w_idx].in_edges.push(entry_idx);
            mapped_entries[w_idx].indegree -= 1;
            if mapped_entries[w_idx].indegree == 0 {
                source_set.push(w_idx);
            }
        }
    }

    // 过滤未合并的条目并转换为结果
    result
        .into_iter()
        .filter(|&idx| !mapped_entries[idx].merged)
        .map(|idx| {
            let entry = &mapped_entries[idx];
            ResolvedEntry {
                vs: entry.vs.clone(),
                i: entry.i,
                barycenter: entry.barycenter,
                weight: entry.weight,
            }
        })
        .collect()
}

/// 判断是否应该合并两个条目
fn should_merge(u_entry: &MappedEntry, v_entry: &MappedEntry) -> bool {
    match (u_entry.barycenter, v_entry.barycenter) {
        (Some(u_bc), Some(v_bc)) => u_bc >= v_bc,
        _ => true,
    }
}

/// 合并重心
fn merge_barycenters(target: &mut BarycenterEntry, other: &SortResult) {
    if let Some(target_bc) = target.barycenter {
        if let (Some(other_bc), Some(target_w), Some(other_w)) =
            (other.barycenter, target.weight, other.weight)
        {
            target.barycenter =
                Some((target_bc * target_w + other_bc * other_w) / (target_w + other_w));
            target.weight = Some(target_w + other_w);
        }
    } else {
        target.barycenter = other.barycenter;
        target.weight = other.weight;
    }
}

/// 合并两个条目
fn merge_entries(mapped_entries: &mut [MappedEntry], target_idx: usize, source_idx: usize) {
    let source = mapped_entries[source_idx].clone();
    let target = &mut mapped_entries[target_idx];

    // 合并重心和权重
    let mut sum = 0.0;
    let mut weight = 0.0;

    if let (Some(t_bc), Some(t_w)) = (target.barycenter, target.weight) {
        sum += t_bc * t_w;
        weight += t_w;
    }

    if let (Some(s_bc), Some(s_w)) = (source.barycenter, source.weight) {
        sum += s_bc * s_w;
        weight += s_w;
    }

    // 更新目标条目
    target.vs.extend(source.vs);
    target.barycenter = if weight > 0.0 {
        Some(sum / weight)
    } else {
        None
    };
    target.weight = if weight > 0.0 { Some(weight) } else { None };
    target.i = target.i.min(source.i);

    // 标记源条目为已合并
    mapped_entries[source_idx].merged = true;
}

/// 展开子图
fn expand_subgraphs(
    entries: Vec<ResolvedEntry>,
    subgraphs: &HashMap<NodeIndex, SortResult>,
) -> Vec<BarycenterEntry> {
    let mut result = Vec::new();

    for entry in entries {
        if let Some(subgraph_result) = subgraphs.get(&entry.vs[0]) {
            // 如果第一个节点有子图，展开它
            for &node in &subgraph_result.vs {
                result.push(BarycenterEntry {
                    v: node,
                    barycenter: None, // 子图节点没有重心
                    weight: None,
                });
            }
        } else {
            // 没有子图，直接添加
            for &node in &entry.vs {
                result.push(BarycenterEntry {
                    v: node,
                    barycenter: entry.barycenter,
                    weight: entry.weight,
                });
            }
        }
    }

    result
}

/// 排序条目
fn sort_entries(entries: &[BarycenterEntry], bias_right: bool) -> SortResult {
    let mut sortable: Vec<_> = entries.iter().filter(|e| e.barycenter.is_some()).collect();
    let unsortable: Vec<_> = entries.iter().filter(|e| e.barycenter.is_none()).collect();

    // 按重心排序，考虑bias参数
    sortable.sort_by(|a, b| {
        let a_bc = a.barycenter.unwrap();
        let b_bc = b.barycenter.unwrap();

        if bias_right {
            // 右偏置：重心相同时，保持原有顺序
            a_bc.partial_cmp(&b_bc).unwrap()
        } else {
            // 左偏置：重心相同时，保持原有顺序
            a_bc.partial_cmp(&b_bc).unwrap()
        }
    });

    // 合并结果
    let mut vs = Vec::new();
    let mut sum = 0.0;
    let mut weight = 0.0;

    for entry in sortable {
        vs.push(entry.v);
        if let (Some(bc), Some(w)) = (entry.barycenter, entry.weight) {
            sum += bc * w;
            weight += w;
        }
    }

    for entry in unsortable {
        vs.push(entry.v);
    }

    SortResult {
        vs,
        barycenter: if weight > 0.0 {
            Some(sum / weight)
        } else {
            None
        },
        weight: if weight > 0.0 { Some(weight) } else { None },
    }
}
