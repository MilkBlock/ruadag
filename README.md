# DagViz - Rust实现的Dagre图布局库

DagViz是dagre JavaScript库的Rust实现，用于有向图的自动布局。它提供了强大的算法来安排节点位置和路由边，使图的可视化更加清晰和美观。

## 特性

- 🚀 **高性能**: 使用Rust实现，提供出色的性能
- 📐 **多种布局算法**: 支持多种排名和排序算法
- 🎨 **灵活配置**: 丰富的配置选项满足不同需求
- 🔧 **易于使用**: 简洁的API设计
- 📦 **无依赖**: 核心算法不依赖外部库
- 🧪 **完整测试**: 包含全面的测试用例

## 快速开始

### 安装

在`Cargo.toml`中添加依赖：

```toml
[dependencies]
dagviz = "0.1.0"
```

### 基本使用

```rust
use dagviz::*;

fn main() {
    // 创建图
    let mut graph = Graph::new();
    
    // 添加节点
    graph.add_node("a".to_string(), NodeLabel {
        width: 50.0,
        height: 100.0,
        label: Some("Node A".to_string()),
        ..Default::default()
    });
    
    graph.add_node("b".to_string(), NodeLabel {
        width: 75.0,
        height: 200.0,
        label: Some("Node B".to_string()),
        ..Default::default()
    });
    
    // 添加边
    let edge = Edge::new("a".to_string(), "b".to_string());
    let _ = graph.add_edge(edge, EdgeLabel::default());
    
    // 执行布局
    layout(&mut graph, None);
    
    // 获取结果
    for node_id in graph.node_ids() {
        if let Some(label) = graph.node_label(&node_id) {
            println!("节点 {}: 位置({:.2}, {:.2})", 
                node_id,
                label.x.unwrap_or(0.0),
                label.y.unwrap_or(0.0)
            );
        }
    }
}
```

## 核心概念

### 图结构

DagViz使用有向图来表示数据，包含：

- **节点 (Nodes)**: 图中的实体，可以设置宽度、高度、标签等属性
- **边 (Edges)**: 连接节点的有向边，可以设置权重、最小长度等属性
- **配置 (Config)**: 图的全局设置，如节点间距、排名方向等

### 布局算法

DagViz实现了完整的布局流水线：

1. **排名 (Ranking)**: 为节点分配垂直层级
   - 最长路径算法
   - 网络单纯形算法
   - 紧树算法

2. **排序 (Ordering)**: 在每层内优化节点顺序以减少边交叉
   - 启发式排序
   - 交叉计数优化

3. **定位 (Positioning)**: 计算节点的最终坐标
   - Brandes-Köpf算法
   - 约束求解

## API参考

### 图操作

```rust
// 创建图
let mut graph = Graph::new();

// 添加节点
graph.add_node(id, label);

// 添加边
graph.add_edge(edge, label);

// 执行布局
layout(&mut graph, options);
```

### 节点标签

```rust
NodeLabel {
    width: 50.0,           // 节点宽度
    height: 100.0,         // 节点高度
    label: Some("文本"),    // 节点标签
    rank: Some(0),         // 排名（布局后设置）
    x: Some(10.0),         // X坐标（布局后设置）
    y: Some(20.0),         // Y坐标（布局后设置）
    ..Default::default()
}
```

### 边标签

```rust
EdgeLabel {
    minlen: 1.0,                    // 最小长度
    weight: 1.0,                    // 权重
    width: 0.0,                     // 标签宽度
    height: 0.0,                    // 标签高度
    labelpos: LabelPosition::Right, // 标签位置
    points: vec![],                 // 路径点（布局后设置）
    ..Default::default()
}
```

### 图配置

```rust
GraphConfig {
    nodesep: 50.0,              // 节点间距
    edgesep: 20.0,              // 边间距
    ranksep: 50.0,              // 排名间距
    rankdir: RankDirection::TopBottom, // 排名方向
    ranker: Ranker::NetworkSimplex,    // 排名算法
    ..Default::default()
}
```

