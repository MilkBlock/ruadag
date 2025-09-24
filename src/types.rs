//! 类型定义
//!
//! 这个模块定义了 dagviz 中使用的核心类型，包括节点、边、配置等。

use crate::graph::NodeIndex;
use indexmap::IndexMap;

/// 边类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Edge {
    pub source: NodeIndex,
    pub target: NodeIndex,
}

impl Edge {
    pub fn new(source: NodeIndex, target: NodeIndex) -> Self {
        Edge { source, target }
    }
}

/// 虚拟节点类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dummy {
    /// 边虚拟节点
    Edge,
    /// 边标签代理节点
    EdgeProxy,
    /// 边界节点
    Border,
    /// 自环边节点
    SelfEdge,
    /// 根节点
    Root,
}
impl std::str::FromStr for Dummy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "edge" => Ok(Dummy::Edge),
            "edge-proxy" => Ok(Dummy::EdgeProxy),
            "border" => Ok(Dummy::Border),
            "selfedge" => Ok(Dummy::SelfEdge),
            "root" | "_root" => Ok(Dummy::Root),
            _ => Err(format!("Unknown dummy type: {}", s)),
        }
    }
}

/// 排序器类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ranker {
    /// 网络单纯形算法
    NetworkSimplex,
    /// 可行树算法
    FeasibleTree,
    /// 最长路径算法
    LongestPath,
    /// 紧树算法
    TightTree,
    /// 无排序器
    None,
}

impl Default for Ranker {
    fn default() -> Self {
        Ranker::NetworkSimplex
    }
}

/// 排序方向
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RankDirection {
    /// 从上到下
    TopBottom,
    /// 从下到上
    BottomTop,
    /// 从左到右
    LeftRight,
    /// 从右到左
    RightLeft,
}

impl Default for RankDirection {
    fn default() -> Self {
        RankDirection::TopBottom
    }
}

/// 标签位置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LabelPosition {
    /// 居中
    Center,
    /// 左侧
    Left,
    /// 右侧
    Right,
    /// 上方
    Top,
    /// 下方
    Bottom,
}

impl Default for LabelPosition {
    fn default() -> Self {
        LabelPosition::Center
    }
}

/// 节点标签，包含节点的所有属性
#[derive(Debug, Clone, Default)]
pub struct NodeLabel {
    /// 节点标签文本
    pub label: Option<String>,
    /// 节点宽度
    pub width: f64,
    /// 节点高度
    pub height: f64,
    /// 节点 X 坐标
    pub x: Option<f64>,
    /// 节点 Y 坐标
    pub y: Option<f64>,
    /// 节点层级（用于分层布局）
    pub rank: Option<i32>,
    /// 节点在层级中的顺序
    pub order: Option<usize>,
    /// 节点是否被隐藏
    pub hidden: bool,
    /// 父节点索引（用于子图）
    pub parent: Option<NodeIndex>,
    /// 虚拟节点类型
    pub dummy: Option<Dummy>,
    /// 最小层级（用于复合图）
    pub min_rank: Option<i32>,
    /// 最大层级（用于复合图）
    pub max_rank: Option<i32>,
    /// 边界节点数组（用于复合图）
    pub border_left: Vec<NodeIndex>,
    /// 边界节点数组（用于复合图）
    pub border_right: Vec<NodeIndex>,
    /// 顶部边界节点
    pub border_top: Option<NodeIndex>,
    /// 底部边界节点
    pub border_bottom: Option<NodeIndex>,
    /// 边界类型
    pub border_type: Option<String>,
    /// 自环边列表
    pub self_edges: Option<Vec<EdgeLabel>>,
    /// 边对象（用于虚拟节点）
    pub edge_obj: Option<Edge>,
    /// low值（用于NetworkSimplex算法）
    pub low: Option<i32>,
    /// lim值（用于NetworkSimplex算法）
    pub lim: Option<i32>,
    /// 自定义属性
    pub custom: IndexMap<String, serde_json::Value>,
}

/// 边标签，包含边的所有属性
#[derive(Debug, Clone)]
pub struct EdgeLabel {
    /// 边标签文本
    pub label: Option<String>,
    /// 边的最小长度
    pub minlen: i32,
    /// 边的权重
    pub weight: f64,
    /// 边是否被隐藏
    pub hidden: bool,
    /// 边的控制点列表（用于多控制点边）
    pub points: Vec<Point>,
    /// 边的路径类型（直线、曲线、折线等）
    pub path_type: PathType,
    /// 边标签的 X 坐标
    pub x: Option<f64>,
    /// 边标签的 Y 坐标
    pub y: Option<f64>,
    /// 边标签的宽度
    pub width: f64,
    /// 边标签的高度
    pub height: f64,
    /// 边是否被反转（用于无环化）
    pub reversed: Option<bool>,
    /// 反转边的原始名称
    pub forward_name: Option<String>,
    /// 边标签的偏移量
    pub labeloffset: f64,
    /// 边标签的位置
    pub labelpos: LabelPosition,
    /// cut值（用于NetworkSimplex算法）
    pub cutvalue: Option<i32>,
    /// 自定义属性
    pub custom: IndexMap<String, serde_json::Value>,
}

