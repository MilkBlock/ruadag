//! Brandes-Köpf 算法实现
//! 用于计算节点在层级中的位置，最小化边交叉
//!
//! 这是对原始 JavaScript 实现的忠实移植，包括：
//! - Type1 和 Type2 冲突检测
//! - 垂直对齐算法
//! - 水平压缩算法
//! - 四种对齐方向 (u+l, u+r, d+l, d+r)
//! - 平衡算法

use crate::graph::Graph;
use crate::graph::NodeIndex;
use crate::types::*;
use crate::util::is_placeholder;
use indexmap::IndexMap;
use indexmap::IndexSet;
use petgraph::Directed;
use petgraph::graph::Graph as PetGraph;
use petgraph::visit::EdgeRef;

/// 节点在层级中的位置信息
#[derive(Debug, Clone)]
pub struct NodePosition {
    /// 节点索引
    pub node: NodeIndex,
    /// 在层级中的位置
    pub position: f64,
    /// 层级
    pub rank: i32,
}

/// 边交叉信息
#[derive(Debug, Clone)]
pub struct EdgeCrossing {
    /// 边
    pub edge: Edge,
    /// 交叉数量
    pub crossings: usize,
}

/// Brandes-Köpf 算法结果
///
/// # Examples
///
/// ```
/// use dagviz::position::bk::BKResult;
/// use indexmap::IndexMap;
/// use crate::graph::NodeIndex;
///
/// let result = BKResult::new();
/// assert_eq!(result.positions.len(), 0);
/// assert_eq!(result.total_crossings, 0);
/// ```
#[derive(Debug, Clone)]
pub struct BKResult {
    /// 节点位置
    pub positions: IndexMap<NodeIndex, NodePosition>,
    /// 边交叉信息
    pub crossings: Vec<EdgeCrossing>,
    /// 总交叉数
    pub total_crossings: usize,
}

impl BKResult {
    /// 创建空结果
    pub fn new() -> Self {
        Self {
            positions: IndexMap::new(),
            crossings: Vec::new(),
            total_crossings: 0,
        }
    }
}

impl Default for BKResult {
    fn default() -> Self {
        Self::new()
    }
}

/// 冲突类型
#[derive(Debug, Clone, PartialEq)]
pub enum ConflictType {
    Type1,
    Type2,
}

/// 对齐信息
///
/// # Examples
///
/// ```
/// use dagviz::position::bk::Alignment;
/// use indexmap::IndexMap;
/// use crate::graph::NodeIndex;
///
/// let mut root = IndexMap::new();
/// let mut align = IndexMap::new();
/// let node1 = NodeIndex::new(0);
/// let node2 = NodeIndex::new(1);
///
/// root.insert(node1, node1);
/// root.insert(node2, node2);
/// align.insert(node1, node1);
/// align.insert(node2, node1);
///
/// let alignment = Alignment { root, align };
/// assert_eq!(alignment.root.len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct Alignment {
    pub root: IndexMap<NodeIndex, NodeIndex>,
    pub align: IndexMap<NodeIndex, NodeIndex>,
}

/// 四种对齐方向
#[derive(Debug, Clone)]
pub enum AlignmentDirection {
    UpLeft,    // u+l
    UpRight,   // u+r
    DownLeft,  // d+l
    DownRight, // d+r
}

/// Brandes-Köpf 算法实现
pub struct BrandesKoepf {
    /// 图
    graph: Graph,
    /// 层级信息
    ranks: IndexMap<NodeIndex, i32>,
    /// 层级到节点列表的映射
    layers: Vec<Vec<NodeIndex>>,
    /// 冲突信息
    conflicts: IndexMap<(NodeIndex, NodeIndex), ConflictType>,
}

impl BrandesKoepf {
    /// 创建新的 Brandes-Köpf 实例
    ///
    /// 注意：此构造函数假设图的所有节点都已经分配了有效的 rank 属性。
    /// 在调用此构造函数之前，应该先使用 rank 模块计算节点层级。
    ///
    /// # Examples
    ///
    /// ```
    /// use dagviz::position::bk::BrandesKoepf;
    /// use dagviz::graph::Graph;
    /// use dagviz::types::*;
    /// use crate::graph::NodeIndex;
    ///
    /// let mut graph = Graph::new();
    /// let mut node_a = NodeLabel::default();
    /// node_a.rank = Some(0);
    /// let a = graph.add_node(node_a);
    ///
    /// let mut node_b = NodeLabel::default();
    /// node_b.rank = Some(1);
    /// let b = graph.add_node(node_b);
    ///
    /// let bk = BrandesKoepf::new(graph);
    /// ```
    pub fn new(graph: Graph) -> Self {
        // 从图的节点中提取 rank 信息，跳过没有 rank 的虚拟节点
        let mut ranks = IndexMap::new();
        let mut nodes_without_rank = Vec::new();

        for node_idx in graph.node_indices() {
            if let Some(node) = graph.node_label(node_idx) {
                if let Some(rank) = node.rank {
                    ranks.insert(node_idx, rank);
                } else {
                    nodes_without_rank.push(node_idx);
                }
            }
        }

        // 如果有节点没有 rank，打印警告但不 panic
        if !nodes_without_rank.is_empty() {
            println!(
                "警告: 以下节点没有分配 rank，将被跳过: {:?}",
                nodes_without_rank
            );
        }

        Self {
            graph,
            ranks,
            layers: Vec::new(),
            conflicts: IndexMap::new(),
        }
    }

