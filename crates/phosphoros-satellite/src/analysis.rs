//! Analytics pipeline for blockchain forensic analysis.
//!
//! This module performs multi-perspective resonance analysis on blockchain entity snapshots,
//! computing hotspots, anomalies, topology, and entropy measures.

use std::collections::HashMap;

use ndarray::{Array2, ArrayView1};
use petgraph::graph::UnGraph;
use uuid::Uuid;

use crate::config::AnalysisConfig;
use crate::models::{
    AnalysisReport, AnalysisRequest, AnomalyScore, EntropySummary, ResonanceHotspot,
    SnapshotRecord, TopologySummary,
};

/// Analytics pipeline performing multi-perspective blockchain resonance analysis.
#[derive(Debug, Clone)]
pub struct AnalyticsPipeline {
    config: AnalysisConfig,
}

impl AnalyticsPipeline {
    /// Creates a new analytics pipeline with the given configuration.
    pub fn new(config: AnalysisConfig) -> Self {
        Self { config }
    }

    /// Runs the complete analysis pipeline on a snapshot.
    pub fn run(&self, snapshot: &SnapshotRecord, request: AnalysisRequest) -> AnalysisReport {
        let knn_k = request.knn_k.unwrap_or(self.config.knn_k);
        let entropy_bins = request.entropy_bins.unwrap_or(self.config.entropy_bins);
        let resonance_threshold = request
            .resonance_threshold
            .unwrap_or(self.config.resonance_threshold);

        let features = build_feature_matrix(snapshot);
        let distances = compute_distance_matrix(&features);
        let (edges, adjacency) = build_knn_graph(&distances, knn_k);

        let hotspots =
            compute_resonance_hotspots(snapshot, &distances, &adjacency, resonance_threshold);
        let anomalies = compute_anomaly_scores(snapshot, &distances, &adjacency);
        let topology = compute_topology(snapshot, &edges);
        let entropy = compute_entropy(&features, entropy_bins);

        AnalysisReport {
            snapshot_id: snapshot.id,
            label: snapshot.label.clone(),
            captured_at: snapshot.captured_at,
            resonance_hotspots: hotspots,
            anomaly_scores: anomalies,
            topology,
            entropy,
        }
    }
}

fn build_feature_matrix(snapshot: &SnapshotRecord) -> Array2<f64> {
    let dim = snapshot
        .observations
        .iter()
        .map(|obs| obs.features.len())
        .max()
        .unwrap_or(0);
    let rows = snapshot.observations.len();
    let mut matrix = Array2::zeros((rows, dim.max(1)));
    for (row, obs) in snapshot.observations.iter().enumerate() {
        for (col, value) in obs.features.iter().enumerate() {
            matrix[(row, col)] = *value;
        }
    }
    matrix
}

fn compute_distance_matrix(features: &Array2<f64>) -> Array2<f64> {
    let rows = features.nrows();
    let mut matrix = Array2::zeros((rows, rows));
    for i in 0..rows {
        for j in i + 1..rows {
            let dist = l2_distance(features.row(i), features.row(j));
            matrix[(i, j)] = dist;
            matrix[(j, i)] = dist;
        }
    }
    matrix
}

fn l2_distance(a: ArrayView1<'_, f64>, b: ArrayView1<'_, f64>) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| {
            let diff = x - y;
            diff * diff
        })
        .sum::<f64>()
        .sqrt()
}

fn build_knn_graph(
    distances: &Array2<f64>,
    k: usize,
) -> (Vec<(usize, usize, f64)>, Vec<Vec<usize>>) {
    let rows = distances.nrows();
    let mut edges = Vec::new();
    let mut adjacency = vec![Vec::new(); rows];
    if rows == 0 {
        return (edges, adjacency);
    }

    for i in 0..rows {
        let mut neighbours: Vec<(usize, f64)> = (0..rows)
            .filter(|&j| j != i)
            .map(|j| (j, distances[(i, j)]))
            .collect();
        neighbours.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        for (j, dist) in neighbours.into_iter().take(k) {
            if !adjacency[i].contains(&j) {
                adjacency[i].push(j);
            }
            if !adjacency[j].contains(&i) {
                adjacency[j].push(i);
            }
            let edge = if i < j { (i, j, dist) } else { (j, i, dist) };
            if !edges
                .iter()
                .any(|existing| existing.0 == edge.0 && existing.1 == edge.1)
            {
                edges.push(edge);
            }
        }
    }
    (edges, adjacency)
}

