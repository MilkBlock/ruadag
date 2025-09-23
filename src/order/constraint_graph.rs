//! 约束图模块 - 用于维护子图约束关系

use crate::graph::Graph;
use crate::graph::NodeIndex;
use indexmap::IndexMap;

/// 约束图 - 用于维护节点之间的约束关系
#[derive(Debug, Clone)]
pub struct ConstraintGraph {
    /// 约束边映射: from -> to
    constraints: IndexMap<NodeIndex, Vec<NodeIndex>>,
}

impl ConstraintGraph {
    /// 创建新的约束图
    pub fn new() -> Self {
        Self {
            constraints: IndexMap::new(),
        }
    }

    /// 添加约束边
    pub fn add_constraint(&mut self, from: NodeIndex, to: NodeIndex) {
        self.constraints
            .entry(from)
            .or_insert_with(Vec::new)
            .push(to);
    }

    /// 获取节点的约束目标
    pub fn get_constraints(&self, node: NodeIndex) -> Option<&Vec<NodeIndex>> {
        self.constraints.get(&node)
    }

    /// 检查是否存在约束
    pub fn has_constraint(&self, from: NodeIndex, to: NodeIndex) -> bool {
        self.constraints
            .get(&from)
            .map_or(false, |targets| targets.contains(&to))
    }

    /// 获取所有约束
    pub fn get_all_constraints(&self) -> &IndexMap<NodeIndex, Vec<NodeIndex>> {
        &self.constraints
    }

    /// 清空约束
    pub fn clear(&mut self) {
        self.constraints.clear();
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }
}

impl Default for ConstraintGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// 添加子图约束 - 对应JavaScript的addSubgraphConstraints函数
pub fn add_subgraph_constraints(
    layer_graph: &Graph,
    constraint_graph: &mut ConstraintGraph,
    sorted_nodes: &[NodeIndex],
) {
    let mut prev: IndexMap<NodeIndex, NodeIndex> = IndexMap::new();
    let mut root_prev: Option<NodeIndex> = None;

    for &node in sorted_nodes {
        let mut child = layer_graph.parent(node);
        let mut parent: Option<NodeIndex>;
        let mut prev_child: Option<NodeIndex>;

        while let Some(child_node) = child {
            parent = layer_graph.parent(child_node);

            if let Some(parent_node) = parent {
                prev_child = prev.get(&parent_node).copied();
                prev.insert(parent_node, child_node);
            } else {
                prev_child = root_prev;
                root_prev = Some(child_node);
            }

            if let Some(prev_child_node) = prev_child {
                if prev_child_node != child_node {
                    constraint_graph.add_constraint(prev_child_node, child_node);
                    break;
                }
            }

            child = parent;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_node_index(graph_id: u32, node_index: usize) -> NodeIndex {
        NodeIndex {
            node_index: petgraph::prelude::NodeIndex::new(node_index),
            which_graph: graph_id,
        }
    }

    #[test]
    fn test_constraint_graph_creation() {
        let cg = ConstraintGraph::new();
        assert!(cg.is_empty());
    }

    #[test]
    fn test_add_constraint() {
        let mut cg = ConstraintGraph::new();
        let node1 = create_test_node_index(1, 0);
        let node2 = create_test_node_index(1, 1);

        cg.add_constraint(node1, node2);
        assert!(cg.has_constraint(node1, node2));
        assert!(!cg.has_constraint(node2, node1));
    }

    #[test]
    fn test_get_constraints() {
        let mut cg = ConstraintGraph::new();
        let node1 = create_test_node_index(1, 0);
        let node2 = create_test_node_index(1, 1);
        let node3 = create_test_node_index(1, 2);

        cg.add_constraint(node1, node2);
        cg.add_constraint(node1, node3);

        let constraints = cg.get_constraints(node1).unwrap();
        assert_eq!(constraints.len(), 2);
        assert!(constraints.contains(&node2));
        assert!(constraints.contains(&node3));
    }
}