    /// 运行 Brandes-Köpf 算法
    ///
    /// # Examples
    ///
    /// ```
    /// use dagviz::position::bk::BrandesKoepf;
    /// use dagviz::graph::Graph;
    /// use dagviz::types::{NodeLabel, Edge, EdgeLabel};
    /// use crate::graph::NodeIndex;
    ///
    /// let mut graph = Graph::new();
    ///
    /// // 创建简单图
    /// let mut node_a = NodeLabel::default();
    /// node_a.label = Some("A".to_string());
    /// let a = graph.add_node(node_a);
    ///
    /// let mut node_b = NodeLabel::default();
    /// node_b.label = Some("B".to_string());
    /// let b = graph.add_node(node_b);
    ///
    /// graph.add_edge(Edge::new(a, b), EdgeLabel::default());
    ///
    /// let mut bk = BrandesKoepf::new(graph);
    /// let result = bk.run();
    ///
    /// assert_eq!(result.positions.len(), 2);
    /// ```
    pub fn run(&mut self) -> BKResult {
        let mut result = BKResult::new();

        println!("=== Brandes-Köpf 算法开始 ===");
        println!("输入图信息:");
        println!("  节点数: {}", self.graph.node_count());
        println!("  边数: {}", self.graph.edge_count());

        // 构建层级结构（层级已在构造函数中从图的节点中提取）
        println!("\n1. 构建层级结构...");
        self.build_layers();
        println!("  层级结构:");
        for (i, layer) in self.layers.iter().enumerate() {
            println!("    层 {}: {:?}", i, layer);
        }

        // 检测冲突
        println!("\n2. 检测冲突...");
        self.find_conflicts();
        println!("  冲突检测完成");

        // 计算四种对齐方向的位置
        println!("\n3. 计算四种对齐方向的位置...");
        let mut xss = IndexMap::new();
        for vert in ["u", "d"] {
            for horiz in ["l", "r"] {
                let direction = format!("{}{}", vert, horiz);
                println!("\n  处理方向: {}", direction);

                let _direction = match (vert, horiz) {
                    ("u", "l") => AlignmentDirection::UpLeft,
                    ("u", "r") => AlignmentDirection::UpRight,
                    ("d", "l") => AlignmentDirection::DownLeft,
                    ("d", "r") => AlignmentDirection::DownRight,
                    _ => unreachable!(),
                };

                let adjusted_layering = if vert == "u" {
                    self.layers.clone()
                } else {
                    self.layers.iter().rev().cloned().collect()
                };

                let adjusted_layering = if horiz == "r" {
                    adjusted_layering
                        .into_iter()
                        .map(|layer| layer.into_iter().rev().collect())
                        .collect()
                } else {
                    adjusted_layering
                };

                println!("    调整后的层级:");
                for (i, layer) in adjusted_layering.iter().enumerate() {
                    println!("      层 {}: {:?}", i, layer);
                }

                let neighbor_fn = if vert == "u" {
                    |g: &Graph, v: NodeIndex| g.predecessors(v).collect::<Vec<_>>()
                } else {
                    |g: &Graph, v: NodeIndex| g.successors(v).collect::<Vec<_>>()
                };

                println!("    执行垂直对齐...");
                let align = self.vertical_alignment(&adjusted_layering, &neighbor_fn);
                println!("      对齐结果: {:?}", align);

                println!("    执行水平压缩...");
                let xs = self.horizontal_compaction(&adjusted_layering, &align, horiz == "r");
                println!("      压缩结果: {:?}", xs);

                xss.insert(direction, xs);
            }
        }

        // 找到最小宽度对齐
        println!("\n5. 找到最小宽度对齐...");
        let smallest_width = self.find_smallest_width_alignment(&xss);
        println!("  最小宽度对齐: {:?}", smallest_width);

        // 如果所有对齐都包含无效值，则只返回ul对齐的结果
        let final_xs = if let Some(alignment) = smallest_width {
            // 对齐坐标
            println!("\n6. 对齐坐标...");
            self.align_coordinates(&mut xss, &alignment);
            println!("  坐标对齐完成");

            // 平衡坐标
            println!("\n7. 平衡坐标...");
            let balanced = self.balance(&xss, None);
            println!("  最终坐标: {:?}", balanced);
            balanced
        } else {
            // 如果所有对齐都包含无效值，只返回ul对齐的结果
            println!("\n6. 所有对齐都包含无效值，返回ul对齐结果...");
            xss.get("ul").cloned().unwrap_or_default()
        };

        // 设置最终位置
        println!("\n8. 设置最终位置...");
        for (node, &x) in &final_xs {
            if let Some(&rank) = self.ranks.get(node) {
                let node_label = self.graph.node_label(*node).unwrap();
                let label = node_label.label.as_deref().unwrap_or("Unknown");
                println!("  {}: x = {:.6}, rank = {}", label, x, rank);

                result.positions.insert(
                    *node,
                    NodePosition {
                        node: *node,
                        position: x,
                        rank,
                    },
                );
            }
        }

        println!("\n=== Brandes-Köpf 算法完成 ===");
        result
    }

    /// 构建层级结构
    ///
    /// 根据已计算的节点层级，将节点按层级分组到不同的层中。
    /// 每层包含具有相同层级的节点。
    ///
    /// # Examples
    ///
    /// ```
    /// use dagviz::position::bk::BrandesKoepf;
    /// use dagviz::graph::Graph;
    /// use dagviz::types::*;
    ///
    /// let mut graph = Graph::new();
    /// let mut node_a = NodeLabel::default();
    /// let mut node_b = NodeLabel::default();
    /// let mut node_c = NodeLabel::default();
    ///
    /// let a = graph.add_node(node_a);
    /// let b = graph.add_node(node_b);
    /// let c = graph.add_node(node_c);
    ///
    /// // 创建 A -> B -> C 的链式结构
    /// graph.add_edge(Edge::new(a, b), EdgeLabel::default());
    /// graph.add_edge(Edge::new(b, c), EdgeLabel::default());
    ///
    /// let mut bk = BrandesKoepf::new(graph);
    /// bk.compute_ranks();
    /// bk.build_layers();
    ///
    /// // 验证层级结构
    /// assert_eq!(bk.layers.len(), 3);
    /// assert!(bk.layers[0].contains(&a));
    /// assert!(bk.layers[1].contains(&b));
    /// assert!(bk.layers[2].contains(&c));
    /// ```
    pub fn build_layers(&mut self) {
        let min_rank = self.ranks.values().min().copied().unwrap_or(0);
        let max_rank = self.ranks.values().max().copied().unwrap_or(0);
        let rank_count = (max_rank - min_rank + 1) as usize;
        self.layers = vec![Vec::new(); rank_count];

        for (node, &rank) in &self.ranks {
            // 跳过占位符节点
            if !is_placeholder(*node) {
                let adjusted_rank = (rank - min_rank) as usize;
                if adjusted_rank < self.layers.len() {
                    self.layers[adjusted_rank].push(*node);
                }
            }
        }

        // 对每层内的节点按order属性排序，与JavaScript的buildLayerMatrix一致
        for layer in &mut self.layers {
            layer.sort_by_key(|&node| {
                self.graph
                    .node_label(node)
                    .and_then(|label| label.order)
                    .unwrap_or(0)
            });
        }
    }

