//! 图数据结构实现，基于 petgraph

use crate::types::*;
use indexmap::IndexMap;
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::{Directed, Graph as PetGraph};

pub struct SrcIdx(NodeIndex);

/// 图结构，基于 petgraph
#[derive(Clone)]
pub struct Graph {
    /// petgraph 图实例
    graph: PetGraph<NodeLabel, EdgeLabel, Directed>,
    /// 边到EdgeIndex的映射
    edge_to_index: IndexMap<Edge, EdgeIndex>,
    /// EdgeIndex到边的映射
    index_to_edge: IndexMap<EdgeIndex, Edge>,
    /// 图配置
    config: GraphConfig,
    /// 虚拟链
    pub dummy_chains: Vec<NodeIndex>,
}

impl Graph {
    /// 创建新的空图
    pub fn new() -> Self {
        Self {
            graph: PetGraph::new(),
            edge_to_index: IndexMap::new(),
            index_to_edge: IndexMap::new(),
            config: GraphConfig::default(),
            dummy_chains: Vec::new(),
        }
    }

    /// 创建带配置的图
    pub fn with_config(config: GraphConfig) -> Self {
        Self {
            graph: PetGraph::new(),
            edge_to_index: IndexMap::new(),
            index_to_edge: IndexMap::new(),
            config,
            dummy_chains: Vec::new(),
        }
    }

    /// 添加节点
    pub fn add_node(&mut self, label: NodeLabel) -> NodeIndex {
        self.graph.add_node(label)
    }

    /// 添加边
    pub fn add_edge(&mut self, edge: Edge, label: EdgeLabel) -> EdgeIndex {
        let edge_index = self.graph.add_edge(edge.source, edge.target, label);
        self.edge_to_index.insert(edge.clone(), edge_index);
        self.index_to_edge.insert(edge_index, edge);
        edge_index
    }

    /// 获取节点标签
    pub fn node_label(&self, node_index: NodeIndex) -> Option<&NodeLabel> {
        self.graph.node_weight(node_index)
    }

    /// 获取节点标签（可变）
    pub fn node_label_mut(&mut self, node_index: NodeIndex) -> Option<&mut NodeLabel> {
        self.graph.node_weight_mut(node_index)
    }

    /// 获取边标签
    pub fn edge_label(&self, edge: &Edge) -> Option<&EdgeLabel> {
        let edge_index = self.edge_to_index.get(edge)?;
        self.graph.edge_weight(*edge_index)
    }

    /// 获取边标签（可变）
    pub fn edge_label_mut(&mut self, edge: &Edge) -> Option<&mut EdgeLabel> {
        let edge_index = self.edge_to_index.get(edge)?;
        self.graph.edge_weight_mut(*edge_index)
    }

    /// 获取所有节点索引
    pub fn node_indices(&self) -> impl Iterator<Item = NodeIndex> + '_ {
        self.graph.node_indices()
    }

    /// 获取所有边
    pub fn edges(&self) -> Vec<Edge> {
        self.edge_to_index.keys().cloned().collect()
    }

    /// 获取节点的前驱节点
    pub fn predecessors(&self, node_index: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self.graph
            .neighbors_directed(node_index, petgraph::Direction::Incoming)
    }

    /// 获取节点的后继节点
    pub fn successors(&self, node_index: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self.graph
            .neighbors_directed(node_index, petgraph::Direction::Outgoing)
    }

    /// 获取节点的入边
    pub fn in_edges(&self, node_index: NodeIndex) -> Vec<Edge> {
        self.graph
            .edges_directed(node_index, petgraph::Direction::Incoming)
            .filter_map(|edge_ref| self.index_to_edge.get(&edge_ref.id()).cloned())
            .collect()
    }

    /// 获取节点的出边
    pub fn out_edges(&self, node_index: NodeIndex) -> Vec<Edge> {
        self.graph
            .edges_directed(node_index, petgraph::Direction::Outgoing)
            .filter_map(|edge_ref| self.index_to_edge.get(&edge_ref.id()).cloned())
            .collect()
    }

    /// 获取图配置
    pub fn config(&self) -> &GraphConfig {
        &self.config
    }

    /// 获取图配置（可变）
    pub fn config_mut(&mut self) -> &mut GraphConfig {
        &mut self.config
    }

    /// 查找边
    pub fn find_edge(&self, source: NodeIndex, target: NodeIndex) -> Option<EdgeIndex> {
        let edge = Edge::new(source, target);
        self.edge_to_index.get(&edge).copied()
    }

    /// 设置图配置
    pub fn set_config(&mut self, config: GraphConfig) {
        self.config = config;
    }

    /// 检查图是否为空
    pub fn is_empty(&self) -> bool {
        self.graph.node_count() == 0
    }

    /// 获取节点数量
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// 获取边数量
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// 检查节点是否存在
    pub fn has_node(&self, node_index: NodeIndex) -> bool {
        self.graph.node_weight(node_index).is_some()
    }

    /// 检查边是否存在
    pub fn has_edge(&self, edge: &Edge) -> bool {
        self.edge_to_index.contains_key(edge)
    }

    /// 移除节点
    pub fn remove_node(&mut self, node_index: NodeIndex) -> Option<NodeLabel> {
        self.graph.remove_node(node_index)
    }

    /// 移除边
    pub fn remove_edge(&mut self, edge: &Edge) -> Option<EdgeLabel> {
        if let Some(edge_index) = self.edge_to_index.swap_remove(edge) {
            self.index_to_edge.swap_remove(&edge_index);
            self.graph.remove_edge(edge_index)
        } else {
            None
        }
    }

    /// 获取子图（用于复合图支持）
    pub fn children(&self, parent_index: NodeIndex) -> Vec<NodeIndex> {
        // 查找所有以指定节点为父节点的子节点
        let mut children = Vec::new();

        for node_id in self.node_indices() {
            if let Some(node_label) = self.node_label(node_id) {
                if node_label.parent == Some(parent_index) {
                    children.push(node_id);
                }
            }
        }

        children
    }

    /// 设置父节点（用于复合图支持）
    pub fn set_parent(&mut self, child_index: NodeIndex, parent_index: NodeIndex) {
        if let Some(label) = self.node_label_mut(child_index) {
            label.parent = Some(parent_index);
        }
    }

    /// 获取父节点
    pub fn parent(&self, child_index: NodeIndex) -> Option<NodeIndex> {
        self.node_label(child_index)?.parent
    }

    /// 获取 petgraph 实例（用于高级操作）
    pub fn petgraph(&self) -> &PetGraph<NodeLabel, EdgeLabel, Directed> {
        &self.graph
    }

    /// 获取 petgraph 实例（可变，用于高级操作）
    pub fn petgraph_mut(&mut self) -> &mut PetGraph<NodeLabel, EdgeLabel, Directed> {
        &mut self.graph
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_graph() {
        let graph = Graph::new();
        assert!(graph.is_empty());
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph = Graph::new();
        let label = NodeLabel::default();

        let node_index = graph.add_node(label);
        assert!(!graph.is_empty());
        assert_eq!(graph.node_count(), 1);
        assert!(graph.has_node(node_index));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();
        let node1 = graph.add_node(NodeLabel::default());
        let node2 = graph.add_node(NodeLabel::default());

        let edge = Edge::new(node1, node2);
        let edge_label = EdgeLabel::default();

        let _edge_index = graph.add_edge(edge.clone(), edge_label);
        assert_eq!(graph.edge_count(), 1);
        assert!(graph.has_edge(&edge));
    }
}
