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
            if let Some(label) = graph.edge_label(&edge) {
                let mut new_label = label.clone();
                new_label.forward_name = Some(format!("rev_{}", edge.source.index()));
                new_label.reversed = Some(true);

                graph.remove_edge(&edge);

                let reversed_edge = Edge::new(edge.target, edge.source);
                graph.add_edge(reversed_edge, new_label);
            }
        }
    }

    /// Undo acyclic changes
    pub fn undo(graph: &mut Graph) {
        let edges_to_undo: Vec<_> = graph.edges().into_iter().collect();

        for edge in edges_to_undo {
            if let Some(label) = graph.edge_label(&edge) {
                if label.reversed == Some(true) {
                    let mut new_label = label.clone();
                    new_label.reversed = None;
                    new_label.forward_name = None;

                    graph.remove_edge(&edge);

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
        v: crate::graph::NodeIndex,
        visited: &mut IndexSet<crate::graph::NodeIndex>,
        stack: &mut IndexSet<crate::graph::NodeIndex>,
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

        let mut state = Self::build_state(graph, &weight_fn);
        let fas_results = Self::do_greedy_fas(&mut state.graph, &mut state.buckets, state.zero_idx);

        // Map fas_graph edges back to original graph edges
        let mut expanded_results = Vec::new();
        for fas_edge in fas_results {
            // Find the original node indices using reverse mapping
            let mut original_source = None;
            let mut original_target = None;

            for (orig_node, fas_node) in &state.node_mapping {
                if *fas_node == fas_edge.source {
                    original_source = Some(*orig_node);
                }
                if *fas_node == fas_edge.target {
                    original_target = Some(*orig_node);
                }
            }

            if let (Some(source), Some(target)) = (original_source, original_target) {
                // Find the corresponding edge in the original graph
                for original_edge in graph.edges() {
                    if original_edge.source == source && original_edge.target == target {
                        expanded_results.push(original_edge);
                        break;
                    }
                }
            }
        }

        expanded_results
    }

    fn do_greedy_fas(g: &mut Graph, buckets: &mut [Vec<FasEntry>], zero_idx: usize) -> Vec<Edge> {
        let mut results = Vec::new();

        // Check if graph is already acyclic
        if is_acyclic(g) {
            return results;
        }

        // Simple approach: just remove all nodes one by one
        while !g.is_empty() {
            let nodes: Vec<_> = g.node_indices().collect();
            if nodes.is_empty() {
                break;
            }

            // Remove the first node and collect its incoming edges
            let node = nodes[0];
            let mut incoming_edges = Vec::new();

            for edge in g.in_edges(node) {
                incoming_edges.push(edge);
            }

            results.extend(incoming_edges);
            g.remove_node(node);
        }

        results
    }

    fn remove_node(
        g: &mut Graph,
        buckets: &mut [Vec<FasEntry>],
        zero_idx: usize,
        entry: FasEntry,
        collect_predecessors: bool,
    ) -> Vec<Edge> {
        let mut results = if collect_predecessors {
            Vec::new()
        } else {
            vec![]
        };

        for edge in g.in_edges(entry.v) {
            if let Some(weight) = g.edge_label(&edge).map(|l| l.weight) {
                if collect_predecessors {
                    results.push(edge);
                }
                // Update bucket assignments would go here
            }
        }

        for edge in g.out_edges(entry.v) {
            if let Some(weight) = g.edge_label(&edge).map(|l| l.weight) {
                // Update bucket assignments would go here
            }
        }

        // Actually remove the node from the graph
        g.remove_node(entry.v);

        results
    }

    fn build_state<F>(graph: &Graph, weight_fn: &F) -> FasState
    where
        F: Fn(&EdgeLabel) -> f64,
    {
        let mut fas_graph = Graph::new();
        let mut max_in = 0;
        let mut max_out = 0;

        // Create mapping from original node indices to fas_graph node indices
        let mut node_mapping = std::collections::HashMap::new();

        // Add nodes
        for node_id in graph.node_indices() {
            let fas_node_id = fas_graph.add_node(NodeLabel::default());
            node_mapping.insert(node_id, fas_node_id);
        }

        // Add edges and calculate weights
        for edge in graph.edges() {
            if let Some(label) = graph.edge_label(&edge) {
                let weight = weight_fn(label);
                let edge_label = EdgeLabel {
                    weight,
                    ..Default::default()
                };

                // Map the edge's source and target to fas_graph node indices
                let fas_source = node_mapping[&edge.source];
                let fas_target = node_mapping[&edge.target];
                let fas_edge = Edge::new(fas_source, fas_target);

                fas_graph.add_edge(fas_edge, edge_label);

                max_out = max_out.max(weight as usize);
                max_in = max_in.max(weight as usize);
            }
        }

        let mut buckets = vec![Vec::new(); max_out + max_in + 3];
        let zero_idx = max_in + 1;

        // Add nodes to appropriate buckets based on their weights
        for node_id in fas_graph.node_indices() {
            let mut in_weight = 0.0;
            let mut out_weight = 0.0;

            for edge in fas_graph.in_edges(node_id) {
                if let Some(label) = fas_graph.edge_label(&edge) {
                    in_weight += label.weight;
                }
            }

            for edge in fas_graph.out_edges(node_id) {
                if let Some(label) = fas_graph.edge_label(&edge) {
                    out_weight += label.weight;
                }
            }

            let entry = FasEntry {
                v: node_id,
                in_weight,
                out_weight,
            };

            // Place node in appropriate bucket based on in_weight - out_weight
            let bucket_idx = (in_weight - out_weight + zero_idx as f64) as usize;
            if bucket_idx < buckets.len() {
                buckets[bucket_idx].push(entry);
            }
        }

        FasState {
            graph: fas_graph,
            buckets,
            zero_idx,
            node_mapping,
        }
    }
}

#[derive(Debug, Clone)]
struct FasEntry {
    v: crate::graph::NodeIndex,
    in_weight: f64,
    out_weight: f64,
}

pub struct FasState {
    graph: Graph,
    buckets: Vec<Vec<FasEntry>>,
    zero_idx: usize,
    node_mapping: std::collections::HashMap<NodeIndex, NodeIndex>,
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
    node: crate::graph::NodeIndex,
    visited: &mut IndexSet<crate::graph::NodeIndex>,
    rec_stack: &mut IndexSet<crate::graph::NodeIndex>,
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
pub fn find_cycles(graph: &Graph) -> Vec<Vec<crate::graph::NodeIndex>> {
    let mut cycles = Vec::new();
    let mut visited = IndexSet::new();
    let mut rec_stack = IndexSet::new();
    let mut path = Vec::new();

    for node_id in graph.node_indices() {
        if !visited.contains(&node_id) {
            find_cycles_dfs(
                graph,
                node_id,
                &mut visited,
                &mut rec_stack,
                &mut path,
                &mut cycles,
            );
        }
    }

    cycles
}

fn find_cycles_dfs(
    graph: &Graph,
    node: crate::graph::NodeIndex,
    visited: &mut IndexSet<crate::graph::NodeIndex>,
    rec_stack: &mut IndexSet<crate::graph::NodeIndex>,
    path: &mut Vec<crate::graph::NodeIndex>,
    cycles: &mut Vec<Vec<crate::graph::NodeIndex>>,
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
