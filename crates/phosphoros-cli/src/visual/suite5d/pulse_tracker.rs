use std::collections::HashMap;

use chrono::{DateTime, Local, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct PulseEvent {
    pub timestamp: DateTime<Utc>,
    pub seed: String,
    pub cell_type: String,
}

#[derive(Debug, Default, Clone)]
pub struct PulseTracker {
    timeline: Vec<PulseEvent>,
    activity_log: HashMap<String, Vec<DateTime<Utc>>>,
}

impl PulseTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_event(&mut self, seed: impl Into<String>, cell_type: impl Into<String>) {
        let seed = seed.into();
        let cell_type = cell_type.into();
        let timestamp = Utc::now();
        self.timeline.push(PulseEvent {
            timestamp,
            seed: seed.clone(),
            cell_type: cell_type.clone(),
        });
        self.activity_log
            .entry(cell_type)
            .or_default()
            .push(timestamp);
    }

    pub fn get_pulse_summary(&self) -> String {
        if self.timeline.is_empty() {
            return "Kein Zellpuls registriert.".to_string();
        }
        let mut summary = vec!["Letzte Zellaktionen:".to_string()];
        for event in self.timeline.iter().rev().take(5) {
            let local_time: DateTime<Local> = DateTime::from(event.timestamp);
            summary.push(format!(
                "{} | {}: {}...",
                local_time.format("%H:%M:%S"),
                event.cell_type,
                &event.seed.chars().take(10).collect::<String>()
            ));
        }
        summary.join("\n")
    }

    pub fn detect_frequency_anomalies(&self) -> Vec<String> {
        let mut anomalies = Vec::new();
        for (cell_type, stamps) in &self.activity_log {
            if stamps.len() < 2 {
                continue;
            }
            let mut deltas = Vec::new();
            for pair in stamps.windows(2) {
                if let [first, second] = pair {
                    deltas.push((*second - *first).num_milliseconds() as f64 / 1000.0);
                }
            }
            if deltas.is_empty() {
                continue;
            }
            let avg: f64 = deltas.iter().sum::<f64>() / deltas.len() as f64;
            if avg < 1.0 {
                anomalies.push(format!("{cell_type}: ungewöhnlich hohe Frequenz"));
            } else if avg > 15.0 {
                anomalies.push(format!("{cell_type}: Aktivität sehr gering"));
            }
        }
        anomalies
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn pulse_tracker_generates_summary() {
        let mut tracker = PulseTracker::new();
        tracker.register_event("seed_a", "NavigatorCell");
        thread::sleep(Duration::from_millis(10));
        tracker.register_event("seed_b", "NavigatorCell");
        let summary = tracker.get_pulse_summary();
        assert!(summary.contains("NavigatorCell"));
        let anomalies = tracker.detect_frequency_anomalies();
        assert!(!anomalies.is_empty());
    }
}
