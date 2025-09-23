//! 排名算法模块

pub mod feasible_tree;
pub mod network_simplex;
pub mod util;

use crate::graph::Graph;
use crate::types::Ranker;
use crate::util::time;

/// 为图中的每个节点分配排名
///
/// 对应 JS 函数: rank() in lib/rank/index.js
pub fn rank(graph: &mut Graph) {
    let ranker = graph.config().ranker;

    match ranker {
        Ranker::NetworkSimplex => {
            time("network-simplex", || {
                network_simplex::network_simplex(graph);
            });
        }
        Ranker::FeasibleTree => {
            time("feasible-tree", || {
                feasible_tree_ranker(graph);
            });
        }
        Ranker::TightTree => {
            time("tight-tree", || {
                tight_tree_ranker(graph);
            });
        }
        Ranker::LongestPath => {
            time("longest-path", || {
                longest_path_ranker(graph);
            });
        }
        Ranker::None => {
            // 不进行排名
        }
    }
}

/// 可行树排名器
///
/// 对应 JS 函数: tightTreeRanker() in lib/rank/index.js
fn feasible_tree_ranker(graph: &mut Graph) {
    feasible_tree::feasible_tree(graph);
}

/// 最长路径排名器（快速但结果不最优）
///
/// 对应 JS 函数: longestPathRanker() in lib/rank/index.js
fn longest_path_ranker(graph: &mut Graph) {
    util::longest_path(graph);
}

/// 紧树排名器
///
/// 对应 JS 函数: tightTreeRanker() in lib/rank/index.js
fn tight_tree_ranker(graph: &mut Graph) {
    util::longest_path(graph);
    feasible_tree::feasible_tree(graph);
}
