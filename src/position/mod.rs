//! 位置计算算法模块

pub mod bk;

use crate::counters::*;
use crate::graph::Graph;
use crate::util::build_layer_matrix;

/// 计算节点位置
///
/// 对应 JS 函数: position() in lib/position/index.js
pub fn position(graph: &mut Graph) {
    // 转换为非复合图，只处理叶子节点
    let (mut simplified_graph, old_to_new) = crate::util::as_non_compound_graph(graph);

    position_y(&mut simplified_graph);

    let bk_result = {
        increment_bk();
        let mut bk = bk::BrandesKoepf::new(simplified_graph.clone());
        bk.run()
    };

    // 将坐标分配回原图
    for (new_node_id, position) in bk_result.positions {
        // 找到对应的原节点ID
        if let Some((&old_node_id, _)) = old_to_new
            .iter()
            .find(|&(_, &new_id)| new_id == new_node_id)
        {
            if let Some(label) = graph.node_label_mut(old_node_id) {
                label.x = Some(position.position);
            }
        }
    }

    // 将Y坐标也分配回原图
    for new_node_id in simplified_graph.node_indices() {
        if let Some(simplified_label) = simplified_graph.node_label(new_node_id) {
            // 找到对应的原节点ID
            if let Some((&old_node_id, _)) = old_to_new
                .iter()
                .find(|&(_, &new_id)| new_id == new_node_id)
            {
                if let Some(label) = graph.node_label_mut(old_node_id) {
                    label.y = simplified_label.y;
                }
            }
        }
    }
}

/// 计算Y坐标
///
/// 对应 JS 函数: positionY() in lib/position/index.js
pub fn position_y(graph: &mut Graph) {
    let layering = build_layer_matrix(graph);
    let rank_sep = graph.config().rank_sep;
    let mut prev_y = 0.0;

    for layer in layering {
        if !layer.is_empty() {
            // 计算该层中节点的最大高度
            let max_height = layer
                .iter()
                .filter_map(|node_id| graph.node_label(*node_id))
                .map(|label| label.height)
                .fold(0.0, f64::max);

            // 设置该层所有节点的Y坐标
            for node_id in layer {
                if let Some(label) = graph.node_label_mut(node_id) {
                    label.y = Some(prev_y + max_height / 2.0);
                }
            }

            // 更新下一层的起始Y坐标
            prev_y += max_height + rank_sep;
        }
    }
}

/// 计算边路径点
pub fn position_edges(graph: &mut Graph) {
    // 收集所有需要处理的边数据，避免借用冲突
    let mut edge_data = Vec::new();

    for edge in graph.edges() {
        if let Some(edge_label) = graph.edge_label(&edge) {
            if let (Some(source_label), Some(target_label)) =
                (graph.node_label(edge.source), graph.node_label(edge.target))
            {
                edge_data.push((
                    edge,
                    source_label.clone(),
                    target_label.clone(),
                    edge_label.clone(),
                ));
            }
        }
    }

    // 处理收集到的边数据
    for (edge, source, target, edge_label_data) in edge_data {
        if let Some(edge_label) = graph.edge_label_mut(&edge) {
            let points = calculate_edge_points(&source, &target, &edge_label_data);
            edge_label.points = points;
        }
    }
}

/// 计算边的路径点
fn calculate_edge_points(
    source: &crate::types::NodeLabel,
    target: &crate::types::NodeLabel,
    edge_label: &crate::types::EdgeLabel,
) -> Vec<crate::types::Point> {
    let mut points = Vec::new();

    if let (Some(source_x), Some(source_y), Some(target_x), Some(target_y)) =
        (source.x, source.y, target.x, target.y)
    {
        // 计算边的起点和终点
        let start_point = crate::types::Point::new(source_x, source_y);
        let end_point = crate::types::Point::new(target_x, target_y);

        // 添加起点
        points.push(start_point);

        // 如果是直线，直接添加终点
        if edge_label.points.is_empty() {
            points.push(end_point);
        } else {
            // 使用预定义的路径点
            points.extend(edge_label.points.clone());
        }
    }

    points
}

/// 调整坐标系统
pub fn adjust_coordinate_system(graph: &mut Graph) {
    let config = graph.config();

    match config.rankdir {
        crate::types::RankDirection::TopBottom => {
            // 默认方向，不需要调整
        }
        crate::types::RankDirection::BottomTop => {
            // 翻转Y坐标
            flip_y_coordinates(graph);
        }
        crate::types::RankDirection::LeftRight => {
            // 交换X和Y坐标
            swap_xy_coordinates(graph);
        }
        crate::types::RankDirection::RightLeft => {
            // 交换X和Y坐标，然后翻转X坐标
            swap_xy_coordinates(graph);
            flip_x_coordinates(graph);
        }
    }
}

