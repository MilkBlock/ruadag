use dagviz::graph::Graph;
use dagviz::position::bk::BrandesKoepf;
use dagviz::types::{Edge, EdgeLabel, NodeLabel};

fn main() {
    println!("=== Subgraph Alignment 测试 ===");

    // 测试简单子图
    test_simple_subgraph();

    // 测试嵌套子图
    test_nested_subgraph();
}

fn test_simple_subgraph() {
    println!("\n=== 简单子图测试 ===");

    let mut graph = Graph::new();

    // 设置图配置
    graph.set_config(dagviz::types::GraphConfig {
        node_sep: 50.0,
        edge_sep: 10.0,
        rank_sep: 50.0,
        ..Default::default()
    });

    // 创建根节点
    let root = graph.add_node(NodeLabel {
        label: Some("root".to_string()),
        width: 100.0,
        height: 50.0,
        rank: Some(0),
        order: Some(0),
        ..Default::default()
    });

    // 创建子图A
    let subgraph_a = graph.add_node(NodeLabel {
        label: Some("subgraph_a".to_string()),
        width: 80.0,
        height: 40.0,
        parent: Some(root),
        rank: Some(0),
        order: Some(1),
        ..Default::default()
    });

    // 创建子图B
    let subgraph_b = graph.add_node(NodeLabel {
        label: Some("subgraph_b".to_string()),
        width: 80.0,
        height: 40.0,
        parent: Some(root),
        rank: Some(0),
        order: Some(2),
        ..Default::default()
    });

    // 创建子图A中的节点
    let a1 = graph.add_node(NodeLabel {
        label: Some("a1".to_string()),
        width: 30.0,
        height: 20.0,
        parent: Some(subgraph_a),
        rank: Some(1),
        order: Some(0),
        ..Default::default()
    });

    let a2 = graph.add_node(NodeLabel {
        label: Some("a2".to_string()),
        width: 30.0,
        height: 20.0,
        parent: Some(subgraph_a),
        rank: Some(1),
        order: Some(1),
        ..Default::default()
    });

    // 创建子图B中的节点
    let b1 = graph.add_node(NodeLabel {
        label: Some("b1".to_string()),
        width: 30.0,
        height: 20.0,
        parent: Some(subgraph_b),
        rank: Some(1),
        order: Some(0),
        ..Default::default()
    });

    let b2 = graph.add_node(NodeLabel {
        label: Some("b2".to_string()),
        width: 30.0,
        height: 20.0,
        parent: Some(subgraph_b),
        rank: Some(1),
        order: Some(1),
        ..Default::default()
    });

    // 添加边
    graph.add_edge(Edge::new(a1, a2), EdgeLabel::default());
    graph.add_edge(Edge::new(b1, b2), EdgeLabel::default());
    graph.add_edge(Edge::new(a1, b1), EdgeLabel::default());

    // 构建层级矩阵
    let layering = vec![vec![root, subgraph_a, subgraph_b], vec![b1, b2]];

    println!("层级矩阵: {:?}", layering);

    // 创建 BK 实例
    let bk = BrandesKoepf::new(graph.clone());

    // 测试 verticalAlignment
    println!("\n--- verticalAlignment 测试 ---");

    let up_align = bk.vertical_alignment(&layering, |g, v| g.predecessors(v).collect());
    println!("向上对齐 (Up):");
    println!("  root: {:?}", up_align.root);
    println!("  align: {:?}", up_align.align);

    let down_align = bk.vertical_alignment(&layering, |g, v| g.successors(v).collect());
    println!("向下对齐 (Down):");
    println!("  root: {:?}", down_align.root);
    println!("  align: {:?}", down_align.align);

    // 测试 horizontalCompaction
    println!("\n--- horizontalCompaction 测试 ---");

    let up_compaction = bk.horizontal_compaction(&layering, &up_align, false);
    println!("向上压缩 (Up):");
    println!("  xs: {:?}", up_compaction);

    let down_compaction = bk.horizontal_compaction(&layering, &down_align, true);
    println!("向下压缩 (Down):");
    println!("  xs: {:?}", down_compaction);

    // 验证结果
    println!("\n--- 验证结果 ---");

    // 验证向上对齐结果
    println!("验证向上对齐...");
    println!("实际结果: {:?}", up_align.root);
    assert_eq!(up_align.root.get(&root), Some(&root));
    assert_eq!(up_align.root.get(&subgraph_a), Some(&subgraph_a));
    assert_eq!(up_align.root.get(&subgraph_b), Some(&subgraph_b));
    // 注意：a1 和 a2 不在层级矩阵中，所以不会出现在对齐结果中
    // 修复后：b1 和 b2 相互对齐（与 JavaScript 版本一致）
    assert_eq!(up_align.root.get(&b1), Some(&b1));
    assert_eq!(up_align.root.get(&b2), Some(&b1));
    println!("✅ 向上对齐 root 验证通过");

    // 验证向下对齐结果
    println!("验证向下对齐...");
    println!("实际结果: {:?}", down_align.root);
    assert_eq!(down_align.root.get(&root), Some(&root));
    assert_eq!(down_align.root.get(&subgraph_a), Some(&subgraph_a));
    assert_eq!(down_align.root.get(&subgraph_b), Some(&subgraph_b));
    // 注意：a1 和 a2 不在层级矩阵中，所以不会出现在对齐结果中
    assert_eq!(down_align.root.get(&b1), Some(&b2)); // b1 对齐到 b2
    assert_eq!(down_align.root.get(&b2), Some(&b2));
    println!("✅ 向下对齐 root 验证通过");

    // 验证水平压缩结果
    println!("验证水平压缩...");
    assert_eq!(up_compaction.get(&root), Some(&0.0));
    assert_eq!(up_compaction.get(&subgraph_a), Some(&140.0));
    assert_eq!(up_compaction.get(&subgraph_b), Some(&270.0));
    // 注意：a1 和 a2 不在层级矩阵中，所以不会出现在压缩结果中
    // 修复后：b1 和 b2 相互对齐，所以坐标相同
    assert_eq!(up_compaction.get(&b1), Some(&0.0));
    assert_eq!(up_compaction.get(&b2), Some(&0.0));
    println!("✅ 向上压缩验证通过");

    assert_eq!(down_compaction.get(&root), Some(&0.0));
    assert_eq!(down_compaction.get(&subgraph_a), Some(&140.0));
    assert_eq!(down_compaction.get(&subgraph_b), Some(&270.0));
    // 注意：a1 和 a2 不在层级矩阵中，所以不会出现在压缩结果中
    assert_eq!(down_compaction.get(&b1), Some(&0.0));
    assert_eq!(down_compaction.get(&b2), Some(&0.0));
    println!("✅ 向下压缩验证通过");

    println!("🎉 简单子图测试全部通过！");
}

