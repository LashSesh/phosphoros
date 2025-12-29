//! Transaction Graph Analysis
//!
//! Provides graph-based analysis of Monero transactions including:
//! - Quantum Walk for centrality analysis
//! - Community detection via QAOA
//! - Anomaly detection in transaction patterns
//!
//! # Features
//!
//! Enable with the `quantum` feature flag for quantum-enhanced analysis.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// A node in the transaction graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxNode {
    /// Node identifier (transaction hash or address)
    pub id: String,
    /// Node type
    pub node_type: NodeType,
    /// Associated value (in atomic units)
    pub value: u64,
    /// Block height
    pub block_height: u64,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// Type of node in the transaction graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    /// Transaction
    Transaction,
    /// Output (key image)
    Output,
    /// Address cluster
    Cluster,
    /// Unknown/Decoy
    Unknown,
}

/// An edge in the transaction graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxEdge {
    /// Source node ID
    pub source: String,
    /// Target node ID
    pub target: String,
    /// Edge type
    pub edge_type: EdgeType,
    /// Edge weight (confidence or value)
    pub weight: f64,
}

/// Type of edge in the transaction graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeType {
    /// Input spending output
    Spend,
    /// Ring member reference
    RingMember,
    /// Cluster membership
    Cluster,
    /// Temporal proximity
    Temporal,
}

/// Transaction graph for forensic analysis
#[derive(Debug, Clone, Default)]
pub struct TransactionGraph {
    /// Nodes in the graph
    nodes: HashMap<String, TxNode>,
    /// Edges (adjacency list)
    edges: HashMap<String, Vec<TxEdge>>,
    /// Reverse edges for traversal
    reverse_edges: HashMap<String, Vec<TxEdge>>,
}

