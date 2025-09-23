//! 子图示例
//!
//! 这个文件展示了如何使用 dagviz 创建和管理子图（复合图）。
//! 子图允许将节点组织成层次结构，这对于复杂的图形布局非常有用。

use dagviz::graph::Graph;
use dagviz::layout;
use dagviz::types::*;
use petgraph::graph::NodeIndex;

/// 示例1：简单的父子关系
///
/// 创建一个包含两个子图的简单图：
/// - 子图A包含节点a1, a2
/// - 子图B包含节点b1, b2
/// - 根节点root包含子图A和B
pub fn simple_subgraph_example() -> Graph {
    let mut graph = Graph::new();

    // 创建根节点
    let root = graph.add_node(NodeLabel {
        label: Some("Root".to_string()),
        width: 100.0,
        height: 50.0,
        ..Default::default()
    });

    // 创建子图A
    let subgraph_a = graph.add_node(NodeLabel {
        label: Some("Subgraph A".to_string()),
        width: 80.0,
        height: 40.0,
        parent: Some(root),
        ..Default::default()
    });

    // 创建子图B
    let subgraph_b = graph.add_node(NodeLabel {
        label: Some("Subgraph B".to_string()),
        width: 80.0,
        height: 40.0,
        parent: Some(root),
        ..Default::default()
    });

    // 创建子图A中的节点
    let a1 = graph.add_node(NodeLabel {
        label: Some("A1".to_string()),
        width: 30.0,
        height: 20.0,
        parent: Some(subgraph_a),
        ..Default::default()
    });

    let a2 = graph.add_node(NodeLabel {
        label: Some("A2".to_string()),
        width: 30.0,
        height: 20.0,
        parent: Some(subgraph_a),
        ..Default::default()
    });

    // 创建子图B中的节点
    let b1 = graph.add_node(NodeLabel {
        label: Some("B1".to_string()),
        width: 30.0,
        height: 20.0,
        parent: Some(subgraph_b),
        ..Default::default()
    });

    let b2 = graph.add_node(NodeLabel {
        label: Some("B2".to_string()),
        width: 30.0,
        height: 20.0,
        parent: Some(subgraph_b),
        ..Default::default()
    });

    // 添加边
    let edge_a1_a2 = Edge::new(a1, a2);
    let edge_b1_b2 = Edge::new(b1, b2);
    let edge_a1_b1 = Edge::new(a1, b1);

    graph.add_edge(edge_a1_a2, EdgeLabel::default());
    graph.add_edge(edge_b1_b2, EdgeLabel::default());
    graph.add_edge(edge_a1_b1, EdgeLabel::default());

    graph
}

/// 示例2：多层嵌套子图
///
/// 创建一个具有多层嵌套的复杂子图结构：
/// - 根节点包含多个子图
/// - 子图内部还可以包含子子图
pub fn nested_subgraph_example() -> Graph {
    let mut graph = Graph::new();

    // 创建根节点
    let root = graph.add_node(NodeLabel {
        label: Some("Main System".to_string()),
        width: 200.0,
        height: 100.0,
        ..Default::default()
    });

    // 创建第一层子图：前端模块
    let frontend = graph.add_node(NodeLabel {
        label: Some("Frontend".to_string()),
        width: 150.0,
        height: 60.0,
        parent: Some(root),
        ..Default::default()
    });

    // 创建第一层子图：后端模块
    let backend = graph.add_node(NodeLabel {
        label: Some("Backend".to_string()),
        width: 150.0,
        height: 60.0,
        parent: Some(root),
        ..Default::default()
    });

    // 创建前端子模块：UI组件
    let ui_components = graph.add_node(NodeLabel {
        label: Some("UI Components".to_string()),
        width: 100.0,
        height: 40.0,
        parent: Some(frontend),
        ..Default::default()
    });

    // 创建前端子模块：状态管理
    let state_mgmt = graph.add_node(NodeLabel {
        label: Some("State Management".to_string()),
        width: 100.0,
        height: 40.0,
        parent: Some(frontend),
        ..Default::default()
    });

    // 创建后端子模块：API层
    let api_layer = graph.add_node(NodeLabel {
        label: Some("API Layer".to_string()),
        width: 100.0,
        height: 40.0,
        parent: Some(backend),
        ..Default::default()
    });

    // 创建后端子模块：数据库层
    let db_layer = graph.add_node(NodeLabel {
        label: Some("Database Layer".to_string()),
        width: 100.0,
        height: 40.0,
        parent: Some(backend),
        ..Default::default()
    });

    // 创建具体的组件节点
    let button = graph.add_node(NodeLabel {
        label: Some("Button".to_string()),
        width: 20.0,
        height: 15.0,
        parent: Some(ui_components),
        ..Default::default()
    });

    let input = graph.add_node(NodeLabel {
        label: Some("Input".to_string()),
        width: 20.0,
        height: 15.0,
        parent: Some(ui_components),
        ..Default::default()
    });

    let redux = graph.add_node(NodeLabel {
        label: Some("Redux".to_string()),
        width: 20.0,
        height: 15.0,
        parent: Some(state_mgmt),
        ..Default::default()
    });

    let rest_api = graph.add_node(NodeLabel {
        label: Some("REST API".to_string()),
        width: 20.0,
        height: 15.0,
        parent: Some(api_layer),
        ..Default::default()
    });

    let postgres = graph.add_node(NodeLabel {
        label: Some("PostgreSQL".to_string()),
        width: 20.0,
        height: 15.0,
        parent: Some(db_layer),
        ..Default::default()
    });

    // 添加连接边
    let edge_button_input = Edge::new(button, input);
    let edge_redux_rest = Edge::new(redux, rest_api);
    let edge_rest_postgres = Edge::new(rest_api, postgres);
    let edge_ui_state = Edge::new(ui_components, state_mgmt);
    let edge_api_db = Edge::new(api_layer, db_layer);

    graph.add_edge(edge_button_input, EdgeLabel::default());
    graph.add_edge(edge_redux_rest, EdgeLabel::default());
    graph.add_edge(edge_rest_postgres, EdgeLabel::default());
    graph.add_edge(edge_ui_state, EdgeLabel::default());
    graph.add_edge(edge_api_db, EdgeLabel::default());

    graph
}

