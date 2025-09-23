//! # DagViz - Directed Graph Layout in Rust
//!
//! A Rust implementation of the dagre library for directed graph layout.
//! This library provides algorithms for laying out directed graphs with
//! automatic node positioning and edge routing.

pub mod counters;
pub mod graph;
pub mod layout;
pub mod order;
pub mod position;
pub mod rank;
pub mod types;
pub mod util;

#[cfg(test)]
mod zero_difference_tests;

#[cfg(test)]
mod test_node_index_stability;

#[cfg(test)]
mod debug_petgraph;

#[cfg(test)]
mod test_petgraph_behavior;

pub use graph::Graph;
pub use layout::layout;
pub use types::*;

/// 版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