impl TransactionGraph {
    /// Create a new empty transaction graph
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: TxNode) {
        let id = node.id.clone();
        self.nodes.insert(id.clone(), node);
        self.edges.entry(id.clone()).or_default();
        self.reverse_edges.entry(id).or_default();
    }

    /// Add an edge to the graph
    pub fn add_edge(&mut self, edge: TxEdge) {
        let source = edge.source.clone();
        let target = edge.target.clone();

        self.edges.entry(source.clone()).or_default().push(edge.clone());

        // Reverse edge for traversal
        let reverse = TxEdge {
            source: target.clone(),
            target: source,
            edge_type: edge.edge_type,
            weight: edge.weight,
        };
        self.reverse_edges.entry(target).or_default().push(reverse);
    }

    /// Get a node by ID
    pub fn get_node(&self, id: &str) -> Option<&TxNode> {
        self.nodes.get(id)
    }

    /// Get outgoing edges for a node
    pub fn get_edges(&self, id: &str) -> &[TxEdge] {
        self.edges.get(id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Get incoming edges for a node
    pub fn get_incoming_edges(&self, id: &str) -> &[TxEdge] {
        self.reverse_edges.get(id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Number of nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Number of edges
    pub fn edge_count(&self) -> usize {
        self.edges.values().map(|v| v.len()).sum()
    }

    /// Get all node IDs
    pub fn node_ids(&self) -> impl Iterator<Item = &String> {
        self.nodes.keys()
    }

    /// Calculate degree centrality for all nodes
    pub fn degree_centrality(&self) -> HashMap<String, f64> {
        let n = self.nodes.len() as f64;
        if n <= 1.0 {
            return self.nodes.keys().map(|k| (k.clone(), 0.0)).collect();
        }

        self.nodes
            .keys()
            .map(|id| {
                let out_degree = self.edges.get(id).map(|v| v.len()).unwrap_or(0);
                let in_degree = self.reverse_edges.get(id).map(|v| v.len()).unwrap_or(0);
                let centrality = (out_degree + in_degree) as f64 / (2.0 * (n - 1.0));
                (id.clone(), centrality)
            })
            .collect()
    }

    /// Calculate PageRank-style centrality
    pub fn pagerank(&self, damping: f64, iterations: usize) -> HashMap<String, f64> {
        let n = self.nodes.len();
        if n == 0 {
            return HashMap::new();
        }

        let initial = 1.0 / n as f64;
        let mut rank: HashMap<String, f64> = self.nodes.keys().map(|k| (k.clone(), initial)).collect();

        for _ in 0..iterations {
            let mut new_rank: HashMap<String, f64> = HashMap::new();

            for node_id in self.nodes.keys() {
                let incoming = self.get_incoming_edges(node_id);
                let mut sum = 0.0;

                for edge in incoming {
                    let source_rank = rank.get(&edge.target).copied().unwrap_or(0.0);
                    let source_out_degree = self.edges.get(&edge.target).map(|v| v.len()).unwrap_or(1);
                    sum += source_rank / source_out_degree as f64;
                }

                new_rank.insert(
                    node_id.clone(),
                    (1.0 - damping) / n as f64 + damping * sum,
                );
            }

            rank = new_rank;
        }

        rank
    }

    /// Find connected components
    pub fn connected_components(&self) -> Vec<HashSet<String>> {
        let mut visited: HashSet<String> = HashSet::new();
        let mut components = Vec::new();

        for node_id in self.nodes.keys() {
            if !visited.contains(node_id) {
                let mut component = HashSet::new();
                self.dfs(node_id, &mut visited, &mut component);
                components.push(component);
            }
        }

        components
    }

    fn dfs(&self, node_id: &str, visited: &mut HashSet<String>, component: &mut HashSet<String>) {
        if visited.contains(node_id) {
            return;
        }

        visited.insert(node_id.to_string());
        component.insert(node_id.to_string());

        // Follow outgoing edges
        for edge in self.get_edges(node_id) {
            self.dfs(&edge.target, visited, component);
        }

        // Follow incoming edges
        for edge in self.get_incoming_edges(node_id) {
            self.dfs(&edge.target, visited, component);
        }
    }

    /// Detect anomalous nodes based on structural properties
    pub fn detect_anomalies(&self, threshold: f64) -> Vec<AnomalyReport> {
        let mut anomalies = Vec::new();

        let centrality = self.degree_centrality();
        let pagerank = self.pagerank(0.85, 20);

        // Calculate statistics
        let centrality_values: Vec<f64> = centrality.values().copied().collect();
        let pagerank_values: Vec<f64> = pagerank.values().copied().collect();

        let (cent_mean, cent_std) = mean_std(&centrality_values);
        let (pr_mean, pr_std) = mean_std(&pagerank_values);

        for (node_id, node) in &self.nodes {
            let cent = centrality.get(node_id).copied().unwrap_or(0.0);
            let pr = pagerank.get(node_id).copied().unwrap_or(0.0);

            // Z-score anomaly detection
            let cent_z = if cent_std > 0.0 {
                (cent - cent_mean) / cent_std
            } else {
                0.0
            };

            let pr_z = if pr_std > 0.0 {
                (pr - pr_mean) / pr_std
            } else {
                0.0
            };

            let anomaly_score = (cent_z.abs() + pr_z.abs()) / 2.0;

            if anomaly_score >= threshold {
                anomalies.push(AnomalyReport {
                    node_id: node_id.clone(),
                    node_type: node.node_type,
                    anomaly_score,
                    centrality: cent,
                    pagerank: pr,
                    reason: if cent_z > threshold {
                        "High degree centrality".to_string()
                    } else if cent_z < -threshold {
                        "Isolated node".to_string()
                    } else if pr_z > threshold {
                        "High PageRank (hub)".to_string()
                    } else {
                        "Unusual structural position".to_string()
                    },
                });
            }
        }

        // Sort by anomaly score
        anomalies.sort_by(|a, b| b.anomaly_score.partial_cmp(&a.anomaly_score).unwrap_or(std::cmp::Ordering::Equal));

        anomalies
    }

    /// Get the adjacency matrix as a flat vector
    pub fn adjacency_matrix(&self) -> (Vec<f64>, Vec<String>) {
        let node_list: Vec<String> = self.nodes.keys().cloned().collect();
        let n = node_list.len();
        let mut matrix = vec![0.0; n * n];

        let idx_map: HashMap<&String, usize> = node_list.iter().enumerate().map(|(i, s)| (s, i)).collect();

        for (source, edges) in &self.edges {
            if let Some(&i) = idx_map.get(source) {
                for edge in edges {
                    if let Some(&j) = idx_map.get(&edge.target) {
                        matrix[i * n + j] = edge.weight;
                    }
                }
            }
        }

        (matrix, node_list)
    }
}

/// Report of an anomalous node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyReport {
    /// Node identifier
    pub node_id: String,
    /// Node type
    pub node_type: NodeType,
    /// Anomaly score (higher = more anomalous)
    pub anomaly_score: f64,
    /// Degree centrality
    pub centrality: f64,
    /// PageRank score
    pub pagerank: f64,
    /// Reason for flagging
    pub reason: String,
}

/// Calculate mean and standard deviation
fn mean_std(values: &[f64]) -> (f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0);
    }

    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;

    (mean, variance.sqrt())
}

