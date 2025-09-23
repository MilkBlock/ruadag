use crate::graph::{Graph, NodeIndex};
use crate::types::{Edge, EdgeLabel, NodeLabel};
use indexmap::IndexSet;

/// Acyclic module for making graphs acyclic
pub struct Acyclic;

impl Acyclic {
    /// Run acyclic algorithm on the graph
    pub fn run(graph: &mut Graph) {
        let acyclicer = graph.config().acyclicer.clone();
        let fas = match acyclicer.as_str() {
            "greedy" => Self::greedy_fas(graph, |e| e.weight),
            _ => Self::dfs_fas(graph),
        };

        for edge in fas {
            let label = graph.edge_label(&edge).cloned();
            if let Some(label) = label {
                graph.remove_edge(&edge);
                let mut new_label = label;
                new_label.forward_name = Some(format!("rev_{}", edge.source.index()));
                new_label.reversed = Some(true);
                
                let reversed_edge = Edge::new(edge.target, edge.source);
                graph.add_edge(reversed_edge, new_label);
            }
        }
    }

    /// Undo acyclic changes
    pub fn undo(graph: &mut Graph) {
        let edges_to_undo: Vec<_> = graph.edges().into_iter().collect();
        
        for edge in edges_to_undo {
            let label = graph.edge_label(&edge).cloned();
            if let Some(label) = label {
                if label.reversed == Some(true) {
                    graph.remove_edge(&edge);
                    
                    let mut new_label = label;
                    new_label.reversed = None;
                    new_label.forward_name = None;
                    
                    let forward_edge = Edge::new(edge.target, edge.source);
                    graph.add_edge(forward_edge, new_label);
                }
            }
        }
    }

    /// DFS-based feedback arc set algorithm
    fn dfs_fas(graph: &Graph) -> Vec<Edge> {
        let mut fas = Vec::new();
        let mut visited = IndexSet::new();
        let mut stack = IndexSet::new();

        for node_id in graph.node_indices() {
            if !visited.contains(&node_id) {
                Self::dfs_visit(graph, node_id, &mut visited, &mut stack, &mut fas);
            }
        }

        fas
    }

    fn dfs_visit(
        graph: &Graph,
        v: NodeIndex,
        visited: &mut IndexSet<NodeIndex>,
        stack: &mut IndexSet<NodeIndex>,
        fas: &mut Vec<Edge>,
    ) {
        if visited.contains(&v) {
            return;
        }

        visited.insert(v);
        stack.insert(v);

        for edge in graph.out_edges(v) {
            if stack.contains(&edge.target) {
                fas.push(edge);
            } else {
                Self::dfs_visit(graph, edge.target, visited, stack, fas);
            }
        }

        stack.remove(&v);
    }

    /// Greedy feedback arc set algorithm
    fn greedy_fas<F>(graph: &Graph, weight_fn: F) -> Vec<Edge>
    where
        F: Fn(&EdgeLabel) -> f64,
    {
        if graph.node_count() <= 1 {
            return Vec::new();
        }

        let state = Self::build_state(graph, &weight_fn);
        let results = Self::do_greedy_fas(&state.graph, &state.buckets, state.zero_idx);

        // Expand multi-edges
        let mut expanded_results = Vec::new();
        for edge in results {
            for multi_edge in graph.out_edges(edge.source) {
                if multi_edge.target == edge.target {
                    expanded_results.push(multi_edge);
                }
            }
        }

        expanded_results
    }

    fn do_greedy_fas(
        g: &Graph,
        buckets: &[Vec<FasEntry>],
        zero_idx: usize,
    ) -> Vec<Edge> {
        let mut results = Vec::new();
        let mut sources = buckets[buckets.len() - 1].clone();
        let mut sinks = buckets[0].clone();

        while !g.is_empty() {
            while let Some(entry) = sinks.pop() {
                Self::remove_node(g, buckets, zero_idx, entry, false);
            }
            while let Some(entry) = sources.pop() {
                Self::remove_node(g, buckets, zero_idx, entry, false);
            }
            
            if !g.is_empty() {
                for i in (1..buckets.len() - 1).rev() {
                    if let Some(entry) = buckets[i].last().cloned() {
                        results.extend(Self::remove_node(g, buckets, zero_idx, entry, true));
                        break;
                    }
                }
            }
        }

        results
    }

    fn remove_node(
        g: &Graph,
        _buckets: &[Vec<FasEntry>],
        _zero_idx: usize,
        entry: FasEntry,
        collect_predecessors: bool,
    ) -> Vec<Edge> {
        let mut results = if collect_predecessors { Vec::new() } else { vec![] };

        for edge in g.in_edges(entry.v) {
            if let Some(_weight) = g.edge_label(&edge).map(|l| l.weight) {
                if collect_predecessors {
                    results.push(edge);
                }
                // Update bucket assignments would go here
            }
        }

        for edge in g.out_edges(entry.v) {
            if let Some(_weight) = g.edge_label(&edge).map(|l| l.weight) {
                // Update bucket assignments would go here
            }
        }

        results
    }