    /// 检测 Type1 和 Type2 冲突
    ///
    /// 检测图中可能影响布局的边交叉冲突。
    /// Type1 冲突：非内部段与内部段交叉
    /// Type2 冲突：内部段之间的交叉
    ///
    /// # Examples
    ///
    /// ```
    /// use dagviz::position::bk::BrandesKoepf;
    /// use dagviz::graph::Graph;
    /// use dagviz::types::*;
    ///
    /// let mut graph = Graph::new();
    /// let mut node_a = NodeLabel::default();
    /// let mut node_b = NodeLabel::default();
    /// let mut node_c = NodeLabel::default();
    /// let mut node_d = NodeLabel::default();
    ///
    /// let a = graph.add_node(node_a);
    /// let b = graph.add_node(node_b);
    /// let c = graph.add_node(node_c);
    /// let d = graph.add_node(node_d);
    ///
    /// // 创建可能产生冲突的图结构
    /// graph.add_edge(Edge::new(a, c), EdgeLabel::default());
    /// graph.add_edge(Edge::new(b, d), EdgeLabel::default());
    ///
    /// let mut bk = BrandesKoepf::new(graph);
    /// bk.compute_ranks();
    /// bk.build_layers();
    /// bk.find_conflicts();
    ///
    /// // 验证冲突检测完成（具体冲突数量取决于图结构）
    /// // 这里主要验证函数能正常执行
    /// ```
    pub fn find_conflicts(&mut self) {
        self.find_type1_conflicts();
        self.find_type2_conflicts();
    }

    /// Marks all edges in the graph with a type-1 conflict with the "type1Conflict"
    /// property. A type-1 conflict is one where a non-inner segment crosses an
    /// inner segment. An inner segment is an edge with both incident nodes marked
    /// with the "dummy" property.
    ///
    /// This algorithm scans layer by layer, starting with the second, for type-1
    /// conflicts between the current layer and the previous layer. For each layer
    /// it scans the nodes from left to right until it reaches one that is incident
    /// on an inner segment. It then scans predecessors to determine if they have
    /// edges that cross that inner segment. At the end a final scan is done for all
    /// nodes on the current rank to see if they cross the last visited inner
    /// segment.
    ///
    /// This algorithm (safely) assumes that a dummy node will only be incident on a
    /// single node in the layers being scanned.
    pub fn find_type1_conflicts(&mut self) {
        for i in 1..self.layers.len() {
            let prev_layer = self.layers[i - 1].clone();
            let layer = self.layers[i].clone();
            self.visit_layer_type1(&prev_layer, &layer);
        }
    }

    /// 检测 Type2 冲突
    pub fn find_type2_conflicts(&mut self) {
        for i in 1..self.layers.len() {
            let prev_layer = self.layers[i - 1].clone();
            let layer = self.layers[i].clone();
            self.visit_layer_type2(&prev_layer, &layer);
        }
    }

    /// 访问层级检测 Type1 冲突
    pub fn visit_layer_type1(&mut self, prev_layer: &[NodeIndex], layer: &[NodeIndex]) {
        let mut k0 = 0;
        let mut scan_pos = 0;
        let prev_layer_length = prev_layer.len();
        let last_node = layer.last().copied();

        for (i, &v) in layer.iter().enumerate() {
            // 跳过占位符节点
            if is_placeholder(v) {
                continue;
            }

            let w = self.find_other_inner_segment_node(v);
            let k1 = if let Some(w) = w {
                self.get_node_order(w).unwrap_or(prev_layer_length)
            } else {
                prev_layer_length
            };

            if w.is_some() || Some(v) == last_node {
                for &scan_node in &layer[scan_pos..=i] {
                    // 跳过占位符节点
                    if is_placeholder(scan_node) {
                        continue;
                    }

                    let predecessors: Vec<NodeIndex> = self.graph.predecessors(scan_node).collect();
                    for u in predecessors {
                        // 跳过占位符节点
                        if is_placeholder(u) {
                            continue;
                        }

                        let u_pos = self.get_node_order(u).unwrap_or(0);
                        let u_dummy = self.is_dummy_node(u);
                        let scan_dummy = self.is_dummy_node(scan_node);

                        if (u_pos < k0 || k1 < u_pos) && !(u_dummy && scan_dummy) {
                            self.add_conflict(u, scan_node, ConflictType::Type1);
                        }
                    }
                }
                scan_pos = i + 1;
                k0 = k1;
            }
        }
    }