/// 翻转Y坐标
fn flip_y_coordinates(graph: &mut Graph) {
    // 先收集所有Y坐标，避免借用冲突
    let max_y = graph
        .node_indices()
        .filter_map(|id| graph.node_label(id)?.y)
        .fold(0.0, f64::max);

    // 收集需要更新的节点
    let mut nodes_to_update = Vec::new();
    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            if let Some(y) = label.y {
                nodes_to_update.push((node_id, max_y - y));
            }
        }
    }

    // 更新节点坐标
    for (node_id, new_y) in nodes_to_update {
        if let Some(label) = graph.node_label_mut(node_id) {
            label.y = Some(new_y);
        }
    }
}

/// 翻转X坐标
fn flip_x_coordinates(graph: &mut Graph) {
    // 先收集所有X坐标，避免借用冲突
    let max_x = graph
        .node_indices()
        .filter_map(|id| graph.node_label(id)?.x)
        .fold(0.0, f64::max);

    // 收集需要更新的节点
    let mut nodes_to_update = Vec::new();
    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            if let Some(x) = label.x {
                nodes_to_update.push((node_id, max_x - x));
            }
        }
    }

    // 更新节点坐标
    for (node_id, new_x) in nodes_to_update {
        if let Some(label) = graph.node_label_mut(node_id) {
            label.x = Some(new_x);
        }
    }
}

/// 交换X和Y坐标
fn swap_xy_coordinates(graph: &mut Graph) {
    // 收集需要更新的节点，避免借用冲突
    let mut nodes_to_update = Vec::new();

    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            if let (Some(x), Some(y)) = (label.x, label.y) {
                nodes_to_update.push((node_id, y, x));
            }
        }
    }

    // 应用更新
    for (node_id, new_x, new_y) in nodes_to_update {
        if let Some(label) = graph.node_label_mut(node_id) {
            label.x = Some(new_x);
            label.y = Some(new_y);
        }
    }
}

/// 平移图到原点
pub fn translate_graph(graph: &mut Graph) {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;

    // 找到最小坐标
    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            if let (Some(x), Some(y)) = (label.x, label.y) {
                min_x = min_x.min(x - label.width / 2.0);
                min_y = min_y.min(y - label.height / 2.0);
            }
        }
    }

    // 平移所有节点
    for node_id in graph.node_indices().collect::<Vec<_>>() {
        if let Some(label) = graph.node_label_mut(node_id) {
            if let (Some(x), Some(y)) = (label.x, label.y) {
                label.x = Some(x - min_x);
                label.y = Some(y - min_y);
            }
        }
    }

    // 平移所有边的控制点
    for edge in graph.edges() {
        if let Some(edge_label) = graph.edge_label_mut(&edge) {
            for point in &mut edge_label.points {
                point.x -= min_x;
                point.y -= min_y;
            }
        }
    }

    // 更新图尺寸
    let mut max_x: f64 = 0.0;
    let mut max_y: f64 = 0.0;

    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            if let (Some(x), Some(y)) = (label.x, label.y) {
                max_x = max_x.max(x + label.width / 2.0);
                max_y = max_y.max(y + label.height / 2.0);
            }
        }
    }

    let config = graph.config_mut();
    config.width = Some(max_x + config.marginx);
    config.height = Some(max_y + config.marginy);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    #[test]
    fn test_position_y() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeLabel::default());
        let b = graph.add_node(NodeLabel::default());

        // 设置排名
        if let Some(label_a) = graph.node_label_mut(a) {
            label_a.rank = Some(0);
            label_a.height = 50.0;
        }
        if let Some(label_b) = graph.node_label_mut(b) {
            label_b.rank = Some(1);
            label_b.height = 100.0;
        }

        position_y(&mut graph);

        assert!(graph.node_label(a).unwrap().y.is_some());
        assert!(graph.node_label(b).unwrap().y.is_some());
    }

    #[test]
    fn test_translate_graph() {
        let mut graph = Graph::new();
        let a = graph.add_node(NodeLabel::default());

        // 设置位置
        if let Some(label_a) = graph.node_label_mut(a) {
            label_a.x = Some(100.0);
            label_a.y = Some(200.0);
            label_a.width = 50.0;
            label_a.height = 30.0;
        }

        translate_graph(&mut graph);

        let final_label = graph.node_label(a).unwrap();
        assert!(final_label.x.unwrap() >= 0.0);
        assert!(final_label.y.unwrap() >= 0.0);
    }
}