/// 示例3：组织架构图
///
/// 创建一个公司组织架构的子图示例：
/// - CEO
///   - 技术部门
///     - 前端团队
///     - 后端团队
///   - 市场部门
///     - 销售团队
///     - 营销团队
pub fn organizational_chart_example() -> Graph {
    let mut graph = Graph::new();

    // CEO
    let ceo = graph.add_node(NodeLabel {
        label: Some("CEO".to_string()),
        width: 60.0,
        height: 30.0,
        ..Default::default()
    });

    // 技术部门
    let tech_dept = graph.add_node(NodeLabel {
        label: Some("Technology".to_string()),
        width: 100.0,
        height: 50.0,
        parent: Some(ceo),
        ..Default::default()
    });

    // 市场部门
    let market_dept = graph.add_node(NodeLabel {
        label: Some("Marketing".to_string()),
        width: 100.0,
        height: 50.0,
        parent: Some(ceo),
        ..Default::default()
    });

    // 前端团队
    let frontend_team = graph.add_node(NodeLabel {
        label: Some("Frontend Team".to_string()),
        width: 80.0,
        height: 30.0,
        parent: Some(tech_dept),
        ..Default::default()
    });

    // 后端团队
    let backend_team = graph.add_node(NodeLabel {
        label: Some("Backend Team".to_string()),
        width: 80.0,
        height: 30.0,
        parent: Some(tech_dept),
        ..Default::default()
    });

    // 销售团队
    let sales_team = graph.add_node(NodeLabel {
        label: Some("Sales Team".to_string()),
        width: 80.0,
        height: 30.0,
        parent: Some(market_dept),
        ..Default::default()
    });

    // 营销团队
    let marketing_team = graph.add_node(NodeLabel {
        label: Some("Marketing Team".to_string()),
        width: 80.0,
        height: 30.0,
        parent: Some(market_dept),
        ..Default::default()
    });

    // 添加团队成员
    let frontend_lead = graph.add_node(NodeLabel {
        label: Some("Frontend Lead".to_string()),
        width: 25.0,
        height: 15.0,
        parent: Some(frontend_team),
        ..Default::default()
    });

    let backend_lead = graph.add_node(NodeLabel {
        label: Some("Backend Lead".to_string()),
        width: 25.0,
        height: 15.0,
        parent: Some(backend_team),
        ..Default::default()
    });

    let sales_manager = graph.add_node(NodeLabel {
        label: Some("Sales Manager".to_string()),
        width: 25.0,
        height: 15.0,
        parent: Some(sales_team),
        ..Default::default()
    });

    let marketing_manager = graph.add_node(NodeLabel {
        label: Some("Marketing Manager".to_string()),
        width: 25.0,
        height: 15.0,
        parent: Some(marketing_team),
        ..Default::default()
    });

    // 添加协作边
    let edge_frontend_backend = Edge::new(frontend_lead, backend_lead);
    let edge_sales_marketing = Edge::new(sales_manager, marketing_manager);
    let edge_tech_market = Edge::new(tech_dept, market_dept);

    graph.add_edge(edge_frontend_backend, EdgeLabel::default());
    graph.add_edge(edge_sales_marketing, EdgeLabel::default());
    graph.add_edge(edge_tech_market, EdgeLabel::default());

    graph
}

