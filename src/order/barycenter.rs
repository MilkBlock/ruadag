use crate::graph::{Graph, NodeIndex};
use crate::types::{Edge, EdgeLabel, NodeLabel};

/// Barycenter result for a node
#[derive(Debug, Clone, PartialEq)]
pub struct BarycenterResult {
    pub v: NodeIndex,
    pub barycenter: Option<f64>,
    pub weight: Option<f64>,
}

impl BarycenterResult {
    pub fn new(v: NodeIndex) -> Self {
        Self {
            v,
            barycenter: None,
            weight: None,
        }
    }

    pub fn with_barycenter(v: NodeIndex, barycenter: f64, weight: f64) -> Self {
        Self {
            v,
            barycenter: Some(barycenter),
            weight: Some(weight),
        }
    }
}

/// Calculate barycenters for movable nodes
pub fn barycenter(graph: &Graph, movable: &[NodeIndex]) -> Vec<BarycenterResult> {
    movable
        .iter()
        .map(|&v| {
            let in_edges: Vec<_> = graph.in_edges(v).into_iter().collect();
            
            if in_edges.is_empty() {
                BarycenterResult::new(v)
            } else {
                let result = in_edges.iter().fold(
                    (0.0, 0.0),
                    |(sum, weight), edge| {
                        if let Some(edge_label) = graph.edge_label(edge) {
                            if let Some(node_label) = graph.node_label(edge.source) {
                                if let Some(order) = node_label.order {
                                    let edge_weight = edge_label.weight;
                                    (sum + (edge_weight * order as f64), weight + edge_weight)
                                } else {
                                    (sum, weight)
                                }
                            } else {
                                (sum, weight)
                            }
                        } else {
                            (sum, weight)
                        }
                    },
                );

                if result.1 > 0.0 {
                    BarycenterResult::with_barycenter(v, result.0 / result.1, result.1)
                } else {
                    BarycenterResult::new(v)
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Edge, EdgeLabel, NodeLabel};

    fn setup_test_graph() -> Graph {
        Graph::new()
    }

    #[test]
    fn test_assigns_undefined_barycenter_for_node_with_no_predecessors() {
        let mut g = setup_test_graph();
        let x = g.add_node(NodeLabel::default());
        
        let results = barycenter(&g, &[x]);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].v, x);
        assert!(results[0].barycenter.is_none());
        assert!(results[0].weight.is_none());
    }

    #[test]
    fn test_assigns_position_of_sole_predecessor() {
        let mut g = setup_test_graph();
        let a = g.add_node(NodeLabel {
            order: Some(2),
            ..Default::default()
        });
        let x = g.add_node(NodeLabel::default());
        
        g.add_edge(Edge::new(a, x), EdgeLabel::default());
        
        let results = barycenter(&g, &[x]);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].v, x);
        assert_eq!(results[0].barycenter, Some(2.0));
        assert_eq!(results[0].weight, Some(1.0));
    }

    #[test]
    fn test_assigns_average_of_multiple_predecessors() {
        let mut g = setup_test_graph();
        let a = g.add_node(NodeLabel {
            order: Some(2),
            ..Default::default()
        });
        let b = g.add_node(NodeLabel {
            order: Some(4),
            ..Default::default()
        });
        let x = g.add_node(NodeLabel::default());
        
        g.add_edge(Edge::new(a, x), EdgeLabel::default());
        g.add_edge(Edge::new(b, x), EdgeLabel::default());
        
        let results = barycenter(&g, &[x]);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].v, x);
        assert_eq!(results[0].barycenter, Some(3.0));
        assert_eq!(results[0].weight, Some(2.0));
    }

    #[test]
    fn test_takes_into_account_weight_of_edges() {
        let mut g = setup_test_graph();
        let a = g.add_node(NodeLabel {
            order: Some(2),
            ..Default::default()
        });
        let b = g.add_node(NodeLabel {
            order: Some(4),
            ..Default::default()
        });
        let x = g.add_node(NodeLabel::default());
        
        g.add_edge(Edge::new(a, x), EdgeLabel {
            weight: 3.0,
            ..Default::default()
        });
        g.add_edge(Edge::new(b, x), EdgeLabel::default());
        
        let results = barycenter(&g, &[x]);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].v, x);
        assert_eq!(results[0].barycenter, Some(2.5));
        assert_eq!(results[0].weight, Some(4.0));
    }

    #[test]
    fn test_calculates_barycenters_for_all_nodes_in_movable_layer() {
        let mut g = setup_test_graph();
        let a = g.add_node(NodeLabel {
            order: Some(1),
            ..Default::default()
        });
        let b = g.add_node(NodeLabel {
            order: Some(2),
            ..Default::default()
        });
        let c = g.add_node(NodeLabel {
            order: Some(4),
            ..Default::default()
        });
        let x = g.add_node(NodeLabel::default());
        let y = g.add_node(NodeLabel::default());
        let z = g.add_node(NodeLabel::default());
        
        g.add_edge(Edge::new(a, x), EdgeLabel::default());
        g.add_edge(Edge::new(b, x), EdgeLabel::default());
        g.add_edge(Edge::new(a, z), EdgeLabel {
            weight: 2.0,
            ..Default::default()
        });
        g.add_edge(Edge::new(c, z), EdgeLabel::default());
        
        let results = barycenter(&g, &[x, y, z]);
        assert_eq!(results.len(), 3);
        
        // Find results by node
        let x_result = results.iter().find(|r| r.v == x).unwrap();
        let y_result = results.iter().find(|r| r.v == y).unwrap();
        let z_result = results.iter().find(|r| r.v == z).unwrap();
        
        assert_eq!(x_result.barycenter, Some(1.5));
        assert_eq!(x_result.weight, Some(2.0));
        
        assert!(y_result.barycenter.is_none());
        assert!(y_result.weight.is_none());
        
        assert_eq!(z_result.barycenter, Some(2.0));
        assert_eq!(z_result.weight, Some(3.0));
    }
}
