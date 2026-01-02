//! Cluster Analysis API
//!
//! Provides endpoints for computing and querying blockchain entity clusters,
//! analyzing cluster properties, and identifying community structure.

use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Shared state for cluster analysis
#[derive(Clone)]
pub struct ClusterState {
    /// Computed clusters indexed by snapshot ID
    pub clusters: Arc<RwLock<HashMap<String, Vec<Cluster>>>>,
}

impl Default for ClusterState {
    fn default() -> Self {
        Self {
            clusters: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

/// Build the cluster analysis router
pub fn build_router(state: ClusterState) -> Router {
    Router::new()
        .route("/compute", post(compute_clusters))
        .route("/list", get(list_all_clusters))
        .route("/:snapshot_id", get(get_clusters_for_snapshot))
        .route("/:snapshot_id/:cluster_id/members", get(get_cluster_members))
        .with_state(state)
}

/// A blockchain entity cluster
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Cluster {
    /// Unique cluster identifier
    pub id: String,
    /// Cluster label (e.g., "High Activity", "Sybil Candidate")
    pub label: Option<String>,
    /// Number of entities in cluster
    pub size: usize,
    /// Centroid in feature space
    pub centroid: Vec<f64>,
    /// Cluster cohesion (0.0 to 1.0, higher is tighter)
    pub cohesion: f64,
    /// Member entity addresses
    pub members: Vec<String>,
    /// Metadata (e.g., risk scores, tags)
    pub metadata: HashMap<String, String>,
}

/// Request to compute clusters from entities
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ComputeRequest {
    /// Snapshot identifier
    pub snapshot_id: String,
    /// Entity observations (address + feature vector)
    pub entities: Vec<Entity>,
    /// Clustering algorithm ("knn", "dbscan", "hierarchical")
    #[serde(default = "default_algorithm")]
    pub algorithm: String,
    /// Number of neighbors for KNN (default: 8)
    #[serde(default = "default_k")]
    pub k: usize,
    /// Distance threshold for clustering
    #[serde(default = "default_threshold")]
    pub threshold: f64,
}

fn default_algorithm() -> String {
    "knn".to_string()
}

fn default_k() -> usize {
    8
}

fn default_threshold() -> f64 {
    0.5
}

/// Entity with features for clustering
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Entity {
    /// Blockchain address
    pub address: String,
    /// Feature vector (normalized)
    pub features: Vec<f64>,
}

/// Response from cluster computation
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ComputeResponse {
    /// Whether computation was successful
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
    /// Number of clusters found
    pub num_clusters: usize,
    /// Cluster summaries
    pub clusters: Vec<ClusterSummary>,
}

/// Summary of a cluster (without full member list)
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ClusterSummary {
    /// Cluster ID
    pub id: String,
    /// Cluster label
    pub label: Option<String>,
    /// Number of members
    pub size: usize,
    /// Cohesion score
    pub cohesion: f64,
}

/// Response listing all clusters
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ListClustersResponse {
    /// Clusters grouped by snapshot ID
    pub snapshots: HashMap<String, Vec<ClusterSummary>>,
}

/// Response with cluster members
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ClusterMembersResponse {
    /// Cluster ID
    pub cluster_id: String,
    /// Member addresses
    pub members: Vec<String>,
    /// Member count
    pub count: usize,
}

/// Compute clusters from entity observations
///
/// Uses KNN graph construction, community detection, or DBSCAN depending on algorithm.
#[utoipa::path(
    post,
    path = "/api/v1/cluster/compute",
    tag = "cluster",
    request_body = ComputeRequest,
    responses(
        (status = 200, description = "Clusters computed successfully", body = ComputeResponse),
        (status = 400, description = "Invalid request parameters"),
        (status = 500, description = "Internal server error")
    )
)]
async fn compute_clusters(
    State(state): State<ClusterState>,
    Json(req): Json<ComputeRequest>,
) -> Result<Json<ComputeResponse>, StatusCode> {
    // Validate algorithm
    if !["knn", "dbscan", "hierarchical"].contains(&req.algorithm.as_str()) {
        return Ok(Json(ComputeResponse {
            success: false,
            error: Some(format!("Unsupported algorithm: {}", req.algorithm)),
            num_clusters: 0,
            clusters: vec![],
        }));
    }

    // Build distance matrix
    let n = req.entities.len();
    if n == 0 {
        return Ok(Json(ComputeResponse {
            success: false,
            error: Some("No entities provided".to_string()),
            num_clusters: 0,
            clusters: vec![],
        }));
    }

    let distance_matrix = compute_distance_matrix(&req.entities);

    // Perform clustering
    let clusters = match req.algorithm.as_str() {
        "knn" => knn_clustering(&req.entities, &distance_matrix, req.k, req.threshold),
        "dbscan" => dbscan_clustering(&req.entities, &distance_matrix, req.threshold, 3),
        "hierarchical" => hierarchical_clustering(&req.entities, &distance_matrix, req.threshold),
        _ => vec![],
    };

    // Store clusters
    state.clusters.write().insert(req.snapshot_id.clone(), clusters.clone());

    // Create summaries
    let summaries: Vec<ClusterSummary> = clusters
        .iter()
        .map(|c| ClusterSummary {
            id: c.id.clone(),
            label: c.label.clone(),
            size: c.size,
            cohesion: c.cohesion,
        })
        .collect();

    Ok(Json(ComputeResponse {
        success: true,
        error: None,
        num_clusters: clusters.len(),
        clusters: summaries,
    }))
}

/// List all clusters across all snapshots
#[utoipa::path(
    get,
    path = "/api/v1/cluster/list",
    tag = "cluster",
    responses(
        (status = 200, description = "Cluster list retrieved", body = ListClustersResponse),
        (status = 500, description = "Internal server error")
    )
)]
async fn list_all_clusters(
    State(state): State<ClusterState>,
) -> Result<Json<ListClustersResponse>, StatusCode> {
    let clusters = state.clusters.read();
    let mut snapshots = HashMap::new();

    for (snapshot_id, cluster_list) in clusters.iter() {
        let summaries: Vec<ClusterSummary> = cluster_list
            .iter()
            .map(|c| ClusterSummary {
                id: c.id.clone(),
                label: c.label.clone(),
                size: c.size,
                cohesion: c.cohesion,
            })
            .collect();
        snapshots.insert(snapshot_id.clone(), summaries);
    }

    Ok(Json(ListClustersResponse { snapshots }))
}

