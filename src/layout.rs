//! 主布局算法

use crate::counters::*;
use crate::graph::Graph;
use crate::graph::NodeIndex;
use crate::order::order;
use crate::position::position;
use crate::rank::rank;
use crate::types::*;
use crate::util::time;
use indexmap::IndexMap;

fn log_graph_state(graph: &Graph, stage: &str) {
    println!("\n--- {} 图状态 ---", stage);
    println!("节点:");
    for node_idx in graph.node_indices() {
        let node = graph.node_label(node_idx).unwrap();
        println!(
            "  {:?}: rank={:?}, x={:?}, y={:?}, width={}, height={}",
            node_idx, node.rank, node.x, node.y, node.width, node.height
        );
    }

    println!("边:");
    for edge in graph.edges() {
        if let Some(edge_obj) = graph.edge_label(&edge) {
            println!(
                "  {:?} -> {:?}: points={}个控制点",
                edge.source,
                edge.target,
                edge_obj.points.len()
            );
        }
    }
}

/// 执行图布局
///
/// 对应 JS 函数: layout() in lib/layout.js
pub fn layout(graph: &mut Graph, opts: Option<&LayoutOptions>) {
    // 重置计数器
    reset_counters();

    let default_opts = LayoutOptions::default();
    let opts = opts.unwrap_or(&default_opts);

    if opts.debug_timing {
        time("layout", || {
            // 构建布局图
            let mut layout_graph = time("build_layout_graph", || build_layout_graph(graph));

            // 运行布局算法
            time("run_layout", || run_layout(&mut layout_graph, opts));

            // 更新输入图
            time("update_input_graph", || {
                update_input_graph(graph, &layout_graph)
            });
        });
    } else {
        // 构建布局图
        let mut layout_graph = build_layout_graph(graph);

        // 运行布局算法
        run_layout(&mut layout_graph, opts);

        // 更新输入图
        update_input_graph(graph, &layout_graph);
    }
}

/// 构建布局图
///
/// 对应 JS 函数: buildLayoutGraph() in lib/layout.js
fn build_layout_graph(input_graph: &Graph) -> Graph {
    let mut layout_graph = Graph::with_config(input_graph.config().clone());

    // 复制节点
    let mut node_mapping = indexmap::IndexMap::new();
    for node_index in input_graph.node_indices() {
        if let Some(label) = input_graph.node_label(node_index) {
            let mut new_label = label.clone();

            // 设置默认值
            if new_label.width == 0.0 {
                new_label.width = 0.0;
            }
            if new_label.height == 0.0 {
                new_label.height = 0.0;
            }

            let new_node_index = layout_graph.add_node(new_label);
            node_mapping.insert(node_index, new_node_index);

            // 设置父节点
            if let Some(parent) = input_graph.parent(node_index) {
                if let Some(&new_parent) = node_mapping.get(&parent) {
                    layout_graph.set_parent(new_node_index, new_parent);
                }
            }
        }
    }

    // 复制边
    for edge in input_graph.edges() {
        if let Some(edge_label) = input_graph.edge_label(&edge) {
            let mut new_edge_label = edge_label.clone();

            // 设置默认值
            if new_edge_label.minlen == 0 {
                new_edge_label.minlen = 1;
            }
            if new_edge_label.weight == 0.0 {
                new_edge_label.weight = 1.0;
            }
            if new_edge_label.width == 0.0 {
                new_edge_label.width = 0.0;
            }
            if new_edge_label.height == 0.0 {
                new_edge_label.height = 0.0;
            }
            if new_edge_label.labeloffset == 0.0 {
                new_edge_label.labeloffset = 10.0;
            }

            if let (Some(&new_source), Some(&new_target)) = (
                node_mapping.get(&edge.source),
                node_mapping.get(&edge.target),
            ) {
                let new_edge = Edge::new(new_source, new_target);
                let _ = layout_graph.add_edge(new_edge, new_edge_label);
            }
        }
    }

    layout_graph
}

