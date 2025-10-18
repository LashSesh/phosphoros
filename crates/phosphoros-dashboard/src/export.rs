//! Export functionality for data persistence and reporting
//!
//! This module provides various export formats for dashboard data.

use crate::panels::{SeedInfo, ClusterInfo};
use crate::services::ClusterData;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs;
use thiserror::Error;
use chrono::{DateTime, Utc};

/// Export error
#[derive(Debug, Error)]
pub enum ExportError {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    /// CSV error
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
}

/// Export format
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    /// JSON format
    Json,
    /// CSV format
    Csv,
    /// Markdown report
    Markdown,
}

/// Cluster export record
#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterExportRecord {
    /// Cluster ID
    pub id: String,
    /// Number of members
    pub members: usize,
    /// Resonance score
    pub resonance: f64,
    /// Discovery timestamp
    pub discovered_at: String,
}

impl From<&ClusterInfo> for ClusterExportRecord {
    fn from(cluster: &ClusterInfo) -> Self {
        Self {
            id: cluster.id.clone(),
            members: cluster.members,
            resonance: cluster.resonance,
            discovered_at: cluster.discovered_at.to_rfc3339(),
        }
    }
}

/// Seed export record
#[derive(Debug, Serialize, Deserialize)]
pub struct SeedExportRecord {
    /// Seed ID
    pub id: String,
    /// Mnemonic (masked)
    pub mnemonic_masked: String,
    /// Number of addresses
    pub address_count: usize,
    /// Import timestamp
    pub imported_at: String,
}

impl From<&SeedInfo> for SeedExportRecord {
    fn from(seed: &SeedInfo) -> Self {
        Self {
            id: seed.id.clone(),
            mnemonic_masked: seed.mnemonic_masked.clone(),
            address_count: seed.addresses.len(),
            imported_at: seed.imported_at.to_rfc3339(),
        }
    }
}

/// Export service
pub struct ExportService;

impl ExportService {
    /// Export clusters to file
    pub fn export_clusters<P: AsRef<Path>>(
        clusters: &[ClusterInfo],
        path: P,
        format: ExportFormat,
    ) -> Result<(), ExportError> {
        let records: Vec<ClusterExportRecord> = clusters.iter().map(|c| c.into()).collect();
        
        match format {
            ExportFormat::Json => {
                let json = serde_json::to_string_pretty(&records)
                    .map_err(|e| ExportError::Serialization(e.to_string()))?;
                fs::write(path, json)?;
            }
            ExportFormat::Csv => {
                let mut wtr = csv::Writer::from_path(path)?;
                for record in records {
                    wtr.serialize(record)?;
                }
                wtr.flush()?;
            }
            ExportFormat::Markdown => {
                let mut md = String::from("# Cluster Export Report\n\n");
                md.push_str(&format!("Generated: {}\n\n", Utc::now().to_rfc3339()));
                md.push_str(&format!("Total Clusters: {}\n\n", clusters.len()));
                md.push_str("## Clusters\n\n");
                md.push_str("| ID | Members | Resonance | Discovered At |\n");
                md.push_str("|---|---|---|---|\n");
                
                for record in records {
                    md.push_str(&format!(
                        "| {} | {} | {:.3} | {} |\n",
                        &record.id[..10],
                        record.members,
                        record.resonance,
                        record.discovered_at
                    ));
                }
                
                fs::write(path, md)?;
            }
        }
        
        Ok(())
    }
    
    /// Export seeds to file
    pub fn export_seeds<P: AsRef<Path>>(
        seeds: &[SeedInfo],
        path: P,
        format: ExportFormat,
    ) -> Result<(), ExportError> {
        let records: Vec<SeedExportRecord> = seeds.iter().map(|s| s.into()).collect();
        
        match format {
            ExportFormat::Json => {
                let json = serde_json::to_string_pretty(&records)
                    .map_err(|e| ExportError::Serialization(e.to_string()))?;
                fs::write(path, json)?;
            }
            ExportFormat::Csv => {
                let mut wtr = csv::Writer::from_path(path)?;
                for record in records {
                    wtr.serialize(record)?;
                }
                wtr.flush()?;
            }
            ExportFormat::Markdown => {
                let mut md = String::from("# Seed Export Report\n\n");
                md.push_str(&format!("Generated: {}\n\n", Utc::now().to_rfc3339()));
                md.push_str(&format!("Total Seeds: {}\n\n", seeds.len()));
                md.push_str("## Seeds\n\n");
                md.push_str("| ID | Mnemonic (Masked) | Addresses | Imported At |\n");
                md.push_str("|---|---|---|---|\n");
                
                for record in records {
                    md.push_str(&format!(
                        "| {} | {} | {} | {} |\n",
                        &record.id[..10],
                        record.mnemonic_masked,
                        record.address_count,
                        record.imported_at
                    ));
                }
                
                fs::write(path, md)?;
            }
        }
        
        Ok(())
    }
    
    /// Generate full system report
    pub fn generate_system_report<P: AsRef<Path>>(
        seeds: &[SeedInfo],
        clusters: &[ClusterInfo],
        stats: &SystemStats,
        path: P,
    ) -> Result<(), ExportError> {
        let mut md = String::from("# PHOSPHOROS Dashboard System Report\n\n");
        md.push_str(&format!("Generated: {}\n\n", Utc::now().to_rfc3339()));
        
        md.push_str("## System Statistics\n\n");
        md.push_str(&format!("- Seeds Imported: {}\n", seeds.len()));
        md.push_str(&format!("- Clusters Discovered: {}\n", clusters.len()));
        md.push_str(&format!("- Entities Scraped: {}\n", stats.entities_scraped));
        md.push_str(&format!("- Anomalies Detected: {}\n", stats.anomalies_detected));
        md.push_str(&format!("- Analysis Runs: {}\n\n", stats.analysis_runs));
        
        md.push_str("## Service Status\n\n");
        md.push_str(&format!("- Scraper: {}\n", if stats.scraper_running { "Running" } else { "Stopped" }));
        md.push_str(&format!("- Analyzer: {}\n", if stats.analyzer_running { "Running" } else { "Stopped" }));
        md.push_str(&format!("- Cluster Engine: {}\n\n", if stats.cluster_engine_running { "Running" } else { "Stopped" }));
        
        md.push_str("## Top Clusters\n\n");
        md.push_str("| ID | Members | Resonance |\n");
        md.push_str("|---|---|---|\n");
        
        for cluster in clusters.iter().take(10) {
            md.push_str(&format!(
                "| {} | {} | {:.3} |\n",
                &cluster.id[..10],
                cluster.members,
                cluster.resonance
            ));
        }
        
        fs::write(path, md)?;
        Ok(())
    }
}

/// System statistics for reporting
#[derive(Debug, Clone)]
pub struct SystemStats {
    /// Entities scraped
    pub entities_scraped: usize,
    /// Anomalies detected
    pub anomalies_detected: usize,
    /// Analysis runs
    pub analysis_runs: usize,
    /// Scraper running
    pub scraper_running: bool,
    /// Analyzer running
    pub analyzer_running: bool,
    /// Cluster engine running
    pub cluster_engine_running: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    
    #[test]
    fn test_export_clusters_json() {
        let clusters = vec![];
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("clusters.json");
        
        let result = ExportService::export_clusters(&clusters, &path, ExportFormat::Json);
        assert!(result.is_ok());
        assert!(path.exists());
    }
}