    /// 访问层级检测 Type2 冲突
    pub fn visit_layer_type2(&mut self, north: &[NodeIndex], south: &[NodeIndex]) {
        let mut prev_north_pos = -1;
        let mut next_north_pos = 0;
        let mut south_pos = 0;

        for (south_lookahead, &v) in south.iter().enumerate() {
            // 跳过占位符节点
            if is_placeholder(v) {
                continue;
            }

            if self.is_border_node(v) {
                let predecessors = self.graph.predecessors(v).collect::<Vec<_>>();
                if !predecessors.is_empty() {
                    next_north_pos = self.get_node_order(predecessors[0]).unwrap_or(0) as i32;
                    self.scan_type2(
                        south,
                        south_pos,
                        south_lookahead,
                        prev_north_pos,
                        next_north_pos,
                    );
                    south_pos = south_lookahead;
                    prev_north_pos = next_north_pos;
                }
            }
            self.scan_type2(
                south,
                south_pos,
                south.len(),
                next_north_pos,
                north.len() as i32,
            );
        }
    }

    /// 扫描 Type2 冲突
    pub fn scan_type2(
        &mut self,
        south: &[NodeIndex],
        south_pos: usize,
        south_end: usize,
        prev_north_border: i32,
        next_north_border: i32,
    ) {
        for i in south_pos..south_end {
            let v = south[i];
            // 跳过占位符节点
            if is_placeholder(v) {
                continue;
            }

            if self.is_dummy_node(v) {
                let predecessors: Vec<NodeIndex> = self.graph.predecessors(v).collect();
                for u in predecessors {
                    // 跳过占位符节点
                    if is_placeholder(u) {
                        continue;
                    }

                    if self.is_dummy_node(u) {
                        let u_order = self.get_node_order(u).unwrap_or(0) as i32;
                        if u_order < prev_north_border || u_order > next_north_border {
                            self.add_conflict(u, v, ConflictType::Type2);
                        }
                    }
                }
            }
        }
    }

    /// 查找其他内部段节点
    pub fn find_other_inner_segment_node(&self, v: NodeIndex) -> Option<NodeIndex> {
        // 跳过占位符节点
        if is_placeholder(v) {
            return None;
        }

        if self.is_dummy_node(v) {
            self.graph
                .predecessors(v)
                .find(|&u| !is_placeholder(u) && self.is_dummy_node(u))
        } else {
            None
        }
    }

    /// 添加冲突
    pub fn add_conflict(&mut self, v: NodeIndex, w: NodeIndex, conflict_type: ConflictType) {
        // 跳过占位符节点
        if is_placeholder(v) || is_placeholder(w) {
            return;
        }

        let (v, w) = if v.index() > w.index() {
            (w, v)
        } else {
            (v, w)
        };
        self.conflicts.insert((v, w), conflict_type);
    }

    /// 检查是否有冲突
    pub fn has_conflict(&self, v: NodeIndex, w: NodeIndex) -> bool {
        // 占位符节点不会有冲突
        if is_placeholder(v) || is_placeholder(w) {
            return false;
        }

        let (v, w) = if v.index() > w.index() {
            (w, v)
        } else {
            (v, w)
        };
        self.conflicts.contains_key(&(v, w))
    }

    /// 获取节点顺序
    pub fn get_node_order(&self, node: NodeIndex) -> Option<usize> {
        // 从节点的标签中获取order字段
        self.graph.node_label(node).and_then(|label| label.order)
    }

    /// 检查是否为虚拟节点
    pub fn is_dummy_node(&self, node: NodeIndex) -> bool {
        // 占位符节点不是虚拟节点
        if is_placeholder(node) {
            return false;
        }

        if let Some(node_label) = self.graph.node_label(node) {
            node_label.dummy.is_some()
        } else {
            false
        }
    }

    /// 检查是否为边界节点
    pub fn is_border_node(&self, node: NodeIndex) -> bool {
        // 占位符节点不是边界节点
        if is_placeholder(node) {
            return false;
        }

        if let Some(node_label) = self.graph.node_label(node) {
            matches!(node_label.dummy, Some(Dummy::Border))
        } else {
            false
        }
    }

    /// 获取图的引用
    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    /// 垂直对齐
    ///
    /// 根据给定的层级结构和邻居函数，计算节点的垂直对齐关系。
    /// 这是 Brandes-Köpf 算法的核心步骤之一。
    ///
    /// # Arguments
    ///
    /// * `layering` - 层级结构，每层包含该层的节点
    /// * `neighbor_fn` - 邻居函数，用于获取节点的前驱或后继节点
    ///
    /// # Examples
    ///
    /// ```
    /// use dagviz::position::bk::BrandesKoepf;
    /// use dagviz::graph::Graph;
    /// use dagviz::types::*;
    /// use crate::graph::NodeIndex;
    ///
    /// let mut graph = Graph::new();
    /// let mut node_a = NodeLabel::default();
    /// let mut node_b = NodeLabel::default();
    /// let mut node_c = NodeLabel::default();
    ///
    /// let a = graph.add_node(node_a);
    /// let b = graph.add_node(node_b);
    /// let c = graph.add_node(node_c);
    ///
    /// graph.add_edge(Edge::new(a, b), EdgeLabel::default());
    /// graph.add_edge(Edge::new(b, c), EdgeLabel::default());
    ///
    /// let bk = BrandesKoepf::new(graph);
    /// let layering = vec![vec![a], vec![b], vec![c]];
    /// let neighbor_fn = |g: &Graph, v: NodeIndex| g.predecessors(v).collect::<Vec<_>>();
    ///
    /// let alignment = bk.vertical_alignment(&layering, neighbor_fn);
    ///
    /// // 验证对齐结果
    /// assert_eq!(alignment.root.len(), 3);
    /// assert_eq!(alignment.align.len(), 3);
    /// ```
    pub fn vertical_alignment<F>(&self, layering: &[Vec<NodeIndex>], neighbor_fn: F) -> Alignment
    where
        F: Fn(&Graph, NodeIndex) -> Vec<NodeIndex>,
    {
        let mut root = IndexMap::new();
        let mut align = IndexMap::new();
        let mut pos = IndexMap::new();

        // 初始化
        for layer in layering {
            for (order, &v) in layer.iter().enumerate() {
                // 跳过占位符节点
                if is_placeholder(v) {
                    continue;
                }

                root.insert(v, v);
                align.insert(v, v);
                pos.insert(v, order);
            }
        }

        // 对齐
        for layer in layering {
            let mut prev_idx = -1;
            for &v in layer {
                // 跳过占位符节点
                if is_placeholder(v) {
                    continue;
                }

                let mut ws = neighbor_fn(&self.graph, v);
                // 过滤掉不在层级矩阵中的邻居节点和占位符节点
                ws.retain(|&w| pos.contains_key(&w) && !is_placeholder(w));
                if !ws.is_empty() {
                    ws.sort_by_key(|&w| pos.get(&w).copied().unwrap_or(0));
                    let mp = (ws.len() - 1) as f64 / 2.0;
                    let start = mp.floor() as usize;
                    let end = mp.ceil() as usize;

                    for i in start..=end {
                        if i < ws.len() {
                            let w = ws[i];
                            if align.get(&v) == Some(&v)
                                && prev_idx < pos.get(&w).copied().unwrap_or(0) as i32
                                && !self.has_conflict(v, w)
                            {
                                let root_w = root.get(&w).copied().unwrap_or(w);
                                align.insert(w, v);
                                align.insert(v, root_w);
                                root.insert(v, root_w);
                                prev_idx = pos.get(&w).copied().unwrap_or(0) as i32;
                            }
                        }
                    }
                }
            }
        }

        Alignment { root, align }
    }