fn compute_resonance_hotspots(
    snapshot: &SnapshotRecord,
    distances: &Array2<f64>,
    adjacency: &[Vec<usize>],
    threshold: f64,
) -> Vec<ResonanceHotspot> {
    snapshot
        .observations
        .iter()
        .enumerate()
        .filter_map(|(idx, obs)| {
            let neighbours = &adjacency[idx];
            if neighbours.is_empty() {
                return None;
            }
            let resonance: f64 = neighbours
                .iter()
                .map(|&n| 1.0 / (1.0 + distances[(idx, n)]))
                .sum::<f64>()
                / neighbours.len() as f64;
            if resonance >= threshold {
                let neighbour_ids = neighbours
                    .iter()
                    .map(|&n| snapshot.observations[n].id)
                    .collect();
                Some(ResonanceHotspot {
                    origin: obs.id,
                    magnitude: resonance,
                    neighbours: neighbour_ids,
                })
            } else {
                None
            }
        })
        .collect()
}

fn compute_anomaly_scores(
    snapshot: &SnapshotRecord,
    distances: &Array2<f64>,
    adjacency: &[Vec<usize>],
) -> Vec<AnomalyScore> {
    let mut influence = Vec::new();
    for (idx, neighbours) in adjacency.iter().enumerate() {
        if neighbours.is_empty() {
            influence.push((idx, 0.0));
            continue;
        }
        let mean_distance =
            neighbours.iter().map(|&n| distances[(idx, n)]).sum::<f64>() / neighbours.len() as f64;
        let degree = neighbours.len() as f64;
        let score = degree / (1.0 + mean_distance);
        influence.push((idx, score));
    }

    let mean =
        influence.iter().map(|(_, score)| score).sum::<f64>() / influence.len().max(1) as f64;
    let variance = influence
        .iter()
        .map(|(_, score)| {
            let diff = score - mean;
            diff * diff
        })
        .sum::<f64>()
        / influence.len().max(1) as f64;
    let std_dev = variance.sqrt().max(f64::EPSILON);

    influence
        .into_iter()
        .map(|(idx, score)| AnomalyScore {
            entity: snapshot.observations[idx].id,
            z_score: (score - mean) / std_dev,
        })
        .collect()
}

fn compute_topology(snapshot: &SnapshotRecord, edges: &[(usize, usize, f64)]) -> TopologySummary {
    let mut graph = UnGraph::<Uuid, f64>::default();
    for obs in &snapshot.observations {
        graph.add_node(obs.id);
    }
    for &(i, j, weight) in edges {
        let node_a = petgraph::graph::NodeIndex::new(i);
        let node_b = petgraph::graph::NodeIndex::new(j);
        if graph.find_edge(node_a, node_b).is_none() {
            graph.add_edge(node_a, node_b, weight);
        }
    }

    let components = petgraph::algo::connected_components(&graph);
    let articulation_points = count_articulation_points(&graph);
    let nodes = graph.node_count() as f64;
    let edges_count = graph.edge_count() as f64;
    let betti_0 = components as f64;
    let betti_1 = (edges_count - nodes + components as f64).max(0.0);

    TopologySummary {
        components,
        articulation_points,
        betti_estimate: vec![betti_0, betti_1],
    }
}

fn count_articulation_points(graph: &UnGraph<Uuid, f64>) -> usize {
    let mut time = 0usize;
    let mut visited = HashMap::new();
    let mut lowlink = HashMap::new();
    let mut parent = HashMap::new();
    let mut articulation = HashMap::new();

    for node in graph.node_indices() {
        if !visited.contains_key(&node) {
            dfs_articulation(
                graph,
                node,
                &mut time,
                &mut visited,
                &mut lowlink,
                &mut parent,
                &mut articulation,
            );
        }
    }

    articulation.values().filter(|&&value| value).count()
}