/// 运行布局算法
///
/// 对应 JS 函数: runLayout() in lib/layout.js
fn run_layout(graph: &mut Graph, opts: &LayoutOptions) {
    println!("=== Rust 布局过程日志 ===");

    make_space_for_edge_labels(graph);
    remove_self_edges(graph);
    acyclic(graph);
    nesting_graph_run(graph);

    println!("--- 执行 rank 前 ---");
    log_graph_state(graph, "rank前");
    increment_rank();
    rank(graph);
    println!("--- 执行 rank 后 ---");
    log_graph_state(graph, "rank后");

    inject_edge_label_proxies(graph);
    crate::util::remove_empty_ranks(graph);
    nesting_graph_cleanup(graph);
    crate::util::normalize_ranks(graph);
    normalize_edges(graph); // 添加虚拟节点
    assign_rank_min_max(graph);
    remove_edge_label_proxies(graph);
    parent_dummy_chains(graph);
    add_border_segments(graph);

    println!("--- 执行 order 前 ---");
    log_graph_state(graph, "order前");
    increment_order();
    order(graph, Some(opts));
    println!("--- 执行 order 后 ---");
    log_graph_state(graph, "order后");

    insert_self_edges(graph);
    crate::position::adjust_coordinate_system(graph);

    println!("--- 执行 position 前 ---");
    log_graph_state(graph, "position前");
    increment_position();
    position(graph);
    println!("--- 执行 position 后 ---");
    log_graph_state(graph, "position后");

    position_self_edges(graph);
    remove_border_nodes(graph);
    fixup_edge_label_coords(graph);
    undo_coordinate_system(graph);
    crate::position::translate_graph(graph);
    assign_node_intersects(graph);
    reverse_points_for_reversed_edges(graph);
    acyclic_undo(graph);
}

/// 为边标签留出空间
///
/// 对应 JS 函数: makeSpaceForEdgeLabels() in lib/layout.js
fn make_space_for_edge_labels(graph: &mut Graph) {
    let config = graph.config_mut();
    config.rank_sep /= 2.0;
    let rankdir = config.rankdir;

    for edge in graph.edges() {
        if let Some(edge_label) = graph.edge_label_mut(&edge) {
            edge_label.minlen *= 2;

            if edge_label.labelpos != LabelPosition::Center {
                match rankdir {
                    RankDirection::TopBottom | RankDirection::BottomTop => {
                        edge_label.width += edge_label.labeloffset;
                    }
                    RankDirection::LeftRight | RankDirection::RightLeft => {
                        edge_label.height += edge_label.labeloffset;
                    }
                }
            }
        }
    }
}

/// 移除自环边
///
/// 对应 JS 函数: removeSelfEdges() in lib/layout.js
fn remove_self_edges(graph: &mut Graph) {
    let mut self_edges = Vec::new();

    for edge in graph.edges() {
        if edge.source == edge.target {
            self_edges.push(edge);
        }
    }

    for edge in self_edges {
        if let Some(_edge_label) = graph.edge_label(&edge) {
            // 存储自环边信息
            if let Some(_node_label) = graph.node_label_mut(edge.source) {
                // 简化实现，不再使用custom字段
            }
        }
        graph.remove_edge(&edge);
    }
}

/// 无环化处理
///
/// 对应 JS 函数: acyclic.run() in lib/acyclic.js
fn acyclic(graph: &mut Graph) {
    // 使用深度优先搜索检测环并反转边
    let mut visited = indexmap::IndexSet::new();
    let mut rec_stack = indexmap::IndexSet::new();
    let mut edges_to_reverse = Vec::new();

    let node_indices: Vec<NodeIndex> = graph.node_indices().collect();
    for node_id in node_indices {
        if !visited.contains(&node_id) {
            dfs_acyclic(
                graph,
                node_id,
                &mut visited,
                &mut rec_stack,
                &mut edges_to_reverse,
            );
        }
    }

    // 反转检测到的环边
    for edge in edges_to_reverse {
        if let Some(edge_label) = graph.edge_label_mut(&edge) {
            edge_label.reversed = Some(true);
        }
    }
}

/// 深度优先搜索检测环
///
/// 对应 JS 函数: dfsAcyclic() in lib/acyclic.js
fn dfs_acyclic(
    graph: &mut Graph,
    node_id: NodeIndex,
    visited: &mut indexmap::IndexSet<NodeIndex>,
    rec_stack: &mut indexmap::IndexSet<NodeIndex>,
    edges_to_reverse: &mut Vec<Edge>,
) {
    visited.insert(node_id);
    rec_stack.insert(node_id);

    let successors: Vec<NodeIndex> = graph.successors(node_id).collect();
    for next_node in successors {
        if !visited.contains(&next_node) {
            dfs_acyclic(graph, next_node, visited, rec_stack, edges_to_reverse);
        } else if rec_stack.contains(&next_node) {
            // 发现环，标记边需要反转
            let edge = Edge::new(node_id, next_node);
            edges_to_reverse.push(edge);
        }
    }

    rec_stack.swap_remove(&node_id);
}