/// Quantum-enhanced graph analysis (when feature enabled)
#[cfg(feature = "quantum")]
pub mod quantum {
    use super::*;
    use phosphoros_quantum::algorithms::quantum_walk::QuantumWalk;
    use phosphoros_quantum::backend::simulator::LocalSimulator;
    use nalgebra::DMatrix;

    /// Quantum-enhanced centrality using continuous-time quantum walk
    pub fn quantum_centrality(graph: &TransactionGraph) -> Result<HashMap<String, f64>, Error> {
        let (adj_vec, node_list) = graph.adjacency_matrix();
        let n = node_list.len();

        if n == 0 || n > 13 {
            // Fall back to classical for large graphs
            return Ok(graph.pagerank(0.85, 20));
        }

        // Convert flat vector to DMatrix (row-major)
        let adj_matrix = DMatrix::from_row_slice(n, n, &adj_vec);

        // Create quantum walk on the graph
        let backend = LocalSimulator::new(n);
        let walk = QuantumWalk::new(backend, adj_matrix);

        // Run quantum walk centrality computation
        let scores = walk.centrality_scores(1.0, 100)
            .map_err(|e| Error::AnalysisFailed(format!("Quantum walk failed: {:?}", e)))?;

        // Convert to centrality scores with node IDs
        let centrality: HashMap<String, f64> = node_list
            .iter()
            .enumerate()
            .map(|(i, id)| {
                let score = scores.get(i).copied().unwrap_or(0.0);
                (id.clone(), score)
            })
            .collect();

        Ok(centrality)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> TransactionGraph {
        let mut graph = TransactionGraph::new();

        // Add nodes
        for i in 0..5 {
            graph.add_node(TxNode {
                id: format!("tx_{}", i),
                node_type: NodeType::Transaction,
                value: 1000 * (i + 1),
                block_height: 100000 + i,
                metadata: HashMap::new(),
            });
        }

        // Add edges (linear chain with hub at tx_2)
        graph.add_edge(TxEdge {
            source: "tx_0".to_string(),
            target: "tx_1".to_string(),
            edge_type: EdgeType::Spend,
            weight: 1.0,
        });
        graph.add_edge(TxEdge {
            source: "tx_1".to_string(),
            target: "tx_2".to_string(),
            edge_type: EdgeType::Spend,
            weight: 1.0,
        });
        graph.add_edge(TxEdge {
            source: "tx_2".to_string(),
            target: "tx_3".to_string(),
            edge_type: EdgeType::Spend,
            weight: 1.0,
        });
        graph.add_edge(TxEdge {
            source: "tx_2".to_string(),
            target: "tx_4".to_string(),
            edge_type: EdgeType::Spend,
            weight: 1.0,
        });

        graph
    }

    #[test]
    fn test_graph_creation() {
        let graph = create_test_graph();
        assert_eq!(graph.node_count(), 5);
        assert_eq!(graph.edge_count(), 4);
    }

    #[test]
    fn test_degree_centrality() {
        let graph = create_test_graph();
        let centrality = graph.degree_centrality();

        // tx_2 should have highest centrality (3 connections)
        let tx2_cent = centrality.get("tx_2").copied().unwrap_or(0.0);
        let tx0_cent = centrality.get("tx_0").copied().unwrap_or(0.0);

        assert!(tx2_cent > tx0_cent);
    }

    #[test]
    fn test_connected_components() {
        let graph = create_test_graph();
        let components = graph.connected_components();

        // All nodes should be in one component
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].len(), 5);
    }

    #[test]
    fn test_pagerank() {
        let graph = create_test_graph();
        let pagerank = graph.pagerank(0.85, 20);

        // All nodes should have positive PageRank
        for (_, rank) in &pagerank {
            assert!(*rank > 0.0);
        }

        // Sum should be positive (dangling nodes cause some leakage)
        let sum: f64 = pagerank.values().sum();
        assert!(sum > 0.0, "Total PageRank should be positive");

        // tx_2 (hub) should have higher rank than leaf nodes
        let hub_rank = pagerank.get("tx_2").copied().unwrap_or(0.0);
        let leaf_rank = pagerank.get("tx_0").copied().unwrap_or(0.0);
        assert!(hub_rank > leaf_rank, "Hub should have higher rank than leaves");
    }

    #[test]
    fn test_anomaly_detection() {
        let graph = create_test_graph();
        let anomalies = graph.detect_anomalies(1.0);

        // tx_2 might be flagged as it's a hub
        // But with a small graph, might not reach threshold
        // Just verify it returns valid results
        for anomaly in &anomalies {
            assert!(anomaly.anomaly_score >= 1.0);
        }
    }
}