/// 示例4：软件架构分层图
///
/// 创建一个典型的软件架构分层子图：
/// - 表示层
/// - 业务逻辑层
/// - 数据访问层
/// - 数据存储层
pub fn layered_architecture_example() -> Graph {
    let mut graph = Graph::new();

    // 表示层
    let presentation = graph.add_node(NodeLabel {
        label: Some("Presentation Layer".to_string()),
        width: 200.0,
        height: 40.0,
        ..Default::default()
    });

    // 业务逻辑层
    let business = graph.add_node(NodeLabel {
        label: Some("Business Logic Layer".to_string()),
        width: 200.0,
        height: 40.0,
        parent: Some(presentation),
        ..Default::default()
    });

    // 数据访问层
    let data_access = graph.add_node(NodeLabel {
        label: Some("Data Access Layer".to_string()),
        width: 200.0,
        height: 40.0,
        parent: Some(business),
        ..Default::default()
    });

    // 数据存储层
    let data_storage = graph.add_node(NodeLabel {
        label: Some("Data Storage Layer".to_string()),
        width: 200.0,
        height: 40.0,
        parent: Some(data_access),
        ..Default::default()
    });

    // 表示层组件
    let web_ui = graph.add_node(NodeLabel {
        label: Some("Web UI".to_string()),
        width: 40.0,
        height: 20.0,
        parent: Some(presentation),
        ..Default::default()
    });

    let mobile_ui = graph.add_node(NodeLabel {
        label: Some("Mobile UI".to_string()),
        width: 40.0,
        height: 20.0,
        parent: Some(presentation),
        ..Default::default()
    });

    // 业务逻辑组件
    let user_service = graph.add_node(NodeLabel {
        label: Some("User Service".to_string()),
        width: 40.0,
        height: 20.0,
        parent: Some(business),
        ..Default::default()
    });

    let order_service = graph.add_node(NodeLabel {
        label: Some("Order Service".to_string()),
        width: 40.0,
        height: 20.0,
        parent: Some(business),
        ..Default::default()
    });

    // 数据访问组件
    let user_repository = graph.add_node(NodeLabel {
        label: Some("User Repository".to_string()),
        width: 40.0,
        height: 20.0,
        parent: Some(data_access),
        ..Default::default()
    });

    let order_repository = graph.add_node(NodeLabel {
        label: Some("Order Repository".to_string()),
        width: 40.0,
        height: 20.0,
        parent: Some(data_access),
        ..Default::default()
    });

    // 数据存储组件
    let user_db = graph.add_node(NodeLabel {
        label: Some("User DB".to_string()),
        width: 40.0,
        height: 20.0,
        parent: Some(data_storage),
        ..Default::default()
    });

    let order_db = graph.add_node(NodeLabel {
        label: Some("Order DB".to_string()),
        width: 40.0,
        height: 20.0,
        parent: Some(data_storage),
        ..Default::default()
    });

    // 添加层间连接
    let edge_web_user = Edge::new(web_ui, user_service);
    let edge_mobile_order = Edge::new(mobile_ui, order_service);
    let edge_user_repo = Edge::new(user_service, user_repository);
    let edge_order_repo = Edge::new(order_service, order_repository);
    let edge_user_db = Edge::new(user_repository, user_db);
    let edge_order_db = Edge::new(order_repository, order_db);

    graph.add_edge(edge_web_user, EdgeLabel::default());
    graph.add_edge(edge_mobile_order, EdgeLabel::default());
    graph.add_edge(edge_user_repo, EdgeLabel::default());
    graph.add_edge(edge_order_repo, EdgeLabel::default());
    graph.add_edge(edge_user_db, EdgeLabel::default());
    graph.add_edge(edge_order_db, EdgeLabel::default());

    graph
}

/// 打印子图层次结构
pub fn print_subgraph_hierarchy(graph: &Graph, root: NodeIndex, indent: usize) {
    if let Some(label) = graph.node_label(root) {
        let spaces = "  ".repeat(indent);
        println!("{}{}", spaces, label.label.as_deref().unwrap_or("Unnamed"));

        // 查找所有子节点
        for node_id in graph.node_indices() {
            if let Some(node_label) = graph.node_label(node_id) {
                if node_label.parent == Some(root) {
                    print_subgraph_hierarchy(graph, node_id, indent + 1);
                }
            }
        }
    }
}

/// 获取所有根节点（没有父节点的节点）
pub fn get_root_nodes(graph: &Graph) -> Vec<NodeIndex> {
    graph
        .node_indices()
        .filter(|&node_id| {
            if let Some(label) = graph.node_label(node_id) {
                label.parent.is_none()
            } else {
                false
            }
        })
        .collect()
}

/// 获取节点的所有子节点
pub fn get_children(graph: &Graph, parent: NodeIndex) -> Vec<NodeIndex> {
    graph
        .node_indices()
        .filter(|&node_id| {
            if let Some(label) = graph.node_label(node_id) {
                label.parent == Some(parent)
            } else {
                false
            }
        })
        .collect()
}

/// 获取节点的所有后代节点（递归）
pub fn get_descendants(graph: &Graph, parent: NodeIndex) -> Vec<NodeIndex> {
    let mut descendants = Vec::new();
    let children = get_children(graph, parent);

    for child in children {
        descendants.push(child);
        descendants.extend(get_descendants(graph, child));
    }

    descendants
}