## 配置选项

### 排名方向

- `TopBottom`: 从上到下（默认）
- `BottomTop`: 从下到上
- `LeftRight`: 从左到右
- `RightLeft`: 从右到左

### 排名算法

- `NetworkSimplex`: 网络单纯形算法（推荐）
- `TightTree`: 紧树算法
- `LongestPath`: 最长路径算法
- `None`: 不进行排名

### 布局选项

```rust
LayoutOptions {
    debug_timing: false,                    // 启用调试计时
    disable_optimal_order_heuristic: false, // 禁用最优排序启发式
    custom_order: None,                     // 自定义排序函数
}
```

## 示例

### 简单流程图

```rust
use dagviz::*;

fn create_flowchart() -> Graph {
    let mut graph = Graph::new();
    
    // 设置图配置
    let config = GraphConfig {
        rankdir: RankDirection::TopBottom,
        nodesep: 30.0,
        ranksep: 50.0,
        ..Default::default()
    };
    graph.set_config(config);
    
    // 添加节点
    graph.add_node("start".to_string(), NodeLabel {
        width: 80.0,
        height: 40.0,
        label: Some("开始".to_string()),
        ..Default::default()
    });
    
    graph.add_node("process".to_string(), NodeLabel {
        width: 100.0,
        height: 60.0,
        label: Some("处理".to_string()),
        ..Default::default()
    });
    
    graph.add_node("end".to_string(), NodeLabel {
        width: 80.0,
        height: 40.0,
        label: Some("结束".to_string()),
        ..Default::default()
    });
    
    // 添加边
    let _ = graph.add_edge(
        Edge::new("start".to_string(), "process".to_string()),
        EdgeLabel::default()
    );
    
    let _ = graph.add_edge(
        Edge::new("process".to_string(), "end".to_string()),
        EdgeLabel::default()
    );
    
    graph
}
```

### 复杂网络图

```rust
use dagviz::*;

fn create_network() -> Graph {
    let mut graph = Graph::new();
    
    // 设置配置
    let config = GraphConfig {
        rankdir: RankDirection::LeftRight,
        nodesep: 40.0,
        ranksep: 80.0,
        ranker: Ranker::NetworkSimplex,
        ..Default::default()
    };
    graph.set_config(config);
    
    // 添加多个节点和边
    for i in 0..5 {
        let node_id = format!("node_{}", i);
        graph.add_node(node_id.clone(), NodeLabel {
            width: 60.0,
            height: 40.0,
            label: Some(format!("节点 {}", i)),
            ..Default::default()
        });
        
        if i > 0 {
            let edge = Edge::new(
                format!("node_{}", i-1),
                node_id
            );
            let _ = graph.add_edge(edge, EdgeLabel {
                weight: 1.0,
                ..Default::default()
            });
        }
    }
    
    graph
}
```

## 性能优化

### 大图处理

对于大型图，可以调整以下参数来优化性能：

```rust
let config = GraphConfig {
    ranker: Ranker::LongestPath,  // 使用更快的排名算法
    ..Default::default()
};

let options = LayoutOptions {
    disable_optimal_order_heuristic: true,  // 禁用复杂的排序优化
    ..Default::default()
};
```

### 内存使用

- 使用`Graph::with_config()`来预设配置
- 避免频繁的节点和边操作
- 考虑使用图的分割算法处理超大图

## 测试

运行测试：

```bash
cargo test
```

运行示例：

```bash
cargo run --example basic
```

## 贡献

欢迎贡献代码！请确保：

1. 代码通过所有测试
2. 添加适当的文档
3. 遵循Rust编码规范

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 致谢

本项目基于 [dagre](https://github.com/dagrejs/dagre) JavaScript库，感谢原作者的优秀工作。

