use rand::{seq::SliceRandom, Rng};
use sha2::{Digest, Sha256};
use std::thread;
use std::time::Duration;

use crate::error::{OuroborosError, Result};
use crate::memory::collective_field::CollectiveField;
use crate::memory::resonance_return::ResonanceReturn;
use crate::network::scorpio_bridge::ScorpioBridge;
use crate::visual::resonance_heatmap::Heatmap;

const DEFAULT_CYCLES: usize = 30;

#[derive(Debug)]
pub struct AutoSeedCascade {
    name: String,
    wordlist: Vec<String>,
    bridge: ScorpioBridge,
    returner: ResonanceReturn,
    heatmap: Heatmap,
    field: CollectiveField,
    cycles: usize,
}

impl AutoSeedCascade {
    pub fn new(wordlist: Vec<String>) -> Result<Self> {
        Self::builder(wordlist).build()
    }

    pub fn builder(wordlist: Vec<String>) -> AutoSeedCascadeBuilder {
        AutoSeedCascadeBuilder::new(wordlist)
    }

    pub fn run(&self) -> Result<()> {
        println!("[{}] Starte automatisierte Seed-Kaskade...", self.name);
        for _ in 0..self.cycles {
            let seed = self.generate_seed()?;
            let vector = self.generate_vector();
            let address = self.address_from_seed(&seed);
            let balance = self.bridge.check_seed(&address)?;
            self.heatmap.log(&address, balance)?;
            self.field.log_point(&seed, &vector, &address, balance)?;
            if let Some(value) = balance {
                if value > 0.0 {
                    self.returner.log_hit(&seed, &address, Some(value))?;
                }
            }
            thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }

    fn generate_seed(&self) -> Result<String> {
        if self.wordlist.len() < 12 {
            return Err(OuroborosError::Config(
                "Wordlist muss mindestens 12 Einträge enthalten".to_string(),
            ));
        }
        let mut rng = rand::thread_rng();
        let words: Vec<&String> = self.wordlist.choose_multiple(&mut rng, 12).collect();
        Ok(words
            .into_iter()
            .map(|w| w.as_str())
            .collect::<Vec<&str>>()
            .join(" "))
    }

    fn generate_vector(&self) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        (0..12).map(|_| rng.gen::<f64>()).collect()
    }

    fn address_from_seed(&self, seed: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(seed.as_bytes());
        let digest = hasher.finalize();
        let hex = hex::encode(digest);
        hex[..34].to_string()
    }
}

pub struct AutoSeedCascadeBuilder {
    name: String,
    wordlist: Vec<String>,
    cycles: usize,
    provider: String,
}

impl AutoSeedCascadeBuilder {
    pub fn new(wordlist: Vec<String>) -> Self {
        Self {
            name: "SEED-CASCADE".to_string(),
            wordlist,
            cycles: DEFAULT_CYCLES,
            provider: "https://cloudflare-eth.com".to_string(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn cycles(mut self, cycles: usize) -> Self {
        self.cycles = cycles;
        self
    }

    pub fn provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = provider.into();
        self
    }

    pub fn build(self) -> Result<AutoSeedCascade> {
        let bridge = ScorpioBridge::new(&self.provider)?;
        Ok(AutoSeedCascade {
            name: self.name,
            wordlist: self.wordlist,
            bridge,
            returner: ResonanceReturn::default(),
            heatmap: Heatmap::default(),
            field: CollectiveField::default(),
            cycles: self.cycles,
        })
    }
}