/// 注入边标签代理
///
/// 对应 JS 函数: injectEdgeLabelProxies() in lib/layout.js
fn inject_edge_label_proxies(graph: &mut Graph) {
    let mut proxies: Vec<(Edge, NodeIndex)> = Vec::new();

    for edge in graph.edges() {
        if let Some(edge_label) = graph.edge_label(&edge) {
            if edge_label.width > 0.0 && edge_label.height > 0.0 {
                if let (Some(source_label), Some(target_label)) =
                    (graph.node_label(edge.source), graph.node_label(edge.target))
                {
                    if let (Some(source_rank), Some(target_rank)) =
                        (source_label.rank, target_label.rank)
                    {
                        let proxy_rank = (target_rank - source_rank) / 2 + source_rank;
                        let _proxy_id = format!("_ep_{}", graph.node_count());

                        let mut proxy_label = crate::types::NodeLabel::default();
                        proxy_label.rank = Some(proxy_rank);
                        proxy_label.dummy = Some(Dummy::EdgeProxy);

                        let proxy_node = graph.add_node(proxy_label);
                        proxies.push((edge, proxy_node));
                    }
                }
            }
        }
    }
}

/// 规范化边，添加虚拟节点
fn normalize_edges(graph: &mut Graph) {
    let mut dummy_chains = Vec::new();
    let mut edges_to_normalize = Vec::new();

    // 收集需要规范化的边
    for edge in graph.edges() {
        if let (Some(source_label), Some(target_label)) =
            (graph.node_label(edge.source), graph.node_label(edge.target))
        {
            if let (Some(source_rank), Some(target_rank)) = (source_label.rank, target_label.rank) {
                if target_rank > source_rank + 1 {
                    edges_to_normalize.push(edge);
                }
            }
        }
    }

    // 规范化每条边
    for edge in edges_to_normalize {
        // 先获取所有需要的信息，避免借用冲突
        let (source_rank, target_rank, edge_weight) =
            if let Some(edge_label) = graph.edge_label(&edge) {
                let source_rank = graph
                    .node_label(edge.source)
                    .and_then(|l| l.rank)
                    .unwrap_or(0);
                let target_rank = graph
                    .node_label(edge.target)
                    .and_then(|l| l.rank)
                    .unwrap_or(0);
                (source_rank, target_rank, edge_label.weight)
            } else {
                continue;
            };

        if target_rank > source_rank + 1 {
            let mut current_node = edge.source;
            let mut current_rank = source_rank;
            let mut is_first_dummy = true;

            // 移除原边
            graph.remove_edge(&edge);

            // 添加虚拟节点
            while current_rank < target_rank - 1 {
                current_rank += 1;

                let mut dummy_label = NodeLabel::default();
                dummy_label.width = 0.0;
                dummy_label.height = 0.0;
                dummy_label.rank = Some(current_rank);
                dummy_label.dummy = Some(Dummy::Edge);
                dummy_label.edge_obj = Some(edge.clone());

                let dummy_node = graph.add_node(dummy_label);

                // 添加边
                let dummy_edge = Edge::new(current_node, dummy_node);
                let mut dummy_edge_label = EdgeLabel::default();
                dummy_edge_label.weight = edge_weight;
                graph.add_edge(dummy_edge, dummy_edge_label);

                if is_first_dummy {
                    dummy_chains.push(dummy_node);
                    is_first_dummy = false;
                }

                current_node = dummy_node;
            }

            // 添加最后一条边到目标节点
            let final_edge = Edge::new(current_node, edge.target);
            let mut final_edge_label = EdgeLabel::default();
            final_edge_label.weight = edge_weight;
            graph.add_edge(final_edge, final_edge_label);
        }
    }

    // 存储虚拟节点链
    let config = graph.config_mut();
    config.dummy_chains = Some(dummy_chains);
}

