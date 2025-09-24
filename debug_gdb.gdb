# GDB调试脚本
set pagination off
set print pretty on

# 设置断点
break dagviz::position::position
break dagviz::position::bk::BrandesKoepf::run

# 运行程序
run

# 当到达position函数时
commands 1
  echo === 进入position函数 ===\n
  print "图节点数: %d", graph.node_count()
  continue
end

# 当到达BrandesKoepf::run时
commands 2
  echo === 进入BrandesKoepf::run ===\n
  print "输入图节点数: %d", self.graph.node_count()
  continue
end

# 继续执行
continue

# 显示最终结果
echo === 最终结果 ===\n
print "程序执行完成"
quit
