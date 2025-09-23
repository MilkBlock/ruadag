//! 节点排序算法模块

pub mod barycenter;
pub mod build_layer_graph;
pub mod constraint_graph;
pub mod cross_count;
pub mod init_order;
pub mod sort_subgraph;

use crate::LayoutOptions;
use crate::counters::*;
use crate::graph::Graph;
use crate::graph::NodeIndex;
use crate::util::{build_layer_matrix, is_placeholder, max_rank, range, range_with_step, time};

use build_layer_graph::{LayerGraph, build_layer_graph};
use constraint_graph::{ConstraintGraph, add_subgraph_constraints};

/// 为图中的节点分配顺序以最小化边交叉
///
/// 对应 JS 函数: order() in lib/order/index.js
pub fn order(graph: &mut Graph, opts: Option<&LayoutOptions>) {
    let default = LayoutOptions::default();
    let opts = opts.unwrap_or(&default);

    // 构建层级信息 - 与JS版本一致，不进行额外的默认排序
    let _layers = build_layers(graph);

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

    // 优化排序以减少交叉 - 与JS版本保持一致的动态停止机制
    let mut best_cc = usize::MAX;
    let mut best_layering = layering;
    let mut last_best = 0;

    for i in 0.. {
        // 修复层级图选择逻辑，与JS版本一致
        let layer_graphs = if i % 2 == 1 {
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
            last_best = 0; // 重置计数器，与JS版本一致
            best_cc = cc;
            best_layering = current_layering;
        } else {
            last_best += 1;
        }

        // 连续4次无改进则停止，与JS版本一致
        if last_best >= 4 {
            break;
        }
    }

    // 分配最佳排序
    assign_order(graph, &best_layering);
}

/// 构建层级图
fn build_layer_graphs(graph: &Graph, ranks: &[i32], relationship: &str) -> Vec<LayerGraph> {
    ranks
        .iter()
        .map(|&rank| build_layer_graph(graph, rank, relationship))
        .collect()
}

/// 扫描层级图
fn sweep_layer_graphs(layer_graphs: &mut [LayerGraph], bias_right: bool) {
    let mut constraint_graph = ConstraintGraph::new();

    for layer_graph in layer_graphs {
        // 跳过空的层级图
        if layer_graph.graph.node_count() == 0 {
            continue;
        }

        // 在层级图中找到根节点
        let root = find_root(&layer_graph.graph);

        // 在层级图中进行排序，传递约束图
        let sort_result =
            sort_subgraph::sort_subgraph(&layer_graph.graph, root, &constraint_graph, bias_right);

        // 分配顺序到层级图
        for (i, layer_node_id) in sort_result.vs.iter().enumerate() {
            // 确保节点ID属于层级图
            if layer_node_id.belongs_to_graph(layer_graph.graph.graph_id()) {
                if let Some(label) = layer_graph.graph.node_label_mut(*layer_node_id) {
                    label.order = Some(i);
                }
            } else {
                panic!(
                    "Layer node ID {:?} does not belong to layer graph {}\nLayer graph debug info: {}\nNode debug info: belongs_to_graph({}) = {}",
                    layer_node_id,
                    layer_graph.graph.graph_id(),
                    layer_graph.graph.debug_info(),
                    layer_graph.graph.graph_id(),
                    layer_node_id.belongs_to_graph(layer_graph.graph.graph_id())
                );
            }
        }

        // 添加子图约束
        add_subgraph_constraints(&layer_graph.graph, &mut constraint_graph, &sort_result.vs);
    }
}

/// 分配顺序到图
fn assign_order(graph: &mut Graph, layering: &Vec<Vec<NodeIndex>>) {
    for layer in layering {
        for (i, node_id) in layer.iter().enumerate() {
            // 跳过占位符节点
            if is_placeholder(*node_id) {
                continue;
            }
            // 检查节点是否属于当前图
            if node_id.belongs_to_graph(graph.graph_id()) {
                if let Some(label) = graph.node_label_mut(*node_id) {
                    label.order = Some(i);
                }
            }
        }
    }
}

/// 查找根节点
fn find_root(graph: &Graph) -> NodeIndex {
    // 查找入度为0的节点作为根节点
    for node_id in graph.node_indices() {
        if graph.predecessors(node_id).into_iter().count() == 0 {
            return node_id;
        }
    }

    // 如果没有入度为0的节点，返回第一个节点
    if let Some(first_node) = graph.node_indices().next() {
        first_node
    } else {
        // 如果图是空的，这是一个错误情况
        panic!("Cannot find root node in empty graph");
    }
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
