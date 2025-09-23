//! 工具函数

use crate::graph::Graph;
use crate::graph::NodeIndex;
use crate::types::*;
use indexmap::IndexMap;
use std::str::FromStr;
use std::u32;

/// 检查节点是否为占位符
///
/// 在 JavaScript 版本中，占位符通过 `dummy` 属性识别
/// 在 Rust 版本中，我们使用特殊的图ID (0) 来标识占位符
pub fn is_placeholder(node_id: NodeIndex) -> bool {
    node_id.which_graph == u32::MAX
}

/// 构建层级矩阵
/// 返回 Vec<Vec<NodeIndex>>，其中索引是 rank
pub fn build_layer_matrix(graph: &Graph) -> Vec<Vec<NodeIndex>> {
    let min_rank = min_rank(graph);
    let max_rank = max_rank(graph);
    let rank_count = (max_rank - min_rank + 1) as usize;
    let mut layering: Vec<Vec<NodeIndex>> = (0..rank_count).map(|_| Vec::new()).collect();

    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            if let Some(rank) = label.rank {
                let adjusted_rank = (rank - min_rank) as usize;
                if adjusted_rank < layering.len() {
                    let layer = &mut layering[adjusted_rank];

                    // 根据 order 属性插入到正确位置
                    if let Some(order) = label.order {
                        // 确保向量足够大
                        while layer.len() <= order as usize {
                            // 创建占位符节点，使用特殊的图ID来标识占位符
                            let placeholder = NodeIndex::new_raw(usize::MAX, u32::MAX);
                            layer.push(placeholder);
                        }
                        // 使用原始节点ID，但确保它属于正确的图
                        layer[order as usize] = node_id;
                    } else {
                        // 如果没有 order，直接 push
                        layer.push(node_id);
                    }
                }
            }
        }
    }

    // 清理占位符并保持顺序
    for layer in layering.iter_mut() {
        layer.retain(|&node_id| !is_placeholder(node_id));
    }

    layering
}

/// 获取最小排名
pub fn min_rank(graph: &Graph) -> i32 {
    graph
        .node_indices()
        .filter_map(|id| graph.node_label(id)?.rank)
        .min()
        .unwrap_or(0)
}

/// 获取最大排名
pub fn max_rank(graph: &Graph) -> i32 {
    graph
        .node_indices()
        .filter_map(|id| graph.node_label(id)?.rank)
        .max()
        .unwrap_or(0)
}

/// 生成数字范围
pub fn range(start: i32, end: i32) -> Vec<i32> {
    (start..end).collect()
}

/// 生成数字范围（带步长）
pub fn range_with_step(start: i32, end: i32, step: i32) -> Vec<i32> {
    if step > 0 {
        (start..end).step_by(step as usize).collect()
    } else {
        (end..start).step_by((-step) as usize).rev().collect()
    }
}

/// 选择数字属性
pub fn select_number_attrs<T>(obj: &T, _attrs: &[&str]) -> IndexMap<String, f64>
where
    T: serde::Serialize,
{
    // 从对象中提取数字属性
    let mut result = IndexMap::new();

    // 尝试从序列化对象中提取数字属性
    if let Ok(serialized) = serde_json::to_value(obj) {
        if let serde_json::Value::Object(map) = serialized {
            for (key, value) in map {
                if let Some(num) = value.as_f64() {
                    result.insert(key, num);
                }
            }
        }
    }

    result
}

/// 选择指定属性
pub fn pick<T>(obj: &T, attrs: &[&str]) -> IndexMap<String, serde_json::Value>
where
    T: serde::Serialize,
{
    // 从对象中选择指定属性
    let mut result = IndexMap::new();

    // 尝试从序列化对象中选择指定属性
    if let Ok(serialized) = serde_json::to_value(obj) {
        if let serde_json::Value::Object(map) = serialized {
            for attr in attrs {
                if let Some(value) = map.get(*attr) {
                    result.insert(attr.to_string(), value.clone());
                }
            }
        }
    }

    result
}

/// 映射值
pub fn map_values<T, F>(map: IndexMap<String, T>, f: F) -> IndexMap<String, T>
where
    F: Fn(T) -> T,
{
    map.into_iter().map(|(k, v)| (k, f(v))).collect()
}

/// 规范化排名
pub fn normalize_ranks(graph: &mut Graph) {
    // 先收集最小排名，避免借用冲突
    let min_rank = graph
        .node_indices()
        .filter_map(|id| graph.node_label(id)?.rank)
        .min()
        .unwrap_or(0);

    // 收集需要更新的节点
    let mut nodes_to_update = Vec::new();
    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            if let Some(rank) = label.rank {
                nodes_to_update.push((node_id, rank - min_rank));
            }
        }
    }

    // 更新节点排名
    for (node_id, new_rank) in nodes_to_update {
        if let Some(label) = graph.node_label_mut(node_id) {
            label.rank = Some(new_rank);
        }
    }
}