/// 将图转换为SVG格式
///
/// 这个函数将Graph结构转换为SVG格式的字符串，支持：
/// - 节点渲染（矩形框）
/// - 边渲染（直线）
/// - 子图渲染（带边框的组）
/// - 标签文本
/// - 坐标定位
pub fn graph_to_svg(graph: &Graph) -> String {
    // 计算图的边界
    let (min_x, min_y, max_x, max_y) = calculate_graph_bounds(graph);

    // 添加边距
    let margin = 50.0;
    let width = max_x - min_x + 2.0 * margin;
    let height = max_y - min_y + 2.0 * margin;

    let mut svg = String::new();

    // SVG头部
    svg.push_str(&format!(
        r#"<svg width="{:.1}" height="{:.1}" xmlns="http://www.w3.org/2000/svg">"#,
        width, height
    ));
    svg.push('\n');

    // 添加背景
    svg.push_str(&format!(
        r#"  <rect width="100%" height="100%" fill="white" stroke="none"/>"#,
    ));
    svg.push('\n');

    // 渲染子图组（按层次结构）
    let root_nodes = get_root_nodes(graph);
    for root in root_nodes {
        render_subgraph_svg(graph, root, &mut svg, min_x, min_y, margin);
    }

    // 渲染所有边
    for edge in graph.edges() {
        if let Some(edge_label) = graph.edge_label(&edge) {
            render_edge_svg(graph, &edge, edge_label, &mut svg, min_x, min_y, margin);
        }
    }

    // 渲染所有节点
    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            render_node_svg(node_id, label, &mut svg, min_x, min_y, margin);
        }
    }

    svg.push_str("</svg>");
    svg
}

/// 计算图的边界
fn calculate_graph_bounds(graph: &Graph) -> (f64, f64, f64, f64) {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            if let (Some(x), Some(y)) = (label.x, label.y) {
                let half_width = label.width / 2.0;
                let half_height = label.height / 2.0;

                min_x = min_x.min(x - half_width);
                min_y = min_y.min(y - half_height);
                max_x = max_x.max(x + half_width);
                max_y = max_y.max(y + half_height);
            }
        }
    }

    // 如果没有节点有坐标，返回默认值
    if min_x == f64::INFINITY {
        (0.0, 0.0, 100.0, 100.0)
    } else {
        (min_x, min_y, max_x, max_y)
    }
}