    /// 水平压缩
    ///
    /// 根据垂直对齐结果，计算节点的水平位置。
    /// 通过构建块图并应用拓扑排序来确定最终的X坐标。
    ///
    /// # Arguments
    ///
    /// * `layering` - 层级结构
    /// * `align` - 垂直对齐结果
    /// * `reverse_sep` - 是否反向分离
    ///
    /// # Examples
    ///
    /// ```
    /// use dagviz::position::bk::{BrandesKoepf, Alignment};
    /// use dagviz::graph::Graph;
    /// use dagviz::types::*;
    /// use indexmap::IndexMap;
    /// use crate::graph::NodeIndex;
    ///
    /// let mut graph = Graph::new();
    /// let mut node_a = NodeLabel::default();
    /// let mut node_b = NodeLabel::default();
    ///
    /// let a = graph.add_node(node_a);
    /// let b = graph.add_node(node_b);
    ///
    /// graph.add_edge(Edge::new(a, b), EdgeLabel::default());
    ///
    /// let bk = BrandesKoepf::new(graph);
    /// let layering = vec![vec![a], vec![b]];
    /// let mut root = IndexMap::new();
    /// let mut align_map = IndexMap::new();
    /// root.insert(a, a);
    /// root.insert(b, b);
    /// align_map.insert(a, a);
    /// align_map.insert(b, b);
    /// let alignment = Alignment { root: root, align: align_map };
    ///
    /// let positions = bk.horizontal_compaction(&layering, &alignment, false);
    ///
    /// // 验证位置计算
    /// assert_eq!(positions.len(), 2);
    /// assert!(positions.contains_key(&a));
    /// assert!(positions.contains_key(&b));
    /// ```
    pub fn horizontal_compaction(
        &self,
        layering: &[Vec<NodeIndex>],
        align: &Alignment,
        reverse_sep: bool,
    ) -> IndexMap<NodeIndex, f64> {
        println!("      开始水平压缩...");
        println!("        输入层级: {:?}", layering);
        println!("        对齐根: {:?}", align.root);
        println!("        反向分离: {}", reverse_sep);

        let mut xs = IndexMap::new();
        let (block_graph, node_map) =
            self.build_block_graph_with_mapping(layering, &align.root, reverse_sep);

        println!("        块图节点数: {}", block_graph.node_count());
        println!("        块图边数: {}", block_graph.edge_count());
        println!("        节点映射: {:?}", node_map);

        // 第一遍：分配最小坐标
        println!("        第一遍：分配最小坐标...");
        self.iterate(
            &block_graph,
            |elem| {
                xs.insert(elem, 0.0);
                for edge in block_graph.edges_directed(elem, petgraph::Direction::Incoming) {
                    let source = edge.source();
                    if let Some(&source_x) = xs.get(&source) {
                        let edge_weight = edge.weight();
                        let current_x: f64 = xs.get(&elem).copied().unwrap_or(0.0);
                        let new_x = current_x.max(source_x + edge_weight);
                        xs.insert(elem, new_x);
                        println!(
                            "          节点 {}: 从 {} 更新到 {} (源: {}, 边权重: {})",
                            elem.index(),
                            current_x,
                            new_x,
                            source_x,
                            edge_weight
                        );
                    }
                }
            },
            |elem| {
                block_graph
                    .neighbors_directed(elem, petgraph::Direction::Incoming)
                    .collect()
            },
        );

        println!("        第一遍完成，坐标: {:?}", xs);

        // 第二遍：分配最大坐标
        println!("        第二遍：分配最大坐标...");
        self.iterate(
            &block_graph,
            |elem| {
                let mut min = f64::INFINITY;
                for edge in block_graph.edges_directed(elem, petgraph::Direction::Outgoing) {
                    let target = edge.target();
                    if let Some(&target_x) = xs.get(&target) {
                        let edge_weight = edge.weight();
                        min = min.min(target_x - edge_weight);
                    }
                }

                if min != f64::INFINITY {
                    let current_x = xs.get(&elem).copied().unwrap_or(0.0);
                    let new_x = current_x.max(min);
                    xs.insert(elem, new_x);
                    println!(
                        "          节点 {}: 从 {} 更新到 {} (最小: {})",
                        elem.index(),
                        current_x,
                        new_x,
                        min
                    );
                }
            },
            |elem| {
                block_graph
                    .neighbors_directed(elem, petgraph::Direction::Outgoing)
                    .collect()
            },
        );

        println!("        第二遍完成，坐标: {:?}", xs);

        // 为所有节点分配 x 坐标
        println!("        分配最终坐标...");
        let mut final_xs = IndexMap::new();
        for (v, &root_v) in &align.root {
            if let Some(&block_node_id) = node_map.get(&root_v) {
                if let Some(&x) = xs.get(&block_node_id) {
                    final_xs.insert(*v, x);
                    let node_label = self.graph.node_label(*v).unwrap();
                    let label = node_label.label.as_deref().unwrap_or("Unknown");
                    println!("          节点 {}: x = {:.6}", label, x);
                }
            }
        }

        println!("        水平压缩完成，最终坐标: {:?}", final_xs);
        final_xs
    }