/// 移除空排名
pub fn remove_empty_ranks(graph: &mut Graph) {
    let layers = build_layer_matrix(graph);
    let mut rank_shift = IndexMap::new();
    let mut current_rank = 0;

    for rank in 0..=max_rank(graph) {
        if (rank as usize) < layers.len() && !layers[rank as usize].is_empty() {
            rank_shift.insert(rank, current_rank);
            current_rank += 1;
        }
    }

    // 收集需要更新的节点
    let mut nodes_to_update = Vec::new();
    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            if let Some(rank) = label.rank {
                if let Some(&new_rank) = rank_shift.get(&rank) {
                    nodes_to_update.push((node_id, new_rank));
                }
            }
        }
    }

    // 更新节点排名
    for (node_id, new_rank) in nodes_to_update {
        if let Some(label) = graph.node_label_mut(node_id) {
            label.rank = Some(new_rank);
        }
    }
}

/// 添加虚拟节点
pub fn add_dummy_node(
    graph: &mut Graph,
    dummy_type: &str,
    label: NodeLabel,
    prefix: &str,
) -> String {
    let dummy_id = format!("{}{}", prefix, graph.node_count());
    let mut dummy_label = label;
    dummy_label.dummy = Some(Dummy::from_str(dummy_type).unwrap_or(Dummy::Edge));
    graph.add_node(dummy_label);
    dummy_id
}

/// 计算矩形相交点
pub fn intersect_rect(node: &NodeLabel, point: &Point) -> Point {
    let x = node.x.unwrap_or(0.0);
    let y = node.y.unwrap_or(0.0);
    let w = node.width / 2.0;
    let h = node.height / 2.0;

    let dx = point.x - x;
    let dy = point.y - y;

    if dx == 0.0 && dy == 0.0 {
        // 如果点在矩形中心，返回中心点
        return Point::new(x, y);
    }

    let sx: f64;
    let sy: f64;

    if dy.abs() * w > dx.abs() * h {
        // 相交点在矩形的顶部或底部
        if dy < 0.0 {
            sy = y - h;
        } else {
            sy = y + h;
        }
        sx = x + (dx * h) / dy.abs();
    } else {
        // 相交点在矩形的左侧或右侧
        if dx < 0.0 {
            sx = x - w;
        } else {
            sx = x + w;
        }
        sy = y + (dy * w) / dx.abs();
    }

    Point::new(sx, sy)
}

/// 时间测量工具
pub struct Timer {
    start: std::time::Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: std::time::Instant::now(),
        }
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.start.elapsed()
    }
}

/// 执行带时间测量的操作
pub fn time<F, R>(name: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let timer = Timer::new();
    let result = f();
    let elapsed = timer.elapsed();
    log::debug!("{} took {:?}", name, elapsed);
    result
}

/// 空操作（用于禁用时间测量）
pub fn no_time<F, R>(_name: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    f()
}

/// 作为非复合图处理
pub fn as_non_compound_graph(graph: &Graph) -> &Graph {
    // 对于复合图，这里应该返回一个简化的非复合版本
    // 目前简化实现，直接返回原图
    // 在实际实现中，这里应该移除复合图的层次结构
    graph
}

/// 计算交叉数
pub fn cross_count(graph: &Graph, layering: &IndexMap<i32, Vec<NodeIndex>>) -> usize {
    let mut crossings = 0;

    for rank in 0..max_rank(graph) {
        if let Some(layer) = layering.get(&rank) {
            for i in 0..layer.len() {
                for j in (i + 1)..layer.len() {
                    let v1 = layer[i];
                    let v2 = layer[j];

                    // 计算v1和v2之间的边交叉数
                    let v1_edges = graph.out_edges(v1);
                    let v2_edges = graph.out_edges(v2);

                    for edge1 in &v1_edges {
                        for edge2 in &v2_edges {
                            if edge1.target != edge2.target {
                                // 检查边是否交叉
                                // 这里需要更复杂的几何计算
                                crossings += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    crossings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        assert_eq!(range(0, 5), vec![0, 1, 2, 3, 4]);
        assert_eq!(range_with_step(0, 10, 2), vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_intersect_rect() {
        let node = NodeLabel {
            x: Some(0.0),
            y: Some(0.0),
            width: 100.0,
            height: 50.0,
            ..Default::default()
        };
        let point = Point::new(200.0, 100.0);
        let intersection = intersect_rect(&node, &point);

        assert!(intersection.x >= 0.0 && intersection.x <= 100.0);
        assert!(intersection.y >= 0.0 && intersection.y <= 50.0);
    }
}