#[allow(clippy::too_many_arguments)]
fn dfs_articulation(
    graph: &UnGraph<Uuid, f64>,
    node: petgraph::graph::NodeIndex,
    time: &mut usize,
    visited: &mut HashMap<petgraph::graph::NodeIndex, usize>,
    lowlink: &mut HashMap<petgraph::graph::NodeIndex, usize>,
    parent: &mut HashMap<petgraph::graph::NodeIndex, Option<petgraph::graph::NodeIndex>>,
    articulation: &mut HashMap<petgraph::graph::NodeIndex, bool>,
) {
    *time += 1;
    visited.insert(node, *time);
    lowlink.insert(node, *time);
    let mut children = 0usize;

    for neighbour in graph.neighbors(node) {
        if !visited.contains_key(&neighbour) {
            parent.insert(neighbour, Some(node));
            children += 1;
            dfs_articulation(
                graph,
                neighbour,
                time,
                visited,
                lowlink,
                parent,
                articulation,
            );
            let low = *lowlink.get(&neighbour).unwrap_or(&usize::MAX);
            let entry = lowlink.entry(node).or_insert(usize::MAX);
            *entry = (*entry).min(low);
            if parent.get(&node).copied().flatten().is_some() && low >= visited[&node] {
                articulation.insert(node, true);
            }
        } else if parent.get(&node).copied().flatten() != Some(neighbour) {
            let entry = lowlink.entry(node).or_insert(usize::MAX);
            *entry = (*entry).min(*visited.get(&neighbour).unwrap());
        }
    }

    if parent.get(&node).copied().flatten().is_none() && children > 1 {
        articulation.insert(node, true);
    }
}

fn compute_entropy(features: &Array2<f64>, bins: usize) -> EntropySummary {
    let bins = bins.max(4);
    let mut histogram = vec![0f64; bins];
    let mut total = 0f64;
    for value in features.iter() {
        let clamped = value.clamp(0.0, 1.0);
        let idx = ((clamped * bins as f64).floor() as usize).min(bins - 1);
        histogram[idx] += 1.0;
        total += 1.0;
    }
    if total == 0.0 {
        return EntropySummary {
            spectral: 0.0,
            distribution: vec![0.0; bins],
        };
    }

    let distribution: Vec<f64> = histogram.iter().map(|count| count / total).collect();
    let spectral = -distribution
        .iter()
        .filter(|&&p| p > 0.0)
        .map(|&p| p * (p.ln() / std::f64::consts::LN_2))
        .sum::<f64>();

    EntropySummary {
        spectral,
        distribution,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{EntityObservation, SnapshotIngest};
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};
    use std::collections::HashMap;

    fn build_snapshot(n: usize, dim: usize) -> SnapshotRecord {
        let mut rng = StdRng::seed_from_u64(42);
        let mut observations = Vec::new();
        for i in 0..n {
            let mut features = Vec::new();
            for _ in 0..dim {
                features.push(rng.gen_range(0.0..1.0));
            }
            let obs = EntityObservation {
                id: Uuid::new_v4(),
                address: format!("wallet_{}", i),
                features,
                connections: Vec::new(),
                metadata: HashMap::new(),
            };
            observations.push(obs);
        }
        let ingest = SnapshotIngest {
            label: "test".into(),
            observations,
            context: HashMap::new(),
        };
        SnapshotRecord::new(ingest).unwrap()
    }

    #[test]
    fn test_pipeline_generates_reports() {
        let snapshot = build_snapshot(10, 5);
        let pipeline = AnalyticsPipeline::new(AnalysisConfig::default());
        let report = pipeline.run(&snapshot, AnalysisRequest::default());
        assert_eq!(report.snapshot_id, snapshot.id);
        assert!(report.topology.components > 0);
        assert!(!report.entropy.distribution.is_empty());
    }
}
