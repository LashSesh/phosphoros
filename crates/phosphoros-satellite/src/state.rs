//! State management for the Satellite subsystem.

use std::collections::{BTreeMap, VecDeque};

use parking_lot::RwLock;
use uuid::Uuid;

use crate::config::AnalysisConfig;
use crate::models::SnapshotRecord;

/// Manages the state of ingested snapshots with bounded retention.
#[derive(Debug)]
pub struct SatelliteState {
    snapshots: RwLock<BTreeMap<Uuid, SnapshotRecord>>,
    order: RwLock<VecDeque<Uuid>>,
    max_snapshots: usize,
}

impl SatelliteState {
    /// Creates a new state manager with the given configuration.
    pub fn new(config: &AnalysisConfig) -> Self {
        Self {
            snapshots: RwLock::new(BTreeMap::new()),
            order: RwLock::new(VecDeque::new()),
            max_snapshots: config.max_snapshots.max(1),
        }
    }

    /// Inserts a snapshot and returns its ID.
    ///
    /// If max_snapshots is exceeded, the oldest snapshot is removed.
    pub fn insert(&self, snapshot: SnapshotRecord) -> Uuid {
        let id = snapshot.id;
        
        // Insert into map
        {
            let mut map = self.snapshots.write();
            map.insert(id, snapshot);
        }
        
        // Update order and enforce size limit
        let mut order = self.order.write();
        order.push_back(id);
        
        while order.len() > self.max_snapshots {
            if let Some(removed) = order.pop_front() {
                self.snapshots.write().remove(&removed);
            }
        }
        
        id
    }

    /// Lists all snapshots, sorted by capture time.
    pub fn list(&self) -> Vec<SnapshotRecord> {
        let map = self.snapshots.read();
        let mut snapshots: Vec<_> = map.values().cloned().collect();
        snapshots.sort_by_key(|record| record.captured_at);
        snapshots
    }

    /// Retrieves a snapshot by ID.
    pub fn get(&self, id: &Uuid) -> Option<SnapshotRecord> {
        self.snapshots.read().get(id).cloned()
    }

    /// Returns the number of snapshots currently stored.
    pub fn len(&self) -> usize {
        self.snapshots.read().len()
    }

    /// Returns true if no snapshots are stored.
    pub fn is_empty(&self) -> bool {
        self.snapshots.read().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{EntityObservation, SnapshotIngest};
    use std::collections::HashMap;

    #[test]
    fn test_bounded_retention() {
        let mut config = AnalysisConfig::default();
        config.max_snapshots = 2;
        let state = SatelliteState::new(&config);

        // Insert 3 snapshots
        for i in 0..3 {
            let ingest = SnapshotIngest {
                label: format!("snapshot_{}", i),
                observations: vec![EntityObservation {
                    id: Uuid::new_v4(),
                    address: format!("0xaddr{}", i),
                    features: vec![1.0],
                    connections: vec![],
                    metadata: HashMap::new(),
                }],
                context: HashMap::new(),
            };
            let snapshot = SnapshotRecord::new(ingest).unwrap();
            state.insert(snapshot);
        }

        // Should only have 2 snapshots
        assert_eq!(state.len(), 2);
    }
}
