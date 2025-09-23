//! 节点排序算法模块

pub mod build_layer_graph;
pub mod cross_count;
pub mod init_order;
pub mod sort_subgraph;

use crate::counters::*;
use crate::graph::Graph;
use crate::util::{build_layer_matrix, max_rank, range, range_with_step, time};
use crate::LayoutOptions;
use petgraph::graph::NodeIndex;

/// 为图中的节点分配顺序以最小化边交叉
///
/// 对应 JS 函数: order() in lib/order/index.js
pub fn order(graph: &mut Graph, opts: Option<&LayoutOptions>) {
    let default = LayoutOptions::default();
    let opts = opts.unwrap_or(&default);

    // 构建层级信息
    let mut layers = build_layers(graph);

    // 检查是否有自定义排序函数
    if let Some(custom_order) = &opts.custom.get("custom_order") {
        // 调用自定义排序函数
        if let Some(custom_fn) = custom_order.as_str() {
            match custom_fn {
                "custom_sort" => {
                    // 执行自定义排序逻辑
                    custom_sort_layers(graph, &mut layers);
                }
                _ => {
                    // 默认排序
                    default_sort_layers(graph, &mut layers);
                }
            }
        }
    } else {
        // 使用默认排序
        default_sort_layers(graph, &mut layers);
    }

    let max_rank = max_rank(graph);

    // 构建层级图
    let mut down_layer_graphs = build_layer_graphs(graph, &range(1, max_rank + 1), "in_edges");
    let mut up_layer_graphs =
        build_layer_graphs(graph, &range_with_step(max_rank - 1, -1, -1), "out_edges");

    // 初始化排序
    let layering = time("init_order", || {
        increment_init_order();
        init_order::init_order(graph)
    });

    // 分配初始顺序
    assign_order(graph, &layering);

    // 如果禁用了最优排序启发式，直接返回
    if opts.disable_optimal_order_heuristic {
        return;
    }

    // 优化排序以减少交叉
    let mut best_cc = usize::MAX;
    let mut best_layering = layering;

    for i in 0..4 {
        let layer_graphs = if i % 2 == 0 {
            &mut down_layer_graphs
        } else {
            &mut up_layer_graphs
        };
        let bias_right = i % 4 >= 2;

        sweep_layer_graphs(layer_graphs, bias_right);

        let current_layering = build_layer_matrix(graph);
        let cc = {
            increment_cross_count();
            cross_count::cross_count(graph, &current_layering)
        };

        if cc < best_cc {
            best_cc = cc;
            best_layering = current_layering;
        }
    }

    // 分配最佳排序
    assign_order(graph, &best_layering);
}

/// 构建层级图
fn build_layer_graphs(graph: &Graph, ranks: &[i32], relationship: &str) -> Vec<Graph> {
    ranks
        .iter()
        .map(|&rank| build_layer_graph::build_layer_graph(graph, rank, relationship))
        .collect()
}

/// 扫描层级图
fn sweep_layer_graphs(layer_graphs: &mut [Graph], bias_right: bool) {
    for layer_graph in layer_graphs {
        let root = find_root(layer_graph);
        let sorted = sort_subgraph::sort_subgraph(layer_graph, root, bias_right);

        // 分配顺序
        for (i, node_id) in sorted.iter().enumerate() {
            if let Some(label) = layer_graph.node_label_mut(*node_id) {
                label.order = Some(i);
            }
        }

        // 添加子图约束（暂时省略，功能已移除）
    }
}

/// 分配顺序到图
fn assign_order(graph: &mut Graph, layering: &Vec<Vec<NodeIndex>>) {
    for layer in layering {
        for (i, node_id) in layer.iter().enumerate() {
            if let Some(label) = graph.node_label_mut(*node_id) {
                label.order = Some(i);
            }
        }
    }
}

/// 查找根节点
fn find_root(graph: &Graph) -> NodeIndex {
    // 查找入度为0的节点作为根节点
    for node_id in graph.node_indices() {
        let predecessors: Vec<NodeIndex> = graph.predecessors(node_id).collect();
        if predecessors.is_empty() {
            return node_id;
        }
    }

    // 如果没有入度为0的节点，返回第一个节点
    graph.node_indices().next().unwrap_or_default()
}

/// 默认排序层级
fn default_sort_layers(graph: &mut Graph, layers: &mut Vec<Vec<NodeIndex>>) {
    // 使用简单的拓扑排序
    for layer in layers.iter_mut() {
        layer.sort_by_key(|&node_id| {
            graph
                .node_label(node_id)
                .and_then(|label| label.order)
                .unwrap_or(0)
        });
    }
}

/// 自定义排序层级
fn custom_sort_layers(graph: &mut Graph, layers: &mut Vec<Vec<NodeIndex>>) {
    // 使用更复杂的排序算法，比如基于交叉数的最小化
    for layer in layers.iter_mut() {
        // 这里可以实现更复杂的排序逻辑
        // 比如基于边的交叉数进行排序
        layer.sort_by_key(|&node_id| {
            // 计算节点的权重进行排序
            calculate_node_weight(graph, node_id)
        });
    }
}

/// 计算节点权重
fn calculate_node_weight(graph: &Graph, node_id: NodeIndex) -> i32 {
    let mut weight = 0;

    // 基于连接数计算权重
    let successors: Vec<NodeIndex> = graph.successors(node_id).collect();
    weight += successors.len() as i32;

    let predecessors: Vec<NodeIndex> = graph.predecessors(node_id).collect();
    weight += predecessors.len() as i32;

    weight
}

/// 构建层级信息
fn build_layers(graph: &Graph) -> Vec<Vec<NodeIndex>> {
    let mut layers: Vec<Vec<NodeIndex>> = Vec::new();
    let mut max_rank = 0;

    // 找到最大层级
    for node_id in graph.node_indices() {
        if let Some(node_label) = graph.node_label(node_id) {
            if let Some(rank) = node_label.rank {
                max_rank = max_rank.max(rank);
            }
        }
    }

    // 按层级组织节点
    for rank in 0..=max_rank {
        let mut layer_nodes = Vec::new();
        for node_id in graph.node_indices() {
            if let Some(node_label) = graph.node_label(node_id) {
                if node_label.rank == Some(rank) {
                    layer_nodes.push(node_id);
                }
            }
        }

        // 按order排序
        layer_nodes.sort_by_key(|&node_id| {
            graph
                .node_label(node_id)
                .and_then(|label| label.order)
                .unwrap_or(0)
        });

        layers.push(layer_nodes);
    }

    layers
}
