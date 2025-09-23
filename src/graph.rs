//! 图数据结构实现，基于 petgraph
use crate::types::*;
use indexmap::IndexMap;
use petgraph::graph::EdgeIndex;
use petgraph::visit::EdgeRef;
use petgraph::{Directed, Graph as PetGraph};
use std::panic::Location;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;

/// 调试跟踪上下文
#[derive(Clone, Debug)]
pub struct StackContext {
    /// 创建时间
    created_at: Instant,
    /// 调用位置信息
    location: &'static Location<'static>,
    /// 上下文 ID
    context_id: u32,
}

impl StackContext {
    /// 创建新的调试上下文，自动跟踪调用位置
    #[track_caller]
    pub fn new() -> Self {
        Self {
            created_at: Instant::now(),
            location: Location::caller(),
            context_id: get_next_graph_id(),
        }
    }

    /// 获取创建时间
    pub fn created_at(&self) -> Instant {
        self.created_at
    }

    /// 获取调用位置信息
    pub fn location(&self) -> &'static Location<'static> {
        self.location
    }

    /// 获取上下文 ID
    pub fn context_id(&self) -> u32 {
        self.context_id
    }

    /// 格式化位置信息为字符串
    pub fn format_location(&self) -> String {
        format!(
            "{}:{}:{}",
            self.location.file(),
            self.location.line(),
            self.location.column()
        )
    }
}

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
    /// 图的唯一 ID
    graph_id: u32,
    /// 调试跟踪上下文
    span: StackContext,
}

impl Graph {
    /// 创建新的空图，自动跟踪调用位置
    #[track_caller]
    pub fn new() -> Self {
        Self {
            graph: PetGraph::new(),
            edge_to_index: IndexMap::new(),
            index_to_edge: IndexMap::new(),
            config: GraphConfig::default(),
            dummy_chains: Vec::new(),
            graph_id: get_next_graph_id(),
            span: StackContext::new(),
        }
    }

    /// 创建带配置的图，自动跟踪调用位置
    #[track_caller]
    pub fn with_config(config: GraphConfig) -> Self {
        Self {
            graph: PetGraph::new(),
            edge_to_index: IndexMap::new(),
            index_to_edge: IndexMap::new(),
            config,
            dummy_chains: Vec::new(),
            graph_id: get_next_graph_id(),
            span: StackContext::new(),
        }
    }

    /// 获取图的 ID
    pub fn graph_id(&self) -> u32 {
        self.graph_id
    }

    /// 获取调试跟踪信息
    pub fn debug_info(&self) -> String {
        format!(
            "Graph ID: {}, Created at: {:?}, Location: {}",
            self.graph_id,
            self.span.created_at(),
            self.span.format_location()
        )
    }

    /// 添加节点
    pub fn add_node(&mut self, label: NodeLabel) -> NodeIndex {
        let node_index = self.graph.add_node(label);
        NodeIndex::new(node_index, self.graph_id)
    }

    /// 创建边（使用正确的图ID）
    pub fn create_edge(&self, source: NodeIndex, target: NodeIndex) -> Edge {
        // 验证source和target是否属于当前图
        if !source.belongs_to_graph(self.graph_id) {
            panic!(
                "Source node belongs to graph {}, but current graph is {}",
                source.which_graph, self.graph_id
            );
        }
        if !target.belongs_to_graph(self.graph_id) {
            panic!(
                "Target node belongs to graph {}, but current graph is {}",
                target.which_graph, self.graph_id
            );
        }
        
        let mut edge = Edge::new(source, target);
        edge.source.set_graph_id(self.graph_id);
        edge.target.set_graph_id(self.graph_id);
        edge
    }

    /// 创建用于查询的边（使用正确的图ID）
    pub fn create_query_edge(&self, source: NodeIndex, target: NodeIndex) -> Edge {
        self.create_edge(source, target)
    }

    /// 添加边
    pub fn add_edge(&mut self, mut edge: Edge, label: EdgeLabel) -> EdgeIndex {
        // 验证边的节点是否属于当前图
        if !edge.source.belongs_to_graph(self.graph_id) {
            panic!(
                "Source node belongs to graph {}, but current graph is {}",
                edge.source.which_graph, self.graph_id
            );
        }
        if !edge.target.belongs_to_graph(self.graph_id) {
            panic!(
                "Target node belongs to graph {}, but current graph is {}",
                edge.target.which_graph, self.graph_id
            );
        }

        // 确保Edge的图ID正确
        edge.source.set_graph_id(self.graph_id);
        edge.target.set_graph_id(self.graph_id);

        let edge_index = self
            .graph
            .add_edge(edge.source.node_index, edge.target.node_index, label);
        self.edge_to_index.insert(edge.clone(), edge_index);
        self.index_to_edge.insert(edge_index, edge);
        edge_index
    }