    /// 构建块图并返回节点映射
    pub fn build_block_graph_with_mapping(
        &self,
        layering: &[Vec<NodeIndex>],
        root: &IndexMap<NodeIndex, NodeIndex>,
        reverse_sep: bool,
    ) -> (
        PetGraph<f64, f64, Directed>,
        IndexMap<NodeIndex, petgraph::graph::NodeIndex>,
    ) {
        println!("        构建块图...");
        println!("          输入层级: {:?}", layering);
        println!("          对齐根: {:?}", root);
        println!("          反向分离: {}", reverse_sep);

        let mut block_graph = PetGraph::<f64, f64, Directed>::new();
        let node_sep = 50.0; // 与JavaScript demo匹配的节点间距
        let edge_sep = 1.0; // 默认边间距

        // 首先添加所有节点
        println!("          添加节点...");
        let mut node_map = IndexMap::new();
        for layer in layering {
            for &v in layer {
                // 跳过占位符节点
                if is_placeholder(v) {
                    continue;
                }

                let v_root = root.get(&v).copied().unwrap_or(v);
                if !node_map.contains_key(&v_root) {
                    let node_id = block_graph.add_node(v_root.index() as f64);
                    node_map.insert(v_root, node_id);
                    let node_label = self.graph.node_label(v).unwrap();
                    let label = node_label.label.as_deref().unwrap_or("Unknown");
                    println!("            添加节点 {} (根: {})", label, v_root.index());
                }
            }
        }

        // 然后添加边
        println!("          添加边...");
        for (_layer_idx, layer) in layering.iter().enumerate() {
            let mut u = None;
            for &v in layer {
                // 跳过占位符节点
                if is_placeholder(v) {
                    continue;
                }

                let v_root = root.get(&v).copied().unwrap_or(v);
                let v_node_id = node_map[&v_root];

                if let Some(u_node) = u {
                    let u_root = root.get(&u_node).copied().unwrap_or(u_node);
                    let u_node_id = node_map[&u_root];

                    // 只有当根节点不同时才添加边
                    if u_root != v_root {
                        let sep_value = self.sep(node_sep, edge_sep, reverse_sep, u_node, v);
                        let prev_max = block_graph
                            .edges_connecting(u_node_id, v_node_id)
                            .map(|e| *e.weight())
                            .fold(0.0, f64::max);
                        let final_weight = sep_value.max(prev_max);
                        block_graph.add_edge(u_node_id, v_node_id, final_weight);

                        let u_label = self
                            .graph
                            .node_label(u_node)
                            .unwrap()
                            .label
                            .as_deref()
                            .unwrap_or("Unknown");
                        let v_label = self
                            .graph
                            .node_label(v)
                            .unwrap()
                            .label
                            .as_deref()
                            .unwrap_or("Unknown");
                        println!(
                            "            添加边 {} -> {} (权重: {:.6})",
                            u_label, v_label, final_weight
                        );
                    } else {
                        let u_label = self
                            .graph
                            .node_label(u_node)
                            .unwrap()
                            .label
                            .as_deref()
                            .unwrap_or("Unknown");
                        let v_label = self
                            .graph
                            .node_label(v)
                            .unwrap()
                            .label
                            .as_deref()
                            .unwrap_or("Unknown");
                        println!("            跳过边 {} -> {} (相同根节点)", u_label, v_label);
                    }
                }
                u = Some(v);
            }
        }

        println!(
            "          块图构建完成: {} 节点, {} 边",
            block_graph.node_count(),
            block_graph.edge_count()
        );
        (block_graph, node_map)
    }

    /// 计算分离值
    pub fn sep(
        &self,
        node_sep: f64,
        edge_sep: f64,
        reverse_sep: bool,
        v: NodeIndex,
        w: NodeIndex,
    ) -> f64 {
        // 如果任一节点是占位符，返回0
        if is_placeholder(v) || is_placeholder(w) {
            return 0.0;
        }

        let mut sum = 0.0;
        let mut delta = 0.0;

        // 获取节点标签
        let v_label = self.graph.node_label(v).unwrap();
        let w_label = self.graph.node_label(w).unwrap();

        // 使用实际的节点宽度
        let v_width = v_label.width;
        let w_width = w_label.width;

        println!(
            "            计算分离值: {} -> {}",
            v_label.label.as_deref().unwrap_or("Unknown"),
            w_label.label.as_deref().unwrap_or("Unknown")
        );
        println!("              v_width: {}, w_width: {}", v_width, w_width);
        println!(
            "              node_sep: {}, edge_sep: {}, reverse_sep: {}",
            node_sep, edge_sep, reverse_sep
        );

        sum += v_width / 2.0;

        // 处理 v 节点的 labelpos (NodeLabel 没有 labelpos 字段，跳过)
        // if let Some(labelpos) = &v_label.labelpos {
        //     match labelpos {
        //         LabelPosition::Left => delta = -v_width / 2.0,
        //         LabelPosition::Right => delta = v_width / 2.0,
        //         LabelPosition::Center => delta = 0.0,
        //         LabelPosition::Top | LabelPosition::Bottom => delta = 0.0,
        //     }
        // }
        if delta != 0.0 {
            sum += if reverse_sep { delta } else { -delta };
        }
        delta = 0.0;

        // 根据节点类型使用不同的间距
        let v_sep = if v_label.dummy.is_some() {
            edge_sep
        } else {
            node_sep
        };
        let w_sep = if w_label.dummy.is_some() {
            edge_sep
        } else {
            node_sep
        };

        sum += v_sep / 2.0;
        sum += w_sep / 2.0;
        sum += w_width / 2.0;

        // 处理 w 节点的 labelpos (NodeLabel 没有 labelpos 字段，跳过)
        // if let Some(labelpos) = &w_label.labelpos {
        //     match labelpos {
        //         LabelPosition::Left => delta = w_width / 2.0,
        //         LabelPosition::Right => delta = -w_width / 2.0,
        //         LabelPosition::Center => delta = 0.0,
        //         LabelPosition::Top | LabelPosition::Bottom => delta = 0.0,
        //     }
        // }
        if delta != 0.0 {
            sum += if reverse_sep { delta } else { -delta };
        }

        println!("              分离值: {:.6}", sum);
        sum
    }

