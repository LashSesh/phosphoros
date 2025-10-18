//! Cluster service for KNN graph construction and resonance hotspot detection
//!
//! This service builds clusters from entities in the data pool using
//! k-nearest neighbors and resonance calculations.

use super::{ServiceState, DataPool, ClusterData};
use parking_lot::RwLock;
use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;

/// Cluster engine service
pub struct ClusterService {
    state: Arc<RwLock<ServiceState>>,
    data_pool: Arc<RwLock<DataPool>>,
}

impl ClusterService {
    /// Create a new cluster service
    pub fn new(state: Arc<RwLock<ServiceState>>, data_pool: Arc<RwLock<DataPool>>) -> Self {
        Self {
            state,
            data_pool,
        }
    }

    /// Start the cluster service
    pub fn start(&self) {
        self.state.write().running = true;
    }

    /// Pause the cluster service
    pub fn pause(&self) {
        self.state.write().running = false;
    }

    /// Execute one clustering step
    pub fn tick(&self) {
        let mut state = self.state.write();
        if !state.running {
            return;
        }

        let mut pool = self.data_pool.write();
        
        // Need at least 5 entities to form a cluster
        if pool.entities.len() < 5 {
            return;
        }

        // Simple clustering: group entities with similar feature vectors
        let entities: Vec<_> = pool.entities.values().collect();
        
        // Find groups of similar entities
        for i in 0..entities.len().saturating_sub(4) {
            let base = &entities[i];
            let mut similar: Vec<Uuid> = vec![base.id];
            
            // Find similar entities
            for other in entities.iter().skip(i + 1) {
                if Self::is_similar(&base.features, &other.features) {
                    similar.push(other.id);
                    if similar.len() >= 5 {
                        break;
                    }
                }
            }

            // Create cluster if we found enough similar entities
            if similar.len() >= 5 {
                let cluster_id = format!("cluster_{}", Uuid::new_v4());
                let resonance = Self::calculate_resonance(&similar, &pool.entities);
                
                let cluster = ClusterData {
                    id: cluster_id.clone(),
                    members: similar,
                    resonance,
                    timestamp: Utc::now(),
                };
                
                pool.clusters.insert(cluster_id, cluster);
                state.processed += 1;
                break; // Only create one cluster per tick
            }
        }

        state.last_update = Some(Utc::now());
        pool.last_activity = Some(Utc::now());
    }

    /// Check if two feature vectors are similar
    fn is_similar(a: &[f64], b: &[f64]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        // Calculate Euclidean distance
        let dist: f64 = a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt();
        
        dist < 0.3 // Threshold for similarity
    }

    /// Calculate resonance for a cluster
    fn calculate_resonance(
        members: &[Uuid],
        entities: &std::collections::HashMap<Uuid, super::EntityData>,
    ) -> f64 {
        let mut sum = 0.0;
        let mut count = 0;
        
        for id in members {
            if let Some(entity) = entities.get(id) {
                // Simple resonance: average of features
                sum += entity.features.iter().sum::<f64>() / entity.features.len() as f64;
                count += 1;
            }
        }
        
        if count > 0 {
            sum / count as f64
        } else {
            0.0
        }
    }

    /// Get statistics
    pub fn stats(&self) -> ClusterStats {
        let state = self.state.read();
        let pool = self.data_pool.read();
        ClusterStats {
            running: state.running,
            clusters_found: pool.clusters.len(),
            last_update: state.last_update,
        }
    }
}

/// Cluster statistics
#[derive(Debug, Clone)]
pub struct ClusterStats {
    /// Running status
    pub running: bool,
    /// Clusters found
    pub clusters_found: usize,
    /// Last update
    pub last_update: Option<chrono::DateTime<chrono::Utc>>,
}