impl Default for EdgeLabel {
    fn default() -> Self {
        Self {
            label: None,
            minlen: 1, // 默认最小长度为 1，与 JavaScript 版本一致
            weight: 1.0,
            hidden: false,
            points: Vec::new(),
            path_type: PathType::default(),
            x: None,
            y: None,
            width: 0.0,
            height: 0.0,
            reversed: None,
            forward_name: None,
            labeloffset: 10.0,
            labelpos: LabelPosition::default(),
            cutvalue: None,
            custom: IndexMap::new(),
        }
    }
}

/// 点坐标
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// 路径类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathType {
    /// 直线
    Line,
    /// 贝塞尔曲线
    Bezier,
    /// 折线（多段直线）
    Polyline,
    /// 正交路径（水平和垂直线段）
    Orthogonal,
}

impl Default for PathType {
    fn default() -> Self {
        PathType::Bezier
    }
}

/// 图配置
#[derive(Debug, Clone)]
pub struct GraphConfig {
    /// 节点间距
    pub node_sep: f64,
    /// 层级间距
    pub rank_sep: f64,
    /// 边间距
    pub edge_sep: f64,
    /// 边的最小长度
    pub minlen: i32,
    /// 是否使用网络单纯形算法
    pub use_network_simplex: bool,
    /// 是否使用 Brandes-Köpf 算法
    pub use_brandes_koepf: bool,
    /// 布局方向
    pub direction: Direction,
    /// 对齐方式
    pub align: Align,
    /// 排序器类型
    pub ranker: Ranker,
    /// 排序方向
    pub rankdir: RankDirection,
    /// 无环化算法
    pub acyclicer: String,
    /// 图的最大层级
    pub max_rank: Option<i32>,
    /// 图的宽度
    pub width: Option<f64>,
    /// 图的高度
    pub height: Option<f64>,
    /// 左边距
    pub marginx: f64,
    /// 上边距
    pub marginy: f64,
    /// 虚拟节点链
    pub dummy_chains: Option<Vec<NodeIndex>>,
    /// 自定义属性
    pub custom: IndexMap<String, serde_json::Value>,
}

impl Default for GraphConfig {
    fn default() -> Self {
        Self {
            node_sep: 50.0,
            rank_sep: 50.0,
            edge_sep: 10.0,
            minlen: 1,
            use_network_simplex: true,
            use_brandes_koepf: true,
            direction: Direction::TopToBottom,
            align: Align::Center,
            ranker: Ranker::default(),
            rankdir: RankDirection::default(),
            acyclicer: "greedy".to_string(),
            max_rank: None,
            width: None,
            height: None,
            marginx: 20.0,
            marginy: 20.0,
            dummy_chains: None,
            custom: IndexMap::new(),
        }
    }
}

/// 布局方向
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    /// 从上到下
    TopToBottom,
    /// 从下到上
    BottomToTop,
    /// 从左到右
    LeftToRight,
    /// 从右到左
    RightToLeft,
}

/// 对齐方式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Align {
    /// 居中对齐
    Center,
    /// 左对齐
    Left,
    /// 右对齐
    Right,
    /// 上对齐
    Top,
    /// 下对齐
    Bottom,
}

/// 布局选项
#[derive(Debug, Clone)]
pub struct LayoutOptions {
    /// 图配置
    pub config: GraphConfig,
    /// 是否启用调试模式
    pub debug: bool,
    /// 是否启用调试计时
    pub debug_timing: bool,
    /// 是否显示边标签
    pub show_edge_labels: bool,
    /// 是否显示节点标签
    pub show_node_labels: bool,
    /// 是否禁用最优排序启发式
    pub disable_optimal_order_heuristic: bool,
    /// 自定义属性
    pub custom: IndexMap<String, serde_json::Value>,
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self {
            config: GraphConfig::default(),
            debug: false,
            debug_timing: false,
            show_edge_labels: true,
            show_node_labels: true,
            disable_optimal_order_heuristic: false,
            custom: IndexMap::new(),
        }
    }
}

/// 子图信息
#[derive(Debug, Clone)]
pub struct Subgraph {
    /// 子图名称
    pub name: String,
    /// 子图中的节点
    pub nodes: Vec<NodeIndex>,
    /// 子图配置
    pub config: GraphConfig,
}

impl Subgraph {
    pub fn new(name: String) -> Self {
        Self {
            name,
            nodes: Vec::new(),
            config: GraphConfig::default(),
        }
    }
}
