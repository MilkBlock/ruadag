//! # DagViz - Directed Graph Layout in Rust
//!
//! A Rust implementation of the dagre library for directed graph layout.
//! This library provides algorithms for laying out directed graphs with
//! automatic node positioning and edge routing.

pub mod acyclic;
pub mod counters;
pub mod graph;
pub mod layout;
pub mod order;
pub mod position;
pub mod rank;
pub mod types;
pub mod util;

pub use graph::Graph;
pub use layout::layout;
pub use types::*;

/// 版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