/// 渲染子图组
fn render_subgraph_svg(
    graph: &Graph,
    parent: NodeIndex,
    svg: &mut String,
    min_x: f64,
    min_y: f64,
    margin: f64,
) {
    let children = get_children(graph, parent);
    if children.is_empty() {
        return;
    }

    // 计算子图的边界
    let (sub_min_x, sub_min_y, sub_max_x, sub_max_y) = calculate_subgraph_bounds(graph, &children);

    let x = sub_min_x - min_x + margin - 10.0;
    let y = sub_min_y - min_y + margin - 10.0;
    let width = sub_max_x - sub_min_x + 20.0;
    let height = sub_max_y - sub_min_y + 20.0;

    // 获取父节点标签
    let parent_label = if let Some(label) = graph.node_label(parent) {
        label.label.as_deref().unwrap_or("Subgraph")
    } else {
        "Subgraph"
    };

    // 渲染子图边框
    svg.push_str(&format!(r#"  <g id="subgraph-{:?}">"#, parent));
    svg.push('\n');

    // 子图背景
    svg.push_str(&format!(
        r#"    <rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" 
              fill="none" stroke="gray" stroke-width="2" stroke-dasharray="5,5"/>"#,
        x, y, width, height
    ));
    svg.push('\n');

    // 子图标签
    svg.push_str(&format!(
        r#"    <text x="{:.1}" y="{:.1}" font-family="Arial, sans-serif" font-size="12" 
              fill="gray" text-anchor="start">{}</text>"#,
        x + 5.0,
        y - 5.0,
        parent_label
    ));
    svg.push('\n');

    // 递归渲染子子图
    for child in children {
        render_subgraph_svg(graph, child, svg, min_x, min_y, margin);
    }

    svg.push_str("  </g>\n");
}

/// 计算子图的边界
fn calculate_subgraph_bounds(graph: &Graph, nodes: &[NodeIndex]) -> (f64, f64, f64, f64) {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for &node_id in nodes {
        if let Some(label) = graph.node_label(node_id) {
            if let (Some(x), Some(y)) = (label.x, label.y) {
                let half_width = label.width / 2.0;
                let half_height = label.height / 2.0;

                min_x = min_x.min(x - half_width);
                min_y = min_y.min(y - half_height);
                max_x = max_x.max(x + half_width);
                max_y = max_y.max(y + half_height);
            }
        }
    }

    if min_x == f64::INFINITY {
        (0.0, 0.0, 100.0, 100.0)
    } else {
        (min_x, min_y, max_x, max_y)
    }
}

/// 渲染节点
fn render_node_svg(
    _node_id: NodeIndex,
    label: &NodeLabel,
    svg: &mut String,
    min_x: f64,
    min_y: f64,
    margin: f64,
) {
    if let (Some(x), Some(y)) = (label.x, label.y) {
        let svg_x = x - min_x + margin - label.width / 2.0;
        let svg_y = y - min_y + margin - label.height / 2.0;

        // 根据是否有父节点选择不同的样式
        let (fill_color, stroke_color, stroke_width) = if label.parent.is_some() {
            ("#e1f5fe", "#01579b", 2) // 子节点：浅蓝色
        } else {
            ("#f3e5f5", "#4a148c", 2) // 根节点：浅紫色
        };

        // 节点矩形
        svg.push_str(&format!(
            r#"  <rect x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" 
                  fill="{}" stroke="{}" stroke-width="{}" rx="5"/>"#,
            svg_x, svg_y, label.width, label.height, fill_color, stroke_color, stroke_width
        ));
        svg.push('\n');

        // 节点标签
        if let Some(text) = &label.label {
            let text_x = svg_x + label.width / 2.0;
            let text_y = svg_y + label.height / 2.0 + 4.0; // 稍微向下偏移以居中

            svg.push_str(&format!(
                r#"  <text x="{:.1}" y="{:.1}" font-family="Arial, sans-serif" font-size="10" 
                      fill="black" text-anchor="middle" dominant-baseline="middle">{}</text>"#,
                text_x, text_y, text
            ));
            svg.push('\n');
        }

        // 如果是虚拟节点，添加特殊标记
        if label.dummy.is_some() {
            svg.push_str(&format!(
                r#"  <circle cx="{:.1}" cy="{:.1}" r="3" fill="red"/>"#,
                svg_x + label.width - 5.0,
                svg_y + 5.0
            ));
            svg.push('\n');
        }
    }
}

/// 渲染边
fn render_edge_svg(
    graph: &Graph,
    edge: &Edge,
    edge_label: &EdgeLabel,
    svg: &mut String,
    min_x: f64,
    min_y: f64,
    margin: f64,
) {
    let source_label = graph.node_label(edge.source);
    let target_label = graph.node_label(edge.target);

    if let (Some(source), Some(target)) = (source_label, target_label) {
        if let (Some(sx), Some(sy), Some(tx), Some(ty)) = (source.x, source.y, target.x, target.y) {
            let svg_sx = sx - min_x + margin;
            let svg_sy = sy - min_y + margin;
            let svg_tx = tx - min_x + margin;
            let svg_ty = ty - min_y + margin;

            // 计算边的起点和终点（考虑节点大小）
            let (start_x, start_y) = calculate_edge_start_point(
                svg_sx,
                svg_sy,
                source.width,
                source.height,
                svg_tx,
                svg_ty,
            );
            let (end_x, end_y) = calculate_edge_end_point(
                svg_sx,
                svg_sy,
                svg_tx,
                svg_ty,
                target.width,
                target.height,
            );

            // 渲染边（使用折线）
            let path =
                create_polyline_path(start_x, start_y, end_x, end_y, source.width, target.width);
            svg.push_str(&format!(
                r#"  <path d="{}" stroke="gray" stroke-width="1" fill="none" marker-end="url(#arrowhead)"/>"#,
                path
            ));
            svg.push('\n');

            // 渲染边标签（如果有）
            if let Some(label_text) = &edge_label.custom.get("label").and_then(|v| v.as_str()) {
                let label_x = (start_x + end_x) / 2.0;
                let label_y = (start_y + end_y) / 2.0;

                svg.push_str(&format!(
                    r#"  <text x="{:.1}" y="{:.1}" font-family="Arial, sans-serif" font-size="8" 
                          fill="gray" text-anchor="middle" dominant-baseline="middle">{}</text>"#,
                    label_x, label_y, label_text
                ));
                svg.push('\n');
            }
        }
    }
}

/// 计算边的起点
fn calculate_edge_start_point(sx: f64, sy: f64, sw: f64, _sh: f64, tx: f64, ty: f64) -> (f64, f64) {
    let dx = tx - sx;
    let dy = ty - sy;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance > 0.0 {
        let ratio = (sw / 2.0) / distance;
        (sx + dx * ratio, sy + dy * ratio)
    } else {
        (sx, sy)
    }
}

/// 计算边的终点
fn calculate_edge_end_point(sx: f64, sy: f64, tx: f64, ty: f64, tw: f64, _th: f64) -> (f64, f64) {
    let dx = tx - sx;
    let dy = ty - sy;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance > 0.0 {
        let ratio = (tw / 2.0) / distance;
        (tx - dx * ratio, ty - dy * ratio)
    } else {
        (tx, ty)
    }
}

/// 创建折线路径
/// 根据起点和终点创建美观的折线路径
fn create_polyline_path(
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
    _source_width: f64,
    _target_width: f64,
) -> String {
    let dx = end_x - start_x;
    let dy = end_y - start_y;
    let distance = (dx * dx + dy * dy).sqrt();

    // 如果距离太短，直接画直线
    if distance < 20.0 {
        return format!(
            "M {:.1} {:.1} L {:.1} {:.1}",
            start_x, start_y, end_x, end_y
        );
    }

    // 根据方向确定控制点位置
    let (control1_x, control1_y, control2_x, control2_y) = if dx.abs() > dy.abs() {
        // 水平方向为主
        if dx > 0.0 {
            // 从左到右
            let mid_x = start_x + dx * 0.5;
            (mid_x, start_y, mid_x, end_y)
        } else {
            // 从右到左
            let mid_x = start_x + dx * 0.5;
            (mid_x, start_y, mid_x, end_y)
        }
    } else {
        // 垂直方向为主
        if dy > 0.0 {
            // 从上到下
            let mid_y = start_y + dy * 0.5;
            (start_x, mid_y, end_x, mid_y)
        } else {
            // 从下到上
            let mid_y = start_y + dy * 0.5;
            (start_x, mid_y, end_x, mid_y)
        }
    };

    // 创建贝塞尔曲线路径
    format!(
        "M {:.1} {:.1} C {:.1} {:.1} {:.1} {:.1} {:.1} {:.1}",
        start_x, start_y, control1_x, control1_y, control2_x, control2_y, end_x, end_y
    )
}

/// 添加SVG箭头标记定义
fn add_svg_definitions(svg: &mut String) {
    svg.push_str(r#"  <defs>"#);
    svg.push('\n');
    svg.push_str(
        r#"    <marker id="arrowhead" markerWidth="10" markerHeight="7" 
                 refX="9" refY="3.5" orient="auto">"#,
    );
    svg.push('\n');
    svg.push_str(r#"      <polygon points="0 0, 10 3.5, 0 7" fill="gray" />"#);
    svg.push('\n');
    svg.push_str(r#"    </marker>"#);
    svg.push('\n');
    svg.push_str(r#"  </defs>"#);
    svg.push('\n');
}

/// 增强版SVG转换函数，包含箭头定义
pub fn graph_to_svg_enhanced(graph: &Graph) -> String {
    // 计算图的边界
    let (min_x, min_y, max_x, max_y) = calculate_graph_bounds(graph);

    // 添加边距
    let margin = 50.0;
    let width = max_x - min_x + 2.0 * margin;
    let height = max_y - min_y + 2.0 * margin;

    let mut svg = String::new();

    // SVG头部
    svg.push_str(&format!(
        r#"<svg width="{:.1}" height="{:.1}" xmlns="http://www.w3.org/2000/svg">"#,
        width, height
    ));
    svg.push('\n');

    // 添加定义（箭头等）
    add_svg_definitions(&mut svg);

    // 添加背景
    svg.push_str(&format!(
        r#"  <rect width="100%" height="100%" fill="white" stroke="none"/>"#,
    ));
    svg.push('\n');

    // 渲染子图组（按层次结构）
    let root_nodes = get_root_nodes(graph);
    for root in root_nodes {
        render_subgraph_svg(graph, root, &mut svg, min_x, min_y, margin);
    }

    // 渲染所有边
    for edge in graph.edges() {
        if let Some(edge_label) = graph.edge_label(&edge) {
            render_edge_svg(graph, &edge, edge_label, &mut svg, min_x, min_y, margin);
        }
    }

    // 渲染所有节点
    for node_id in graph.node_indices() {
        if let Some(label) = graph.node_label(node_id) {
            render_node_svg(node_id, label, &mut svg, min_x, min_y, margin);
        }
    }

    svg.push_str("</svg>");
    svg
}

/// 保存SVG到文件
pub fn save_graph_as_svg(graph: &Graph, filename: &str) -> Result<(), std::io::Error> {
    let svg_content = graph_to_svg_enhanced(graph);
    std::fs::write(filename, svg_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_subgraph_creation() {
        let graph = simple_subgraph_example();

        // 检查图的基本属性
        assert!(graph.node_count() > 0);

        // 检查根节点
        let root_nodes = get_root_nodes(&graph);
        assert_eq!(root_nodes.len(), 1);

        // 检查子图结构
        let root = root_nodes[0];
        let children = get_children(&graph, root);
        assert_eq!(children.len(), 2); // 两个子图A和B
    }

    #[test]
    fn test_nested_subgraph_creation() {
        let graph = nested_subgraph_example();

        // 检查多层嵌套
        let root_nodes = get_root_nodes(&graph);
        assert_eq!(root_nodes.len(), 1);

        let root = root_nodes[0];
        let first_level = get_children(&graph, root);
        assert_eq!(first_level.len(), 2); // 前端和后端

        // 检查第二层嵌套
        for child in first_level {
            let second_level = get_children(&graph, child);
            assert!(!second_level.is_empty()); // 每个第一层子图都应该有子节点
        }
    }

    #[test]
    fn test_organizational_chart() {
        let graph = organizational_chart_example();

        // 检查CEO是根节点
        let root_nodes = get_root_nodes(&graph);
        assert_eq!(root_nodes.len(), 1);

        let ceo = root_nodes[0];
        let departments = get_children(&graph, ceo);
        assert_eq!(departments.len(), 2); // 技术部门和市场部门
    }

    #[test]
    fn test_layered_architecture() {
        let graph = layered_architecture_example();

        // 检查分层结构
        let root_nodes = get_root_nodes(&graph);
        assert_eq!(root_nodes.len(), 1);

        let presentation = root_nodes[0];
        let presentation_children = get_children(&graph, presentation);
        assert_eq!(presentation_children.len(), 3); // business层 + web_ui + mobile_ui

        // 找到business层（它应该是presentation的子节点之一）
        let business = presentation_children
            .iter()
            .find(|&&node_id| {
                if let Some(label) = graph.node_label(node_id) {
                    label.label.as_deref() == Some("Business Logic Layer")
                } else {
                    false
                }
            })
            .unwrap();

        let business_children = get_children(&graph, *business);
        assert_eq!(business_children.len(), 3); // data_access + user_service + order_service

        // 找到data_access层
        let data_access = business_children
            .iter()
            .find(|&&node_id| {
                if let Some(label) = graph.node_label(node_id) {
                    label.label.as_deref() == Some("Data Access Layer")
                } else {
                    false
                }
            })
            .unwrap();

        let data_access_children = get_children(&graph, *data_access);
        assert_eq!(data_access_children.len(), 3); // data_storage + user_repository + order_repository

        // 找到data_storage层
        let data_storage = data_access_children
            .iter()
            .find(|&&node_id| {
                if let Some(label) = graph.node_label(node_id) {
                    label.label.as_deref() == Some("Data Storage Layer")
                } else {
                    false
                }
            })
            .unwrap();

        let data_storage_children = get_children(&graph, *data_storage);
        assert_eq!(data_storage_children.len(), 2); // user_db + order_db
    }

    #[test]
    fn test_svg_generation() {
        let graph = simple_subgraph_example();

        // 测试基本SVG生成
        let svg = graph_to_svg(&graph);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("xmlns=\"http://www.w3.org/2000/svg\""));

        // 测试增强版SVG生成
        let enhanced_svg = graph_to_svg_enhanced(&graph);
        assert!(enhanced_svg.contains("<svg"));
        assert!(enhanced_svg.contains("</svg>"));
        assert!(enhanced_svg.contains("<defs>"));
        assert!(enhanced_svg.contains("</defs>"));
        assert!(enhanced_svg.contains("arrowhead"));
    }

    #[test]
    fn test_svg_with_coordinates() {
        let mut graph = simple_subgraph_example();

        // 手动设置一些坐标 - 使用两遍方法避免借用冲突
        let node_indices: Vec<_> = graph.node_indices().collect();
        for (i, node_id) in node_indices.iter().enumerate() {
            if let Some(label) = graph.node_label_mut(*node_id) {
                label.x = Some(i as f64 * 100.0);
                label.y = Some(i as f64 * 50.0);
            }
        }

        let svg = graph_to_svg_enhanced(&graph);
        assert!(svg.contains("<svg"));

        // 检查是否包含节点矩形
        assert!(svg.contains("<rect"));

        // 检查是否包含文本
        assert!(svg.contains("<text"));
    }

    #[test]
    fn test_svg_file_save() {
        let graph = simple_subgraph_example();

        // 测试保存到临时文件
        let temp_file = "test_temp.svg";
        let result = save_graph_as_svg(&graph, temp_file);
        assert!(result.is_ok());

        // 检查文件是否被创建
        assert!(std::path::Path::new(temp_file).exists());

        // 清理临时文件
        let _ = std::fs::remove_file(temp_file);
    }
}

fn main() {
    println!("=== 子图示例 ===\n");

    // 示例1：简单子图
    println!("1. 简单子图示例:");
    let simple_graph = simple_subgraph_example();
    let root_nodes = get_root_nodes(&simple_graph);
    if !root_nodes.is_empty() {
        print_subgraph_hierarchy(&simple_graph, root_nodes[0], 0);
    }
    println!();

    // 示例2：嵌套子图
    println!("2. 嵌套子图示例:");
    let nested_graph = nested_subgraph_example();
    let root_nodes = get_root_nodes(&nested_graph);
    if !root_nodes.is_empty() {
        print_subgraph_hierarchy(&nested_graph, root_nodes[0], 0);
    }
    println!();

    // 示例3：组织架构图
    println!("3. 组织架构图示例:");
    let org_graph = organizational_chart_example();
    let root_nodes = get_root_nodes(&org_graph);
    if !root_nodes.is_empty() {
        print_subgraph_hierarchy(&org_graph, root_nodes[0], 0);
    }
    println!();

    // 示例4：分层架构图
    println!("4. 分层架构图示例:");
    let layered_graph = layered_architecture_example();
    let root_nodes = get_root_nodes(&layered_graph);
    if !root_nodes.is_empty() {
        print_subgraph_hierarchy(&layered_graph, root_nodes[0], 0);
    }
    println!();

    // 运行布局算法并生成SVG
    println!("5. 运行布局算法并生成SVG:");
    let mut layout_graph = simple_subgraph_example();
    layout::layout(&mut layout_graph, Some(&LayoutOptions::default()));
    println!("布局算法执行成功！");

    // 生成SVG
    let svg_content = graph_to_svg_enhanced(&layout_graph);
    println!("SVG内容长度: {} 字符", svg_content.len());

    // 保存到文件
    match save_graph_as_svg(&layout_graph, "simple_subgraph.svg") {
        Ok(_) => println!("SVG文件已保存为: simple_subgraph.svg"),
        Err(e) => println!("保存SVG文件失败: {}", e),
    }

    // 生成其他示例的SVG
    println!("\n6. 生成其他示例的SVG:");

    // 嵌套子图示例
    let mut nested_graph = nested_subgraph_example();
    layout::layout(&mut nested_graph, Some(&LayoutOptions::default()));
    match save_graph_as_svg(&nested_graph, "nested_subgraph.svg") {
        Ok(_) => println!("嵌套子图SVG已保存为: nested_subgraph.svg"),
        Err(e) => println!("保存嵌套子图SVG失败: {}", e),
    }

    // 组织架构图示例
    let mut org_graph = organizational_chart_example();
    layout::layout(&mut org_graph, Some(&LayoutOptions::default()));
    match save_graph_as_svg(&org_graph, "organizational_chart.svg") {
        Ok(_) => println!("组织架构图SVG已保存为: organizational_chart.svg"),
        Err(e) => println!("保存组织架构图SVG失败: {}", e),
    }

    // 分层架构图示例
    let mut layered_graph = layered_architecture_example();
    layout::layout(&mut layered_graph, Some(&LayoutOptions::default()));
    match save_graph_as_svg(&layered_graph, "layered_architecture.svg") {
        Ok(_) => println!("分层架构图SVG已保存为: layered_architecture.svg"),
        Err(e) => println!("保存分层架构图SVG失败: {}", e),
    }

    // 复杂网络图示例（展示折线效果）
    println!("\n7. 生成复杂网络图（展示折线效果）:");
    let mut complex_graph = create_complex_network_example();
    layout::layout(&mut complex_graph, Some(&LayoutOptions::default()));
    match save_graph_as_svg(&complex_graph, "complex_network.svg") {
        Ok(_) => println!("复杂网络图SVG已保存为: complex_network.svg"),
        Err(e) => println!("保存复杂网络图SVG失败: {}", e),
    }
}

/// 创建复杂网络图示例
/// 这个示例包含多个交叉边和复杂连接，用于展示折线效果
fn create_complex_network_example() -> Graph {
    let mut graph = Graph::new();

    // 创建中心节点
    let center = graph.add_node(NodeLabel {
        label: Some("中心节点".to_string()),
        width: 80.0,
        height: 40.0,
        ..Default::default()
    });

    // 创建周围的节点
    let node1 = graph.add_node(NodeLabel {
        label: Some("节点1".to_string()),
        width: 60.0,
        height: 30.0,
        ..Default::default()
    });

    let node2 = graph.add_node(NodeLabel {
        label: Some("节点2".to_string()),
        width: 60.0,
        height: 30.0,
        ..Default::default()
    });

    let node3 = graph.add_node(NodeLabel {
        label: Some("节点3".to_string()),
        width: 60.0,
        height: 30.0,
        ..Default::default()
    });

    let node4 = graph.add_node(NodeLabel {
        label: Some("节点4".to_string()),
        width: 60.0,
        height: 30.0,
        ..Default::default()
    });

    let node5 = graph.add_node(NodeLabel {
        label: Some("节点5".to_string()),
        width: 60.0,
        height: 30.0,
        ..Default::default()
    });

    let node6 = graph.add_node(NodeLabel {
        label: Some("节点6".to_string()),
        width: 60.0,
        height: 30.0,
        ..Default::default()
    });

    // 创建子节点
    let sub1 = graph.add_node(NodeLabel {
        label: Some("子节点1".to_string()),
        width: 50.0,
        height: 25.0,
        parent: Some(node1),
        ..Default::default()
    });

    let sub2 = graph.add_node(NodeLabel {
        label: Some("子节点2".to_string()),
        width: 50.0,
        height: 25.0,
        parent: Some(node2),
        ..Default::default()
    });

    let sub3 = graph.add_node(NodeLabel {
        label: Some("子节点3".to_string()),
        width: 50.0,
        height: 25.0,
        parent: Some(node3),
        ..Default::default()
    });

    // 添加复杂的边连接
    // 中心节点到所有周围节点
    graph.add_edge(Edge::new(center, node1), EdgeLabel::default());
    graph.add_edge(Edge::new(center, node2), EdgeLabel::default());
    graph.add_edge(Edge::new(center, node3), EdgeLabel::default());
    graph.add_edge(Edge::new(center, node4), EdgeLabel::default());
    graph.add_edge(Edge::new(center, node5), EdgeLabel::default());
    graph.add_edge(Edge::new(center, node6), EdgeLabel::default());

    // 周围节点之间的连接（形成交叉）
    graph.add_edge(Edge::new(node1, node3), EdgeLabel::default());
    graph.add_edge(Edge::new(node2, node4), EdgeLabel::default());
    graph.add_edge(Edge::new(node3, node5), EdgeLabel::default());
    graph.add_edge(Edge::new(node4, node6), EdgeLabel::default());
    graph.add_edge(Edge::new(node5, node1), EdgeLabel::default());
    graph.add_edge(Edge::new(node6, node2), EdgeLabel::default());

    // 对角线连接（更多交叉）
    graph.add_edge(Edge::new(node1, node5), EdgeLabel::default());
    graph.add_edge(Edge::new(node2, node6), EdgeLabel::default());
    graph.add_edge(Edge::new(node3, node1), EdgeLabel::default());
    graph.add_edge(Edge::new(node4, node2), EdgeLabel::default());

    // 子节点之间的连接
    graph.add_edge(Edge::new(sub1, sub2), EdgeLabel::default());
    graph.add_edge(Edge::new(sub2, sub3), EdgeLabel::default());
    graph.add_edge(Edge::new(sub3, sub1), EdgeLabel::default());

    // 子节点到其他父节点的连接
    graph.add_edge(Edge::new(sub1, node4), EdgeLabel::default());
    graph.add_edge(Edge::new(sub2, node5), EdgeLabel::default());
    graph.add_edge(Edge::new(sub3, node6), EdgeLabel::default());

    graph
}