    /// 获取节点标签
    pub fn node_label(&self, node_index: NodeIndex) -> Option<&NodeLabel> {
        if !node_index.belongs_to_graph(self.graph_id) {
            panic!(
                "Node belongs to graph {}, but current graph is {}\nGraph debug info: {}\nNode debug info: belongs_to_graph({}) = {}",
                node_index.which_graph,
                self.graph_id,
                self.debug_info(),
                self.graph_id,
                node_index.belongs_to_graph(self.graph_id)
            );
        }
        self.graph.node_weight(node_index.node_index)
    }

    /// 获取节点标签（可变）
    pub fn node_label_mut(&mut self, node_index: NodeIndex) -> Option<&mut NodeLabel> {
        if !node_index.belongs_to_graph(self.graph_id) {
            panic!(
                "Node belongs to graph {}, but current graph is {}\nGraph debug info: {}\nNode debug info: belongs_to_graph({}) = {}",
                node_index.which_graph,
                self.graph_id,
                self.debug_info(),
                self.graph_id,
                node_index.belongs_to_graph(self.graph_id)
            );
        }
        self.graph.node_weight_mut(node_index.node_index)
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
        self.graph
            .node_indices()
            .map(move |node_index| NodeIndex::new(node_index, self.graph_id))
    }

    /// 获取所有边
    pub fn edges(&self) -> Vec<Edge> {
        self.edge_to_index.keys().cloned().collect()
    }

    /// 获取节点的前驱节点
    pub fn predecessors(&self, node_index: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        if !node_index.belongs_to_graph(self.graph_id) {
            panic!(
                "Node belongs to graph {}, but current graph is {}\nGraph debug info: {}\nNode debug info: belongs_to_graph({}) = {}",
                node_index.which_graph,
                self.graph_id,
                self.debug_info(),
                self.graph_id,
                node_index.belongs_to_graph(self.graph_id)
            );
        }
        self.graph
            .neighbors_directed(node_index.node_index, petgraph::Direction::Incoming)
            .map(move |idx| NodeIndex::new(idx, self.graph_id))
    }

    /// 获取节点的后继节点
    pub fn successors(&self, node_index: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        if !node_index.belongs_to_graph(self.graph_id) {
            panic!(
                "Node belongs to graph {}, but current graph is {}\nGraph debug info: {}\nNode debug info: belongs_to_graph({}) = {}",
                node_index.which_graph,
                self.graph_id,
                self.debug_info(),
                self.graph_id,
                node_index.belongs_to_graph(self.graph_id)
            );
        }
        self.graph
            .neighbors_directed(node_index.node_index, petgraph::Direction::Outgoing)
            .map(move |idx| NodeIndex::new(idx, self.graph_id))
    }

    /// 获取节点的入边
    pub fn in_edges(&self, node_index: NodeIndex) -> Vec<Edge> {
        if !node_index.belongs_to_graph(self.graph_id) {
            panic!(
                "Node belongs to graph {}, but current graph is {}\nGraph debug info: {}\nNode debug info: belongs_to_graph({}) = {}",
                node_index.which_graph,
                self.graph_id,
                self.debug_info(),
                self.graph_id,
                node_index.belongs_to_graph(self.graph_id)
            );
        }
        self.graph
            .edges_directed(node_index.node_index, petgraph::Direction::Incoming)
            .filter_map(|edge_ref| self.index_to_edge.get(&edge_ref.id()).cloned())
            .collect()
    }

    /// 获取节点的出边
    pub fn out_edges(&self, node_index: NodeIndex) -> Vec<Edge> {
        if !node_index.belongs_to_graph(self.graph_id) {
            panic!(
                "Node belongs to graph {}, but current graph is {}\nGraph debug info: {}\nNode debug info: belongs_to_graph({}) = {}",
                node_index.which_graph,
                self.graph_id,
                self.debug_info(),
                self.graph_id,
                node_index.belongs_to_graph(self.graph_id)
            );
        }
        self.graph
            .edges_directed(node_index.node_index, petgraph::Direction::Outgoing)
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
        if !source.belongs_to_graph(self.graph_id) {
            panic!(
                "Source node belongs to graph {}, but current graph is {}",
                source.which_graph, self.graph_id
            );
        }
        if !target.belongs_to_graph(self.graph_id) {
            panic!(
                "Target node belongs to graph {}, but current graph is {}",
                target.which_graph, self.graph_id
            );
        }
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
        if !node_index.belongs_to_graph(self.graph_id) {
            panic!(
                "Node belongs to graph {}, but current graph is {}\nGraph debug info: {}\nNode debug info: belongs_to_graph({}) = {}",
                node_index.which_graph,
                self.graph_id,
                self.debug_info(),
                self.graph_id,
                node_index.belongs_to_graph(self.graph_id)
            );
        }
        self.graph.node_weight(node_index.node_index).is_some()
    }