    fn build_state<F>(graph: &Graph, weight_fn: &F) -> FasState
    where
        F: Fn(&EdgeLabel) -> f64,
    {
        let mut fas_graph = Graph::new();
        let mut max_in = 0;
        let mut max_out = 0;

        // Add nodes
        for _node_id in graph.node_indices() {
            fas_graph.add_node(NodeLabel::default());
        }

        // Add edges and calculate weights
        for edge in graph.edges() {
            if let Some(label) = graph.edge_label(&edge) {
                let weight = weight_fn(label);
                let edge_label = EdgeLabel {
                    weight,
                    ..Default::default()
                };
                fas_graph.add_edge(edge, edge_label);
                
                max_out = max_out.max(weight as usize);
                max_in = max_in.max(weight as usize);
            }
        }

        let buckets = vec![Vec::new(); max_out + max_in + 3];
        let zero_idx = max_in + 1;

        FasState {
            graph: fas_graph,
            buckets,
            zero_idx,
        }
    }
}

#[derive(Debug, Clone)]
struct FasEntry {
    v: NodeIndex,
    in_weight: f64,
    out_weight: f64,
}

struct FasState {
    graph: Graph,
    buckets: Vec<Vec<FasEntry>>,
    zero_idx: usize,
}

/// Check if graph is acyclic
pub fn is_acyclic(graph: &Graph) -> bool {
    let mut visited = IndexSet::new();
    let mut rec_stack = IndexSet::new();

    for node_id in graph.node_indices() {
        if !visited.contains(&node_id) {
            if has_cycle(graph, node_id, &mut visited, &mut rec_stack) {
                return false;
            }
        }
    }

    true
}

fn has_cycle(
    graph: &Graph,
    node: NodeIndex,
    visited: &mut IndexSet<NodeIndex>,
    rec_stack: &mut IndexSet<NodeIndex>,
) -> bool {
    visited.insert(node);
    rec_stack.insert(node);

    for edge in graph.out_edges(node) {
        if !visited.contains(&edge.target) {
            if has_cycle(graph, edge.target, visited, rec_stack) {
                return true;
            }
        } else if rec_stack.contains(&edge.target) {
            return true;
        }
    }

    rec_stack.remove(&node);
    false
}

/// Find cycles in the graph
pub fn find_cycles(graph: &Graph) -> Vec<Vec<NodeIndex>> {
    let mut cycles = Vec::new();
    let mut visited = IndexSet::new();
    let mut rec_stack = IndexSet::new();
    let mut path = Vec::new();

    for node_id in graph.node_indices() {
        if !visited.contains(&node_id) {
            find_cycles_dfs(graph, node_id, &mut visited, &mut rec_stack, &mut path, &mut cycles);
        }
    }

    cycles
}

fn find_cycles_dfs(
    graph: &Graph,
    node: NodeIndex,
    visited: &mut IndexSet<NodeIndex>,
    rec_stack: &mut IndexSet<NodeIndex>,
    path: &mut Vec<NodeIndex>,
    cycles: &mut Vec<Vec<NodeIndex>>,
) {
    if rec_stack.contains(&node) {
        // Found a cycle
        if let Some(start_idx) = path.iter().position(|&n| n == node) {
            cycles.push(path[start_idx..].to_vec());
        }
        return;
    }

    if visited.contains(&node) {
        return;
    }

    visited.insert(node);
    rec_stack.insert(node);
    path.push(node);

    for edge in graph.out_edges(node) {
        find_cycles_dfs(graph, edge.target, visited, rec_stack, path, cycles);
    }

    rec_stack.remove(&node);
    path.pop();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Edge, EdgeLabel, NodeLabel};

    fn setup_test_graph() -> Graph {
        let mut graph = Graph::new();
        // Add some test nodes
        let _a = graph.add_node(NodeLabel::default());
        let _b = graph.add_node(NodeLabel::default());
        let _c = graph.add_node(NodeLabel::default());
        let _d = graph.add_node(NodeLabel::default());
        graph
    }

    #[test]
    fn test_is_acyclic_empty() {
        let graph = Graph::new();
        assert!(is_acyclic(&graph));
    }

    #[test]
    fn test_is_acyclic_simple() {
        let mut graph = setup_test_graph();
        let nodes: Vec<_> = graph.node_indices().collect();
        
        if nodes.len() >= 2 {
            let edge = Edge::new(nodes[0], nodes[1]);
            graph.add_edge(edge, EdgeLabel::default());
            assert!(is_acyclic(&graph));
        }
    }

    #[test]
    fn test_find_cycles_empty() {
        let graph = Graph::new();
        let cycles = find_cycles(&graph);
        assert!(cycles.is_empty());
    }

    #[test]
    fn test_find_cycles_simple() {
        let mut graph = setup_test_graph();
        let nodes: Vec<_> = graph.node_indices().collect();
        
        if nodes.len() >= 3 {
            // Create a cycle: a -> b -> c -> a
            let edge1 = Edge::new(nodes[0], nodes[1]);
            let edge2 = Edge::new(nodes[1], nodes[2]);
            let edge3 = Edge::new(nodes[2], nodes[0]);
            
            graph.add_edge(edge1, EdgeLabel::default());
            graph.add_edge(edge2, EdgeLabel::default());
            graph.add_edge(edge3, EdgeLabel::default());
            
            let cycles = find_cycles(&graph);
            assert!(!cycles.is_empty());
        }
    }
}
