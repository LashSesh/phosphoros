use chrono::Local;

#[derive(Debug, Default, Clone)]
pub struct ResonanceOverlay {
    entries: Vec<String>,
}

impl ResonanceOverlay {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn log(
        &mut self,
        source: impl AsRef<str>,
        action: impl AsRef<str>,
        detail: Option<impl AsRef<str>>,
    ) {
        let timestamp = Local::now().format("%H:%M:%S");
        let detail_text = detail.map(|d| d.as_ref().to_string()).unwrap_or_default();
        let entry = if detail_text.is_empty() {
            format!("[{timestamp}] {}: {}", source.as_ref(), action.as_ref())
        } else {
            format!(
                "[{timestamp}] {}: {} {}",
                source.as_ref(),
                action.as_ref(),
                detail_text
            )
        };
        self.entries.push(entry);
    }

    pub fn get_overlay(&self, limit: usize) -> String {
        if self.entries.is_empty() {
            return "Keine Systemaktivität aufgezeichnet.".to_string();
        }
        let start = self.entries.len().saturating_sub(limit);
        self.entries[start..].join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlay_logs_entries() {
        let mut overlay = ResonanceOverlay::new();
        overlay.log("SeedCore", "Analyse", Some("OK"));
        overlay.log("SeedCore", "Analyse", None::<&str>);
        let output = overlay.get_overlay(5);
        assert!(output.contains("SeedCore"));
    }
}