fn test_nested_subgraph() {
    println!("\n=== 嵌套子图测试 ===");

    let mut graph = Graph::new();

    // 设置图配置
    graph.set_config(dagviz::types::GraphConfig {
        node_sep: 50.0,
        edge_sep: 10.0,
        rank_sep: 50.0,
        ..Default::default()
    });

    // 创建根节点
    let main_system = graph.add_node(NodeLabel {
        label: Some("main_system".to_string()),
        width: 200.0,
        height: 100.0,
        rank: Some(0),
        order: Some(0),
        ..Default::default()
    });

    // 创建第一层子图：前端模块
    let frontend = graph.add_node(NodeLabel {
        label: Some("frontend".to_string()),
        width: 150.0,
        height: 60.0,
        parent: Some(main_system),
        rank: Some(0),
        order: Some(1),
        ..Default::default()
    });

    // 创建第一层子图：后端模块
    let backend = graph.add_node(NodeLabel {
        label: Some("backend".to_string()),
        width: 150.0,
        height: 60.0,
        parent: Some(main_system),
        rank: Some(0),
        order: Some(2),
        ..Default::default()
    });

    // 创建前端子模块：UI组件
    let ui_components = graph.add_node(NodeLabel {
        label: Some("ui_components".to_string()),
        width: 100.0,
        height: 40.0,
        parent: Some(frontend),
        rank: Some(1),
        order: Some(0),
        ..Default::default()
    });

    // 创建前端子模块：状态管理
    let state_mgmt = graph.add_node(NodeLabel {
        label: Some("state_mgmt".to_string()),
        width: 100.0,
        height: 40.0,
        parent: Some(frontend),
        rank: Some(1),
        order: Some(1),
        ..Default::default()
    });

    // 创建后端子模块：API层
    let api_layer = graph.add_node(NodeLabel {
        label: Some("api_layer".to_string()),
        width: 100.0,
        height: 40.0,
        parent: Some(backend),
        rank: Some(1),
        order: Some(0),
        ..Default::default()
    });

    // 创建后端子模块：数据库层
    let db_layer = graph.add_node(NodeLabel {
        label: Some("db_layer".to_string()),
        width: 100.0,
        height: 40.0,
        parent: Some(backend),
        rank: Some(1),
        order: Some(1),
        ..Default::default()
    });

    // 创建具体的组件节点
    let button = graph.add_node(NodeLabel {
        label: Some("button".to_string()),
        width: 20.0,
        height: 15.0,
        parent: Some(ui_components),
        rank: Some(2),
        order: Some(0),
        ..Default::default()
    });

    let input = graph.add_node(NodeLabel {
        label: Some("input".to_string()),
        width: 20.0,
        height: 15.0,
        parent: Some(ui_components),
        rank: Some(2),
        order: Some(1),
        ..Default::default()
    });

    let redux = graph.add_node(NodeLabel {
        label: Some("redux".to_string()),
        width: 20.0,
        height: 15.0,
        parent: Some(state_mgmt),
        rank: Some(2),
        order: Some(0),
        ..Default::default()
    });

    let rest_api = graph.add_node(NodeLabel {
        label: Some("rest_api".to_string()),
        width: 20.0,
        height: 15.0,
        parent: Some(api_layer),
        rank: Some(2),
        order: Some(0),
        ..Default::default()
    });

    let postgres = graph.add_node(NodeLabel {
        label: Some("postgres".to_string()),
        width: 20.0,
        height: 15.0,
        parent: Some(db_layer),
        rank: Some(2),
        order: Some(0),
        ..Default::default()
    });

    // 添加连接边
    graph.add_edge(Edge::new(button, input), EdgeLabel::default());
    graph.add_edge(Edge::new(redux, rest_api), EdgeLabel::default());
    graph.add_edge(Edge::new(rest_api, postgres), EdgeLabel::default());
    graph.add_edge(Edge::new(ui_components, state_mgmt), EdgeLabel::default());
    graph.add_edge(Edge::new(api_layer, db_layer), EdgeLabel::default());

    // 构建层级矩阵
    let layering = vec![
        vec![main_system, frontend, backend],
        vec![api_layer, db_layer],
        vec![postgres, input],
    ];

    println!("层级矩阵: {:?}", layering);

    // 创建 BK 实例
    let bk = BrandesKoepf::new(graph.clone());

    // 测试 verticalAlignment
    println!("\n--- verticalAlignment 测试 ---");

    let up_align = bk.vertical_alignment(&layering, |g, v| g.predecessors(v).collect());
    println!("向上对齐 (Up):");
    println!("  root: {:?}", up_align.root);
    println!("  align: {:?}", up_align.align);

    let down_align = bk.vertical_alignment(&layering, |g, v| g.successors(v).collect());
    println!("向下对齐 (Down):");
    println!("  root: {:?}", down_align.root);
    println!("  align: {:?}", down_align.align);

    // 测试 horizontalCompaction
    println!("\n--- horizontalCompaction 测试 ---");

    let up_compaction = bk.horizontal_compaction(&layering, &up_align, false);
    println!("向上压缩 (Up):");
    println!("  xs: {:?}", up_compaction);

    let down_compaction = bk.horizontal_compaction(&layering, &down_align, true);
    println!("向下压缩 (Down):");
    println!("  xs: {:?}", down_compaction);

    // 验证结果
    println!("\n--- 验证结果 ---");

    // 验证向上对齐结果
    println!("验证向上对齐...");
    println!("实际结果: {:?}", up_align.root);
    assert_eq!(up_align.root.get(&main_system), Some(&main_system));
    assert_eq!(up_align.root.get(&frontend), Some(&frontend));
    assert_eq!(up_align.root.get(&backend), Some(&backend));
    assert_eq!(up_align.root.get(&api_layer), Some(&api_layer));
    assert_eq!(up_align.root.get(&db_layer), Some(&api_layer)); // db_layer 对齐到 api_layer
    assert_eq!(up_align.root.get(&postgres), Some(&postgres)); // postgres 对齐到自己（修复后行为）
    assert_eq!(up_align.root.get(&input), Some(&input));
    println!("✅ 向上对齐 root 验证通过");

    // 验证向下对齐结果
    println!("验证向下对齐...");
    println!("实际结果: {:?}", down_align.root);
    assert_eq!(down_align.root.get(&main_system), Some(&main_system));
    assert_eq!(down_align.root.get(&frontend), Some(&frontend));
    assert_eq!(down_align.root.get(&backend), Some(&backend));
    assert_eq!(down_align.root.get(&api_layer), Some(&db_layer)); // api_layer 对齐到 db_layer
    assert_eq!(down_align.root.get(&db_layer), Some(&db_layer));
    assert_eq!(down_align.root.get(&postgres), Some(&postgres));
    assert_eq!(down_align.root.get(&input), Some(&input));
    println!("✅ 向下对齐 root 验证通过");

    // 验证水平压缩结果
    println!("验证水平压缩...");
    assert_eq!(up_compaction.get(&main_system), Some(&0.0));
    assert_eq!(up_compaction.get(&frontend), Some(&225.0));
    assert_eq!(up_compaction.get(&backend), Some(&425.0));
    assert_eq!(up_compaction.get(&api_layer), Some(&0.0));
    assert_eq!(up_compaction.get(&db_layer), Some(&0.0));
    assert_eq!(up_compaction.get(&postgres), Some(&0.0));
    assert_eq!(up_compaction.get(&input), Some(&70.0));
    println!("✅ 向上压缩验证通过");

    assert_eq!(down_compaction.get(&main_system), Some(&0.0));
    assert_eq!(down_compaction.get(&frontend), Some(&225.0));
    assert_eq!(down_compaction.get(&backend), Some(&425.0));
    assert_eq!(down_compaction.get(&api_layer), Some(&0.0));
    assert_eq!(down_compaction.get(&db_layer), Some(&0.0));
    assert_eq!(down_compaction.get(&postgres), Some(&0.0));
    assert_eq!(down_compaction.get(&input), Some(&70.0));
    println!("✅ 向下压缩验证通过");

    println!("🎉 嵌套子图测试全部通过！");
}
