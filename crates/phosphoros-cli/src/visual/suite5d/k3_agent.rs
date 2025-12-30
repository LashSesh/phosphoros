use std::collections::HashMap;

use super::SpectralLink;

#[derive(Debug, Default, Clone)]
pub struct K3ResonanceAnalyzer {
    patterns: HashMap<String, usize>,
}

impl K3ResonanceAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn observe(&mut self, fabric_links: &[SpectralLink]) {
        let mut stats: HashMap<String, usize> = HashMap::new();
        for link in fabric_links {
            for seed in [&link.seed_a, &link.seed_b] {
                let prefix = seed.chars().take(6).collect::<String>();
                *stats.entry(prefix).or_insert(0) += 1;
            }
        }
        self.patterns = stats.into_iter().filter(|(_, count)| *count >= 2).collect();
    }

    pub fn report(&self) -> Vec<String> {
        if self.patterns.is_empty() {
            return vec!["Keine dominanten Muster erkannt.".to_string()];
        }
        let mut entries: Vec<_> = self.patterns.iter().collect();
        entries.sort_by(|a, b| b.1.cmp(a.1));
        entries
            .into_iter()
            .map(|(prefix, count)| format!("Cluster {prefix} → {count}x Resonanzpunkte"))
            .collect()
    }

    pub fn patterns(&self) -> &HashMap<String, usize> {
        &self.patterns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyzer_detects_patterns() {
        let mut analyzer = K3ResonanceAnalyzer::new();
        let links = vec![
            SpectralLink {
                seed_a: "abcdef1234".into(),
                seed_b: "abcdef6789".into(),
                distance: 0.3,
            },
            SpectralLink {
                seed_a: "abcdefaaaa".into(),
                seed_b: "bbbbbb1234".into(),
                distance: 0.5,
            },
        ];
        analyzer.observe(&links);
        assert!(analyzer.patterns().contains_key("abcdef"));
    }
}