    /// 迭代函数
    pub fn iterate<F, G>(
        &self,
        graph: &PetGraph<f64, f64, Directed>,
        mut set_xs_func: F,
        next_nodes_func: G,
    ) where
        F: FnMut(petgraph::graph::NodeIndex),
        G: Fn(petgraph::graph::NodeIndex) -> Vec<petgraph::graph::NodeIndex>,
    {
        let mut stack = graph.node_indices().collect::<Vec<_>>();
        let mut visited = IndexSet::new();

        while let Some(elem) = stack.pop() {
            if visited.contains(&elem) {
                set_xs_func(elem);
            } else {
                visited.insert(elem);
                stack.push(elem);
                stack.extend(next_nodes_func(elem));
            }
        }
    }

    /// 找到最小宽度对齐
    ///
    /// 在四种对齐方向中选择产生最小宽度的对齐方式。
    /// 宽度定义为所有节点位置的最大值减去最小值。
    ///
    /// # Arguments
    ///
    /// * `xss` - 包含四种对齐方向位置结果的映射
    ///
    /// # Examples
    ///
    /// ```
    /// use dagviz::position::bk::BrandesKoepf;
    /// use dagviz::graph::Graph;
    /// use indexmap::IndexMap;
    /// use crate::graph::NodeIndex;
    ///
    /// let graph = Graph::new();
    /// let bk = BrandesKoepf::new(graph);
    ///
    /// let mut xss = IndexMap::new();
    /// let mut ul = IndexMap::new();
    /// let mut ur = IndexMap::new();
    /// let n1 = NodeIndex::new(0);
    /// let n2 = NodeIndex::new(1);
    ///
    /// // ul 对齐：宽度较小
    /// ul.insert(n1, 10.0);
    /// ul.insert(n2, 20.0);
    ///
    /// // ur 对齐：宽度较大
    /// ur.insert(n1, 5.0);
    /// ur.insert(n2, 30.0);
    ///
    /// xss.insert("ul".to_string(), ul);
    /// xss.insert("ur".to_string(), ur);
    ///
    /// let best_alignment = bk.find_smallest_width_alignment(&xss);
    ///
    /// // 验证返回了最小宽度的坐标对象
    /// assert!(best_alignment.is_some());
    /// ```
    pub fn find_smallest_width_alignment(
        &self,
        xss: &IndexMap<String, IndexMap<NodeIndex, f64>>,
    ) -> Option<IndexMap<NodeIndex, f64>> {
        let mut min_width = f64::INFINITY;
        let mut best_alignment = None;

        for (_, xs) in xss {
            let mut max = f64::NEG_INFINITY;
            let mut min = f64::INFINITY;

            for (&v, &x) in xs {
                let half_width = self.width(v) / 2.0;
                max = max.max(x + half_width);
                min = min.min(x - half_width);
            }

            let width = max - min;
            if width < min_width {
                min_width = width;
                best_alignment = Some(xs.clone());
            }
        }

        best_alignment
    }

    /// 获取节点宽度
    pub fn width(&self, node: NodeIndex) -> f64 {
        self.graph
            .node_label(node)
            .map(|label| label.width)
            .unwrap_or(0.0)
    }

    /// 对齐坐标
    ///
    /// 将所有对齐方向的位置结果对齐到指定的基准对齐。
    /// 通过调整偏移量使所有对齐具有相同的边界。
    ///
    /// # Arguments
    ///
    /// * `xss` - 包含四种对齐方向位置结果的映射（可变引用）
    /// * `align_to` - 作为基准的对齐方向
    ///
    /// # Examples
    ///
    /// ```
    /// use dagviz::position::bk::BrandesKoepf;
    /// use dagviz::graph::Graph;
    /// use indexmap::IndexMap;
    /// use crate::graph::NodeIndex;
    ///
    /// let graph = Graph::new();
    /// let bk = BrandesKoepf::new(graph);
    ///
    /// let mut xss = IndexMap::new();
    /// let mut ul = IndexMap::new();
    /// let mut ur = IndexMap::new();
    /// let n1 = NodeIndex::new(0);
    /// let n2 = NodeIndex::new(1);
    ///
    /// ul.insert(n1, 10.0);
    /// ul.insert(n2, 20.0);
    /// ur.insert(n1, 5.0);
    /// ur.insert(n2, 15.0);
    ///
    /// xss.insert("ul".to_string(), ul);
    /// xss.insert("ur".to_string(), ur);
    ///
    /// // 对齐到 ul
    /// bk.align_coordinates(&mut xss, "ul");
    ///
    /// // 验证对齐后的结果
    /// let ul_vals: Vec<f64> = xss["ul"].values().cloned().collect();
    /// let ur_vals: Vec<f64> = xss["ur"].values().cloned().collect();
    ///
    /// // 对齐后应该有相同的边界
    /// assert_eq!(ul_vals.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
    ///            ur_vals.iter().fold(f64::INFINITY, |a, &b| a.min(b)));
    /// ```
    pub fn align_coordinates(
        &self,
        xss: &mut IndexMap<String, IndexMap<NodeIndex, f64>>,
        align_to: &IndexMap<NodeIndex, f64>,
    ) {
        let align_to_vals: Vec<f64> = align_to.values().cloned().collect();
        let align_to_min = align_to_vals.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let align_to_max = align_to_vals
            .iter()
            .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        for vert in ["u", "d"] {
            for horiz in ["l", "r"] {
                let alignment = format!("{}{}", vert, horiz);
                if let Some(xs) = xss.get_mut(&alignment) {
                    let xs_vals: Vec<f64> = xs.values().cloned().collect();
                    let xs_min = xs_vals.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                    let xs_max = xs_vals.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

                    let delta = if horiz == "l" {
                        align_to_min - xs_min
                    } else {
                        align_to_max - xs_max
                    };

                    if delta != 0.0 {
                        for (_, x) in xs.iter_mut() {
                            *x += delta;
                        }
                    }
                }
            }
        }
    }