    /// 检查边是否存在
    pub fn has_edge(&self, edge: &Edge) -> bool {
        self.edge_to_index.contains_key(edge)
    }

    /// 移除节点
    pub fn remove_node(&mut self, node_index: NodeIndex) -> Option<NodeLabel> {
        if !node_index.belongs_to_graph(self.graph_id) {
            panic!(
                "Node belongs to graph {}, but current graph is {}\nGraph debug info: {}\nNode debug info: belongs_to_graph({}) = {}",
                node_index.which_graph,
                self.graph_id,
                self.debug_info(),
                self.graph_id,
                node_index.belongs_to_graph(self.graph_id)
            );
        }
        self.graph.remove_node(node_index.node_index)
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
        if !parent_index.belongs_to_graph(self.graph_id) {
            panic!(
                "Parent node belongs to graph {}, but current graph is {}",
                parent_index.which_graph, self.graph_id
            );
        }

        // 查找所有以指定节点为父节点的子节点
        let mut children = Vec::new();

        for node_id in self.node_indices() {
            if let Some(node_label) = self.node_label(node_id) {
                if let Some(parent) = node_label.parent {
                    // 需要将 NodeIndex 转换为 WrappedNodeIndex 进行比较
                    if parent == parent_index {
                        children.push(node_id);
                    }
                }
            }
        }

        children
    }

    /// 设置父节点（用于复合图支持）
    pub fn set_parent(&mut self, child_index: NodeIndex, parent_index: NodeIndex) {
        if !child_index.belongs_to_graph(self.graph_id) {
            panic!(
                "Child node belongs to graph {}, but current graph is {}",
                child_index.which_graph, self.graph_id
            );
        }
        if !parent_index.belongs_to_graph(self.graph_id) {
            panic!(
                "Parent node belongs to graph {}, but current graph is {}",
                parent_index.which_graph, self.graph_id
            );
        }

        if let Some(label) = self.node_label_mut(child_index) {
            label.parent = Some(parent_index);
        }
    }

    /// 获取父节点
    pub fn parent(&self, child_index: NodeIndex) -> Option<NodeIndex> {
        if !child_index.belongs_to_graph(self.graph_id) {
            panic!(
                "Child node belongs to graph {}, but current graph is {}",
                child_index.which_graph, self.graph_id
            );
        }
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

/// 全局图 ID 计数器
static GRAPH_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

/// 包装的节点索引，包含图 ID 用于验证
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeIndex {
    /// 内部的 NodeIndex
    pub node_index: petgraph::graph::NodeIndex,
    /// 所属图的 ID
    pub which_graph: u32,
}

impl NodeIndex {
    /// 创建新的 WrappedNodeIndex
    fn new(node_index: petgraph::prelude::NodeIndex, which_graph: u32) -> Self {
        Self {
            node_index,
            which_graph,
        }
    }
    /// 创建新的 WrappedNodeIndex
    pub fn new_raw(usize_index: usize, which_graph: u32) -> Self {
        Self {
            node_index: petgraph::prelude::NodeIndex::new(usize_index),
            which_graph,
        }
    }

    /// 验证是否属于指定的图
    pub fn belongs_to_graph(&self, graph_id: u32) -> bool {
        self.which_graph == graph_id
    }

    /// 从 NodeIndex 创建 WrappedNodeIndex（用于兼容性）
    /// 注意：这会创建一个无效的图 ID，仅用于临时兼容
    fn from_node_index(node_index: petgraph::prelude::NodeIndex) -> Self {
        Self {
            node_index,
            which_graph: 0, // 无效的图 ID，需要后续设置
        }
    }

    /// 获取内部的 NodeIndex（用于兼容性）
    fn as_node_index(&self) -> petgraph::prelude::NodeIndex {
        self.node_index
    }

    /// 设置图 ID（用于兼容性）
    pub fn set_graph_id(&mut self, graph_id: u32) {
        self.which_graph = graph_id;
    }

    /// 获取节点索引
    pub fn index(&self) -> usize {
        self.node_index.index()
    }
}

impl Default for NodeIndex {
    fn default() -> Self {
        Self {
            node_index: petgraph::prelude::NodeIndex::new(0),
            which_graph: 0,
        }
    }
}

/// 获取下一个图 ID
pub fn get_next_graph_id() -> u32 {
    GRAPH_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}