/// 分配排名最小最大值
///
/// 对应 JS 函数: assignRankMinMax() in lib/layout.js
fn assign_rank_min_max(graph: &mut Graph) {
    let mut max_rank = 0;

    for node_index in graph.node_indices() {
        if let Some(label) = graph.node_label(node_index) {
            if let Some(rank) = label.rank {
                max_rank = max_rank.max(rank);
            }
        }
    }

    let config = graph.config_mut();
    config.max_rank = Some(max_rank);
}

/// 移除边标签代理
///
/// 对应 JS 函数: removeEdgeLabelProxies() in lib/layout.js
fn remove_edge_label_proxies(graph: &mut Graph) {
    let mut to_remove = Vec::new();

    for node_index in graph.node_indices() {
        if let Some(label) = graph.node_label(node_index) {
            if let Some(dummy_type) = &label.dummy {
                if matches!(dummy_type, Dummy::EdgeProxy) {
                    to_remove.push(node_index);
                }
            }
        }
    }

    for node_index in to_remove {
        graph.remove_node(node_index);
    }
}

/// 父虚拟链
///
/// 对应 JS 函数: parentDummyChains() in lib/parent-dummy-chains.js
fn parent_dummy_chains(graph: &mut Graph) {
    // 获取虚拟链信息
    let dummy_chains = graph.dummy_chains.clone();

    for dummy_node in dummy_chains {
        // 先获取所有需要的信息，避免借用冲突
        let (edge_obj, _current_rank) = if let Some(node_label) = graph.node_label(dummy_node) {
            if let Some(edge_obj) = &node_label.edge_obj {
                (Some(edge_obj.clone()), node_label.rank.unwrap_or(0))
            } else {
                (None, 0)
            }
        } else {
            (None, 0)
        };

        if let Some(edge_obj) = edge_obj {
            // 找到从源节点到目标节点的路径
            let path_data = find_path(graph, edge_obj.source, edge_obj.target);
            let path = path_data.path;
            let lca = path_data.lca;

            let mut path_idx = 0;
            let mut current_node = dummy_node;
            let mut ascending = true;

            while current_node != edge_obj.target {
                if let Some(node_label) = graph.node_label(current_node) {
                    let current_rank = node_label.rank.unwrap_or(0);

                    if ascending {
                        while path_idx < path.len() {
                            let path_node = path[path_idx];
                            if let Some(path_node_label) = graph.node_label(path_node) {
                                if path_node == lca
                                    || path_node_label.max_rank.unwrap_or(0) < current_rank
                                {
                                    path_idx += 1;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }

                        if path_idx < path.len() && path[path_idx] == lca {
                            ascending = false;
                        }
                    }

                    if !ascending {
                        while path_idx < path.len() - 1 {
                            let next_path_node = path[path_idx + 1];
                            if let Some(next_node_label) = graph.node_label(next_path_node) {
                                if next_node_label.min_rank.unwrap_or(0) <= current_rank {
                                    path_idx += 1;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    if path_idx < path.len() {
                        let parent_node = path[path_idx];
                        graph.set_parent(current_node, parent_node);
                    }

                    // 移动到下一个节点
                    let successors: Vec<NodeIndex> = graph.successors(current_node).collect();
                    if let Some(next_node) = successors.first() {
                        current_node = *next_node;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }
}

/// 查找从源节点到目标节点的路径
fn find_path(graph: &Graph, v: NodeIndex, w: NodeIndex) -> PathData {
    let mut v_path = Vec::new();
    let _w_path: Vec<NodeIndex> = Vec::new();

    // 简化的路径查找实现
    // 在实际实现中，这里应该使用更复杂的算法来找到LCA
    let mut current = v;
    while let Some(parent) = graph.parent(current) {
        v_path.push(parent);
        current = parent;
        if parent == w {
            break;
        }
    }

    let lca = if v_path.is_empty() {
        v
    } else {
        v_path[v_path.len() - 1]
    };

    PathData { path: v_path, lca }
}

#[derive(Debug)]
struct PathData {
    path: Vec<NodeIndex>,
    lca: NodeIndex,
}

/// 添加边界段
///
/// 对应 JS 函数: addBorderSegments() in lib/add-border-segments.js
fn add_border_segments(graph: &mut Graph) {
    // 遍历所有节点，为有子图的节点添加边界段
    let nodes_to_process: Vec<NodeIndex> = graph.node_indices().collect();

    for node_id in nodes_to_process {
        // 先检查节点是否有最小和最大层级信息
        let (min_rank, max_rank) = if let Some(node_label) = graph.node_label(node_id) {
            if node_label.min_rank.is_some() && node_label.max_rank.is_some() {
                (
                    Some(node_label.min_rank.unwrap()),
                    Some(node_label.max_rank.unwrap()),
                )
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        if let (Some(min_rank), Some(max_rank)) = (min_rank, max_rank) {
            // 初始化边界数组
            if let Some(node_label) = graph.node_label_mut(node_id) {
                node_label.border_left = Vec::new();
                node_label.border_right = Vec::new();
            }

            // 为每个层级添加边界节点
            for rank in min_rank..=max_rank {
                // 添加左边界节点
                let left_border = add_border_node(graph, "borderLeft", "_bl", node_id, rank);
                if let Some(node_label) = graph.node_label_mut(node_id) {
                    node_label.border_left.push(left_border);
                }

                // 添加右边界节点
                let right_border = add_border_node(graph, "borderRight", "_br", node_id, rank);
                if let Some(node_label) = graph.node_label_mut(node_id) {
                    node_label.border_right.push(right_border);
                }

                // 连接相邻的边界节点
                if rank > min_rank {
                    // 先获取边界节点信息
                    let (prev_left, prev_right) =
                        if let Some(node_label) = graph.node_label(node_id) {
                            (
                                node_label
                                    .border_left
                                    .get((rank - min_rank - 1) as usize)
                                    .copied(),
                                node_label
                                    .border_right
                                    .get((rank - min_rank - 1) as usize)
                                    .copied(),
                            )
                        } else {
                            (None, None)
                        };

                    if let Some(prev_left) = prev_left {
                        graph.add_edge(Edge::new(prev_left, left_border), EdgeLabel::default());
                    }
                    if let Some(prev_right) = prev_right {
                        graph.add_edge(Edge::new(prev_right, right_border), EdgeLabel::default());
                    }
                }
            }
        }
    }
}

/// 添加边界节点
///
/// 对应 JS 函数: addBorderNode() in lib/add-border-segments.js
fn add_border_node(
    graph: &mut Graph,
    border_type: &str,
    _prefix: &str,
    parent: NodeIndex,
    rank: i32,
) -> NodeIndex {
    let mut border_label = NodeLabel::default();
    border_label.width = 0.0;
    border_label.height = 0.0;
    border_label.rank = Some(rank);
    border_label.border_type = Some(border_type.to_string());
    border_label.dummy = Some(Dummy::Border);

    let border_node = graph.add_node(border_label);
    graph.set_parent(border_node, parent);

    border_node
}

/// 插入自环边
///
/// 对应 JS 函数: insertSelfEdges() in lib/layout.js
fn insert_self_edges(graph: &mut Graph) {
    // 构建层级矩阵
    let layers = crate::util::build_layer_matrix(graph);

    for layer in layers {
        let mut order_shift = 0;

        for (i, node_id) in layer.iter().enumerate() {
            // 先获取自环边信息
            let self_edges = if let Some(node_label) = graph.node_label(*node_id) {
                node_label.self_edges.clone()
            } else {
                None
            };

            // 更新节点顺序
            if let Some(node_label) = graph.node_label_mut(*node_id) {
                node_label.order = Some(i + order_shift as usize);
            }

            // 处理自环边
            if let Some(self_edges) = self_edges {
                for self_edge in self_edges {
                    // 创建自环边虚拟节点
                    let mut dummy_label = NodeLabel::default();
                    dummy_label.width = self_edge.width;
                    dummy_label.height = self_edge.height;
                    dummy_label.dummy = Some(Dummy::SelfEdge);
                    // 创建边对象
                    let edge = Edge::new(*node_id, *node_id);
                    dummy_label.edge_obj = Some(edge);
                    if let Some(node_label) = graph.node_label(*node_id) {
                        dummy_label.rank = node_label.rank;
                    }
                    dummy_label.order = Some(i + order_shift as usize);

                    let dummy_node = graph.add_node(dummy_label);
                    graph.set_parent(dummy_node, *node_id);

                    // 添加自环边
                    let edge_label = self_edge;
                    graph.add_edge(Edge::new(*node_id, dummy_node), edge_label);

                    order_shift += 1;
                }
            }
        }
    }
}

/// 位置自环边
///
/// 对应 JS 函数: positionSelfEdges() in lib/layout.js
fn position_self_edges(graph: &mut Graph) {
    let nodes_to_process: Vec<NodeIndex> = graph.node_indices().collect();

    for node_id in nodes_to_process {
        if let Some(node_label) = graph.node_label(node_id) {
            if node_label.dummy == Some(Dummy::SelfEdge) {
                if let Some(edge_obj) = &node_label.edge_obj {
                    // 获取源节点信息
                    if let Some(source_node_label) = graph.node_label(edge_obj.source) {
                        let x = source_node_label.x.unwrap_or(0.0) + source_node_label.width / 2.0;
                        let y = source_node_label.y.unwrap_or(0.0);

                        // 计算自环边的位置
                        let dx = node_label.x.unwrap_or(0.0) - x;
                        let dy = source_node_label.height / 2.0;

                        // 更新自环边标签位置
                        if let Some(edge_label) =
                            graph.edge_label_mut(&Edge::new(edge_obj.source, edge_obj.target))
                        {
                            edge_label.x = Some(x + dx);
                            edge_label.y = Some(y + dy);
                        }

                        // 移除虚拟节点
                        graph.remove_node(node_id);
                    }
                }
            }
        }
    }
}

/// 构建层级矩阵

/// 移除边界节点
///
/// 对应 JS 函数: removeBorderNodes() in lib/layout.js
fn remove_border_nodes(graph: &mut Graph) {
    let nodes_to_process: Vec<NodeIndex> = graph.node_indices().collect();

    for node_id in nodes_to_process {
        if let Some(node_label) = graph.node_label(node_id) {
            // 检查节点是否有子节点（复合节点）
            if !graph.children(node_id).is_empty() {
                // 获取边界节点信息
                if let Some(border_top) = node_label.border_top {
                    if let Some(border_bottom) = node_label.border_bottom {
                        if let Some(border_left) = node_label.border_left.last() {
                            if let Some(border_right) = node_label.border_right.last() {
                                // 获取边界节点的位置信息
                                if let (
                                    Some(top_node),
                                    Some(bottom_node),
                                    Some(left_node),
                                    Some(right_node),
                                ) = (
                                    graph.node_label(border_top),
                                    graph.node_label(border_bottom),
                                    graph.node_label(*border_left),
                                    graph.node_label(*border_right),
                                ) {
                                    // 计算节点的新尺寸
                                    let new_width = (right_node.x.unwrap_or(0.0)
                                        - left_node.x.unwrap_or(0.0))
                                    .abs();
                                    let new_height = (bottom_node.y.unwrap_or(0.0)
                                        - top_node.y.unwrap_or(0.0))
                                    .abs();

                                    // 更新节点尺寸
                                    if let Some(node_label_mut) = graph.node_label_mut(node_id) {
                                        node_label_mut.width = new_width;
                                        node_label_mut.height = new_height;
                                    }
                                }
                            }
                        }
                    }
                }

                // 移除所有边界节点
                remove_border_nodes_recursive(graph, node_id);
            }
        }
    }
}

/// 递归移除边界节点
///
/// 对应 JS 函数: removeBorderNodes() 内部递归部分 in lib/layout.js
fn remove_border_nodes_recursive(graph: &mut Graph, parent: NodeIndex) {
    let children: Vec<NodeIndex> = graph.children(parent);

    for child in children {
        if let Some(child_label) = graph.node_label(child) {
            if child_label.dummy == Some(Dummy::Border) {
                // 移除边界节点
                graph.remove_node(child);
            } else {
                // 递归处理子节点
                remove_border_nodes_recursive(graph, child);
            }
        }
    }
}

/// 修复边标签坐标
///
/// 对应 JS 函数: fixupEdgeLabelCoords() in lib/layout.js
fn fixup_edge_label_coords(graph: &mut Graph) {
    for edge in graph.edges() {
        if let Some(edge_label) = graph.edge_label_mut(&edge) {
            if let Some(x) = edge_label.x {
                match edge_label.labelpos {
                    LabelPosition::Left => {
                        edge_label.x = Some(x - edge_label.width / 2.0 - edge_label.labeloffset);
                    }
                    LabelPosition::Right => {
                        edge_label.x = Some(x + edge_label.width / 2.0 + edge_label.labeloffset);
                    }
                    LabelPosition::Center => {
                        // 居中，不需要调整
                    }
                    LabelPosition::Top => {
                        // 上方标签不需要调整
                    }
                    LabelPosition::Bottom => {
                        // 下方标签不需要调整
                    }
                }
            }
        }
    }
}

/// 撤销坐标系统调整
///
/// 对应 JS 函数: coordinateSystem.undo() in lib/coordinate-system.js
fn undo_coordinate_system(graph: &mut Graph) {
    // 获取图的配置
    let rank_dir = "TB"; // 简化实现，使用默认值

    // 根据层级方向调整坐标
    let node_indices: Vec<NodeIndex> = graph.node_indices().collect();
    for node_id in node_indices {
        if let Some(node_label) = graph.node_label_mut(node_id) {
            if let (Some(x), Some(y)) = (node_label.x, node_label.y) {
                match rank_dir {
                    "BT" => {
                        // 从下到上，需要翻转Y坐标
                        node_label.y = Some(-y);
                    }
                    "RL" => {
                        // 从右到左，交换X和Y坐标
                        node_label.x = Some(y);
                        node_label.y = Some(-x);
                    }
                    "LR" => {
                        // 从左到右，交换X和Y坐标
                        node_label.x = Some(y);
                        node_label.y = Some(x);
                    }
                    _ => {
                        // "TB" 默认情况，不需要调整
                    }
                }
            }
        }
    }
}

/// 分配节点交点
///
/// 对应 JS 函数: assignNodeIntersects() in lib/layout.js
pub fn assign_node_intersects(graph: &mut Graph) {
    let mut edge_updates = Vec::new();

    for edge in graph.edges() {
        if let (Some(source_label), Some(target_label)) =
            (graph.node_label(edge.source), graph.node_label(edge.target))
        {
            let mut points = Vec::new();

            // 添加起点相交点
            if let (Some(source_x), Some(source_y)) = (source_label.x, source_label.y) {
                let intersect = crate::util::intersect_rect(
                    source_label,
                    &crate::types::Point::new(source_x, source_y),
                );
                points.push(intersect);
            }

            // 添加终点相交点
            if let (Some(target_x), Some(target_y)) = (target_label.x, target_label.y) {
                let intersect = crate::util::intersect_rect(
                    target_label,
                    &crate::types::Point::new(target_x, target_y),
                );
                points.push(intersect);
            }

            edge_updates.push((edge, points));
        }
    }

    for (edge, points) in edge_updates {
        if let Some(edge_label) = graph.edge_label_mut(&edge) {
            edge_label.points = points;
        }
    }
}

/// 反转边的点
///
/// 对应 JS 函数: reversePointsForReversedEdges() in lib/layout.js
fn reverse_points_for_reversed_edges(graph: &mut Graph) {
    for edge in graph.edges() {
        if let Some(_edge_label) = graph.edge_label_mut(&edge) {
            // 简化实现，不再使用custom字段
        }
    }
}

/// 撤销无环化
///
/// 对应 JS 函数: acyclic.undo() in lib/acyclic.js
fn acyclic_undo(graph: &mut Graph) {
    // 恢复被反转的边
    let edges_to_reverse: Vec<Edge> = graph
        .edges()
        .into_iter()
        .filter(|edge| {
            if let Some(edge_label) = graph.edge_label(edge) {
                edge_label.reversed.unwrap_or(false)
            } else {
                false
            }
        })
        .collect();

    for edge in edges_to_reverse {
        // 移除当前边
        if let Some(edge_label) = graph.edge_label(&edge) {
            let mut new_edge_label = edge_label.clone();
            new_edge_label.reversed = Some(false);

            // 添加反向边
            graph.add_edge(Edge::new(edge.target, edge.source), new_edge_label);
            graph.remove_edge(&edge);
        }
    }
}

/// 更新输入图
///
/// 对应 JS 函数: updateInputGraph() in lib/layout.js
fn update_input_graph(input_graph: &mut Graph, layout_graph: &Graph) {
    // 由于现在使用 NodeIndex，我们需要通过索引来匹配节点
    // 这里简化实现，假设两个图的节点顺序相同
    let input_nodes: Vec<_> = input_graph.node_indices().collect();
    let layout_nodes: Vec<_> = layout_graph.node_indices().collect();

    for (i, &input_node) in input_nodes.iter().enumerate() {
        if let Some(layout_node) = layout_nodes.get(i) {
            if let (Some(input_label), Some(layout_label)) = (
                input_graph.node_label_mut(input_node),
                layout_graph.node_label(*layout_node),
            ) {
                input_label.x = layout_label.x;
                input_label.y = layout_label.y;
                input_label.rank = layout_label.rank;

                if layout_graph.children(*layout_node).len() > 0 {
                    input_label.width = layout_label.width;
                    input_label.height = layout_label.height;
                }
            }
        }
    }

    // 复制边信息 - 简化实现，假设边顺序相同
    let input_edges: Vec<_> = input_graph.edges();
    let layout_edges: Vec<_> = layout_graph.edges();

    for (i, input_edge) in input_edges.iter().enumerate() {
        if let Some(layout_edge) = layout_edges.get(i) {
            if let (Some(input_edge_label), Some(layout_edge_label)) = (
                input_graph.edge_label_mut(input_edge),
                layout_graph.edge_label(layout_edge),
            ) {
                input_edge_label.points = layout_edge_label.points.clone();
                input_edge_label.x = layout_edge_label.x;
                input_edge_label.y = layout_edge_label.y;
            }
        }
    }

    // 复制图信息
    let input_config = input_graph.config_mut();
    let layout_config = layout_graph.config();
    input_config.width = layout_config.width;
    input_config.height = layout_config.height;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_single_node() {
        let mut graph = Graph::new();
        let node_a = graph.add_node(NodeLabel {
            width: 50.0,
            height: 100.0,
            ..Default::default()
        });

        layout(&mut graph, None);

        let label = graph.node_label(node_a).unwrap();
        assert!(label.x.is_some());
        assert!(label.y.is_some());
    }

    #[test]
    fn test_layout_two_nodes() {
        let mut graph = Graph::new();
        let node_a: NodeIndex = graph.add_node(NodeLabel {
            width: 50.0,
            height: 100.0,
            ..Default::default()
        });
        let node_b = graph.add_node(NodeLabel {
            width: 75.0,
            height: 200.0,
            ..Default::default()
        });

        let edge = Edge::new(node_a, node_b);
        let _ = graph.add_edge(edge, EdgeLabel::default());

        layout(&mut graph, None);

        let label_a = graph.node_label(node_a).unwrap();
        let label_b = graph.node_label(node_b).unwrap();

        assert!(label_a.x.is_some());
        assert!(label_a.y.is_some());
        assert!(label_b.x.is_some());
        assert!(label_b.y.is_some());
    }
}

/// 运行嵌套图算法
///
/// 对应 JS 函数: nestingGraph.run() in lib/nesting-graph.js
fn nesting_graph_run(graph: &mut Graph) {
    // 添加根节点
    let mut root_label = NodeLabel::default();
    root_label.dummy = Some(Dummy::Root);
    root_label.width = 0.0;
    root_label.height = 0.0;
    let root = graph.add_node(root_label);

    // 为所有现有节点添加到根节点的边
    let existing_nodes: Vec<NodeIndex> = graph.node_indices().filter(|&id| id != root).collect();
    for node_id in existing_nodes {
        let edge = Edge::new(root, node_id);
        let mut edge_label = EdgeLabel::default();
        edge_label.weight = 1.0;
        graph.add_edge(edge, edge_label);
    }
}

/// 清理嵌套图
///
/// 对应 JS 函数: nestingGraph.cleanup() in lib/nesting-graph.js
fn nesting_graph_cleanup(graph: &mut Graph) {
    // 移除根节点
    let root_nodes: Vec<NodeIndex> = graph
        .node_indices()
        .filter(|&id| {
            if let Some(label) = graph.node_label(id) {
                label
                    .dummy
                    .as_ref()
                    .map_or(false, |d| matches!(d, Dummy::Root))
            } else {
                false
            }
        })
        .collect();

    for root in root_nodes {
        graph.remove_node(root);
    }
}