    /// 平衡坐标 - 函数式写法
    ///
    /// 根据四种对齐方向的位置结果，选择最优的平衡位置。
    /// 优先使用指定的对齐方向，否则使用 ul 对齐作为基准。
    ///
    /// # Arguments
    ///
    /// * `xss` - 包含四种对齐方向位置结果的映射
    /// * `align` - 可选的对齐方向，如果指定则优先使用
    ///
    /// # Examples
    ///
    /// ```
    /// use dagviz::position::bk::BrandesKoepf;
    /// use dagviz::graph::Graph;
    /// use dagviz::types::*;
    /// use indexmap::IndexMap;
    /// use crate::graph::NodeIndex;
    ///
    /// let graph = Graph::new();
    /// let bk = BrandesKoepf::new(graph);
    ///
    /// let mut xss = IndexMap::new();
    /// let mut ul = IndexMap::new();
    /// let mut ur = IndexMap::new();
    /// let n1 = NodeIndex::new(0);
    /// let n2 = NodeIndex::new(1);
    ///
    /// ul.insert(n1, 10.0);
    /// ul.insert(n2, 20.0);
    /// ur.insert(n1, 12.0);
    /// ur.insert(n2, 22.0);
    ///
    /// xss.insert("ul".to_string(), ul);
    /// xss.insert("ur".to_string(), ur);
    ///
    /// let balanced = bk.balance(&xss, Some("ur"));
    ///
    /// // 验证平衡结果
    /// assert_eq!(balanced.len(), 2);
    /// assert_eq!(balanced.get(&n1), Some(&12.0));
    /// assert_eq!(balanced.get(&n2), Some(&22.0));
    /// ```
    pub fn balance(
        &self,
        xss: &IndexMap<String, IndexMap<NodeIndex, f64>>,
        align: Option<&str>,
    ) -> IndexMap<NodeIndex, f64> {
        // 使用 ul 对齐作为基准，与 JavaScript 版本完全一致
        let ul_xs = xss.get("ul").expect("xss must contain 'ul' alignment");

        ul_xs
            .iter()
            .map(|(v, _)| {
                if let Some(align_key) = align {
                    // 如果指定了 align 参数，直接返回该对齐的值
                    if let Some(align_xs) = xss.get(align_key) {
                        if let Some(&x) = align_xs.get(v) {
                            return (*v, x);
                        }
                    }
                }

                // 收集所有对齐中该节点的 x 值
                let values: Vec<f64> = xss.values().filter_map(|xs| xs.get(v).copied()).collect();

                // 排序
                let mut sorted_values = values;
                sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

                // 与 JavaScript 版本完全一致：直接使用 (xs[1] + xs[2]) / 2
                // 如果数组长度不足，使用平均值
                let balanced_x = if sorted_values.len() >= 3 {
                    (sorted_values[1] + sorted_values[2]) / 2.0
                } else if sorted_values.len() == 2 {
                    (sorted_values[0] + sorted_values[1]) / 2.0
                } else {
                    sorted_values[0]
                };

                (*v, balanced_x)
            })
            .collect()
    }
}

/// 为图添加 Brandes-Köpf 位置计算
impl Graph {
    /// 使用 Brandes-Köpf 算法计算节点位置
    ///
    /// # Examples
    ///
    /// ```
    /// use dagviz::graph::Graph;
    /// use dagviz::types::{NodeLabel, Edge, EdgeLabel};
    /// use crate::graph::NodeIndex;
    ///
    /// let mut graph = Graph::new();
    ///
    /// // 创建简单图
    /// let mut node_a = NodeLabel::default();
    /// node_a.label = Some("A".to_string());
    /// let a = graph.add_node(node_a);
    ///
    /// let mut node_b = NodeLabel::default();
    /// node_b.label = Some("B".to_string());
    /// let b = graph.add_node(node_b);
    ///
    /// let mut node_c = NodeLabel::default();
    /// node_c.label = Some("C".to_string());
    /// let c = graph.add_node(node_c);
    ///
    /// graph.add_edge(Edge::new(a, b), EdgeLabel::default());
    /// graph.add_edge(Edge::new(b, c), EdgeLabel::default());
    ///
    /// let result = graph.compute_bk_positions();
    ///
    /// assert_eq!(result.positions.len(), 3);
    /// assert!(result.total_crossings >= 0);
    /// ```
    pub fn compute_bk_positions(&self) -> BKResult {
        let mut bk = BrandesKoepf::new(self.clone());
        bk.run()
    }
}
