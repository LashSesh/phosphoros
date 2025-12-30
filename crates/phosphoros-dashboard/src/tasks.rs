//! Task management for async operations
//!
//! This module provides async task spawning and management for long-running
//! operations in the dashboard.

use tokio::sync::mpsc;
use std::sync::Arc;
use parking_lot::RwLock;
use chrono::Utc;
use uuid::Uuid;

use crate::services::{ServiceManager, EntityData, AnomalyData, ClusterData};

/// Task message types
#[derive(Debug, Clone)]
pub enum TaskMessage {
    /// Service tick update
    ServiceTick,
    /// Entity discovered
    EntityDiscovered(EntityData),
    /// Anomaly detected
    AnomalyDetected(AnomalyData),
    /// Cluster found
    ClusterFound(ClusterData),
    /// Task completed
    TaskCompleted { id: String, success: bool },
}

/// Task manager for async operations
pub struct TaskManager {
    tx: mpsc::UnboundedSender<TaskMessage>,
    rx: Arc<RwLock<mpsc::UnboundedReceiver<TaskMessage>>>,
}

impl TaskManager {
    /// Create a new task manager
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            tx,
            rx: Arc::new(RwLock::new(rx)),
        }
    }
    
    /// Get a sender clone
    pub fn sender(&self) -> mpsc::UnboundedSender<TaskMessage> {
        self.tx.clone()
    }
    
    /// Try to receive a message
    pub fn try_recv(&self) -> Option<TaskMessage> {
        self.rx.write().try_recv().ok()
    }
    
    /// Spawn scraper task
    pub fn spawn_scraper(
        &self,
        service_manager: Arc<ServiceManager>,
    ) -> tokio::task::JoinHandle<()> {
        let tx = self.tx.clone();
        
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                
                // Check if scraper is running
                if !service_manager.scraper.read().running {
                    continue;
                }
                
                // Generate entity
                let entity = EntityData {
                    id: Uuid::new_v4(),
                    address: format!("0x{:x}", rand::random::<u64>()),
                    features: vec![
                        rand::random::<f64>(),
                        rand::random::<f64>(),
                        rand::random::<f64>(),
                        rand::random::<f64>(),
                        rand::random::<f64>(),
                    ],
                    timestamp: Utc::now(),
                };
                
                // Add to data pool
                service_manager.data_pool.write().entities.insert(entity.id, entity.clone());
                service_manager.data_pool.write().last_activity = Some(Utc::now());
                
                // Update stats
                {
                    let mut state = service_manager.scraper.write();
                    state.processed += 1;
                    state.last_update = Some(Utc::now());
                }
                
                // Send message
                let _ = tx.send(TaskMessage::EntityDiscovered(entity));
            }
        })
    }
    
    /// Spawn analyzer task
    pub fn spawn_analyzer(
        &self,
        service_manager: Arc<ServiceManager>,
    ) -> tokio::task::JoinHandle<()> {
        let tx = self.tx.clone();
        
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                
                // Check if analyzer is running
                if !service_manager.analyzer.read().running {
                    continue;
                }
                
                // Get entities to analyze
                let entities: Vec<EntityData> = {
                    let pool = service_manager.data_pool.read();
                    pool.entities.values().cloned().collect()
                };
                
                // Analyze for anomalies
                for entity in entities {
                    if entity.features.iter().any(|&f| f > 0.95 || f < 0.05) {
                        let anomaly = AnomalyData {
                            id: Uuid::new_v4(),
                            entity_id: entity.id,
                            score: entity.features.iter().sum::<f64>() / entity.features.len() as f64,
                            reason: "Extreme feature values".to_string(),
                            timestamp: Utc::now(),
                        };
                        
                        service_manager.data_pool.write().anomalies.push(anomaly.clone());
                        let _ = tx.send(TaskMessage::AnomalyDetected(anomaly));
                    }
                }
                
                // Update stats
                {
                    let mut state = service_manager.analyzer.write();
                    state.processed += 1;
                    state.last_update = Some(Utc::now());
                }
            }
        })
    }
    
    /// Spawn cluster engine task
    pub fn spawn_cluster_engine(
        &self,
        service_manager: Arc<ServiceManager>,
    ) -> tokio::task::JoinHandle<()> {
        let tx = self.tx.clone();
        
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                
                // Check if cluster engine is running
                if !service_manager.cluster_engine.read().running {
                    continue;
                }
                
                // Get entities for clustering
                let entities: Vec<EntityData> = {
                    let pool = service_manager.data_pool.read();
                    pool.entities.values().cloned().collect()
                };
                
                // Need at least 5 entities
                if entities.len() < 5 {
                    continue;
                }
                
                // Simple clustering: find similar entities
                for i in 0..entities.len().saturating_sub(4) {
                    let base = &entities[i];
                    let mut cluster_members = vec![base.id];
                    
                    for other in entities.iter().skip(i + 1) {
                        // Calculate distance
                        let dist: f64 = base.features.iter()
                            .zip(other.features.iter())
                            .map(|(a, b)| (a - b).powi(2))
                            .sum::<f64>()
                            .sqrt();
                        
                        if dist < 0.3 {
                            cluster_members.push(other.id);
                            if cluster_members.len() >= 5 {
                                break;
                            }
                        }
                    }
                    
                    // Create cluster if enough members
                    if cluster_members.len() >= 5 {
                        let resonance = rand::random::<f64>() * 0.5 + 0.5; // 0.5-1.0
                        
                        let cluster = ClusterData {
                            id: format!("cluster_{}", Uuid::new_v4()),
                            members: cluster_members,
                            resonance,
                            timestamp: Utc::now(),
                        };
                        
                        service_manager.data_pool.write().clusters.insert(cluster.id.clone(), cluster.clone());
                        
                        // Update stats
                        {
                            let mut state = service_manager.cluster_engine.write();
                            state.processed += 1;
                            state.last_update = Some(Utc::now());
                        }
                        
                        let _ = tx.send(TaskMessage::ClusterFound(cluster));
                        break; // Only one cluster per iteration
                    }
                }
            }
        })
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}