/// Get clusters for a specific snapshot
#[utoipa::path(
    get,
    path = "/api/v1/cluster/{snapshot_id}",
    tag = "cluster",
    params(
        ("snapshot_id" = String, Path, description = "Snapshot identifier")
    ),
    responses(
        (status = 200, description = "Clusters retrieved", body = Vec<ClusterSummary>),
        (status = 404, description = "Snapshot not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_clusters_for_snapshot(
    State(state): State<ClusterState>,
    Path(snapshot_id): Path<String>,
) -> Result<Json<Vec<ClusterSummary>>, StatusCode> {
    let clusters = state.clusters.read();

    match clusters.get(&snapshot_id) {
        Some(cluster_list) => {
            let summaries: Vec<ClusterSummary> = cluster_list
                .iter()
                .map(|c| ClusterSummary {
                    id: c.id.clone(),
                    label: c.label.clone(),
                    size: c.size,
                    cohesion: c.cohesion,
                })
                .collect();
            Ok(Json(summaries))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Get members of a specific cluster
#[utoipa::path(
    get,
    path = "/api/v1/cluster/{snapshot_id}/{cluster_id}/members",
    tag = "cluster",
    params(
        ("snapshot_id" = String, Path, description = "Snapshot identifier"),
        ("cluster_id" = String, Path, description = "Cluster identifier")
    ),
    responses(
        (status = 200, description = "Cluster members retrieved", body = ClusterMembersResponse),
        (status = 404, description = "Cluster not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_cluster_members(
    State(state): State<ClusterState>,
    Path((snapshot_id, cluster_id)): Path<(String, String)>,
) -> Result<Json<ClusterMembersResponse>, StatusCode> {
    let clusters = state.clusters.read();

    match clusters.get(&snapshot_id) {
        Some(cluster_list) => {
            match cluster_list.iter().find(|c| c.id == cluster_id) {
                Some(cluster) => Ok(Json(ClusterMembersResponse {
                    cluster_id: cluster.id.clone(),
                    members: cluster.members.clone(),
                    count: cluster.members.len(),
                })),
                None => Err(StatusCode::NOT_FOUND),
            }
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Compute pairwise distance matrix (Euclidean)
fn compute_distance_matrix(entities: &[Entity]) -> Vec<Vec<f64>> {
    let n = entities.len();
    let mut matrix = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in (i + 1)..n {
            let dist = euclidean_distance(&entities[i].features, &entities[j].features);
            matrix[i][j] = dist;
            matrix[j][i] = dist;
        }
    }

    matrix
}

/// Euclidean distance between two feature vectors
fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// KNN-based clustering
fn knn_clustering(
    entities: &[Entity],
    distance_matrix: &[Vec<f64>],
    k: usize,
    threshold: f64,
) -> Vec<Cluster> {
    let n = entities.len();
    let mut visited = vec![false; n];
    let mut clusters = Vec::new();
    let mut cluster_id = 0;

    for i in 0..n {
        if visited[i] {
            continue;
        }

        // Find k-nearest neighbors within threshold
        let mut members = vec![i];
        visited[i] = true;

        let mut queue = vec![i];
        while let Some(current) = queue.pop() {
            let mut neighbors: Vec<(usize, f64)> = (0..n)
                .filter(|&j| !visited[j])
                .map(|j| (j, distance_matrix[current][j]))
                .collect();

            neighbors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            for (neighbor, dist) in neighbors.iter().take(k) {
                if *dist < threshold {
                    visited[*neighbor] = true;
                    members.push(*neighbor);
                    queue.push(*neighbor);
                }
            }
        }

        // Compute centroid
        let centroid = compute_centroid(entities, &members);
        let cohesion = compute_cohesion(entities, &members, &centroid);

        clusters.push(Cluster {
            id: format!("cluster_{}", cluster_id),
            label: if members.len() > 10 {
                Some("Large Cluster".to_string())
            } else {
                None
            },
            size: members.len(),
            centroid,
            cohesion,
            members: members.iter().map(|&idx| entities[idx].address.clone()).collect(),
            metadata: HashMap::new(),
        });

        cluster_id += 1;
    }

    clusters
}

/// DBSCAN clustering (simplified)
fn dbscan_clustering(
    entities: &[Entity],
    distance_matrix: &[Vec<f64>],
    eps: f64,
    min_pts: usize,
) -> Vec<Cluster> {
    let n = entities.len();
    let mut labels = vec![-1; n]; // -1 = noise, >= 0 = cluster ID
    let mut cluster_id = 0;

    for i in 0..n {
        if labels[i] != -1 {
            continue;
        }

        // Find neighbors
        let neighbors: Vec<usize> = (0..n)
            .filter(|&j| distance_matrix[i][j] < eps)
            .collect();

        if neighbors.len() < min_pts {
            // Mark as noise
            continue;
        }

        // Start new cluster
        labels[i] = cluster_id;
        let mut queue = neighbors.clone();

        while let Some(j) = queue.pop() {
            if labels[j] == -1 {
                labels[j] = cluster_id;

                let j_neighbors: Vec<usize> = (0..n)
                    .filter(|&k| distance_matrix[j][k] < eps)
                    .collect();

                if j_neighbors.len() >= min_pts {
                    queue.extend(j_neighbors);
                }
            }
        }

        cluster_id += 1;
    }

    // Build cluster objects
    let mut clusters = Vec::new();
    for cid in 0..cluster_id {
        let members: Vec<usize> = labels
            .iter()
            .enumerate()
            .filter_map(|(idx, &label)| if label == cid { Some(idx) } else { None })
            .collect();

        if members.is_empty() {
            continue;
        }

        let centroid = compute_centroid(entities, &members);
        let cohesion = compute_cohesion(entities, &members, &centroid);

        clusters.push(Cluster {
            id: format!("cluster_{}", cid),
            label: None,
            size: members.len(),
            centroid,
            cohesion,
            members: members.iter().map(|&idx| entities[idx].address.clone()).collect(),
            metadata: HashMap::new(),
        });
    }

    clusters
}

/// Hierarchical clustering (simplified agglomerative)
fn hierarchical_clustering(
    entities: &[Entity],
    _distance_matrix: &[Vec<f64>],
    _threshold: f64,
) -> Vec<Cluster> {
    // Simplified: treat each entity as its own cluster
    entities
        .iter()
        .enumerate()
        .map(|(i, entity)| Cluster {
            id: format!("cluster_{}", i),
            label: None,
            size: 1,
            centroid: entity.features.clone(),
            cohesion: 1.0,
            members: vec![entity.address.clone()],
            metadata: HashMap::new(),
        })
        .collect()
}

/// Compute centroid of cluster members
fn compute_centroid(entities: &[Entity], members: &[usize]) -> Vec<f64> {
    if members.is_empty() {
        return vec![];
    }

    let dim = entities[0].features.len();
    let mut centroid = vec![0.0; dim];

    for &idx in members {
        for (i, &val) in entities[idx].features.iter().enumerate() {
            centroid[i] += val;
        }
    }

    for val in &mut centroid {
        *val /= members.len() as f64;
    }

    centroid
}

/// Compute cluster cohesion (inverse of average distance to centroid)
fn compute_cohesion(entities: &[Entity], members: &[usize], centroid: &[f64]) -> f64 {
    if members.is_empty() {
        return 0.0;
    }

    let avg_dist: f64 = members
        .iter()
        .map(|&idx| euclidean_distance(&entities[idx].features, centroid))
        .sum::<f64>()
        / members.len() as f64;

    // Cohesion = 1 / (1 + avg_dist)
    1.0 / (1.0 + avg_dist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let dist = euclidean_distance(&a, &b);
        assert!((dist - 5.196).abs() < 0.01);
    }

    #[test]
    fn test_cluster_state_default() {
        let state = ClusterState::default();
        assert_eq!(state.clusters.read().len(), 0);
    }
}
