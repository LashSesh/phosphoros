use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;
use serde::Serialize;

use crate::core::auto_seed_cascade::AutoSeedCascade;
use crate::crypto::seed_crypto::SeedCrypto;
use crate::error::{OuroborosError, Result};
use crate::memory::collective_field::CollectiveField;
use crate::memory::pattern_memory::PatternMemory;
use crate::memory::resonance_return::ResonanceReturn;
use crate::network::scorpio_bridge::ScorpioBridge;
use crate::utils::path_recorder::PathRecorder;
use crate::visual::resonance_heatmap::log_address_heat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Auto,
    Memory,
    Hybrid,
}

impl Mode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Auto => "auto",
            Mode::Memory => "memory",
            Mode::Hybrid => "hybrid",
        }
    }
}

impl std::str::FromStr for Mode {
    type Err = OuroborosError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(Mode::Auto),
            "memory" => Ok(Mode::Memory),
            "hybrid" => Ok(Mode::Hybrid),
            other => Err(OuroborosError::Config(format!(
                "unbekannter Modus: {other}"
            ))),
        }
    }
}

#[derive(Debug)]
pub struct TritonCore {
    name: String,
    mode: Mode,
    cycles: usize,
    wordlist: Vec<String>,
    memory: PatternMemory,
    field: CollectiveField,
    signature_path: PathBuf,
}

impl TritonCore {
    pub fn new(wordlist: Vec<String>) -> Self {
        Self {
            name: "TRITON".to_string(),
            mode: Mode::Auto,
            cycles: 40,
            wordlist,
            memory: PatternMemory::default(),
            field: CollectiveField::default(),
            signature_path: PathBuf::from("triton_signature.json"),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    pub fn cycles(mut self, cycles: usize) -> Self {
        self.cycles = cycles;
        self
    }

    pub fn run(&self) -> Result<()> {
        println!(
            "[{}] Initialisiert in Modus: {}",
            self.name,
            self.mode.as_str()
        );
        match self.mode {
            Mode::Auto => {
                self.run_auto()?;
                self.update_signature()?;
            }
            Mode::Memory => self.run_memory()?,
            Mode::Hybrid => self.run_hybrid()?,
        }
        Ok(())
    }

    fn run_auto(&self) -> Result<()> {
        AutoSeedCascade::builder(self.wordlist.clone())
            .name(format!("{}-AUTO", self.name))
            .cycles(self.cycles)
            .build()?
            .run()
    }

    fn run_memory(&self) -> Result<()> {
        let cells = self.get_known_cells()?;
        if cells.is_empty() {
            println!("[Memory] Keine Zellpfade vorhanden.");
            return Ok(());
        }
        let mut rng = rand::thread_rng();
        let cell = cells
            .iter()
            .choose(&mut rng)
            .cloned()
            .ok_or_else(|| OuroborosError::Config("Keine Zellpfade vorhanden".to_string()))?;
        if let Some(pattern) = self.memory.repeat_path(&cell, 0.2)? {
            self.simulate_pattern_drill(&pattern, &cell)?;
        } else {
            println!("[Memory] Leeres Muster.");
        }
        Ok(())
    }

    fn run_hybrid(&self) -> Result<()> {
        let cells = self.get_known_cells()?;
        if cells.len() < 2 {
            println!("[Hybrid] Zu wenige Musterpfade.");
            return Ok(());
        }
        let mut rng = rand::thread_rng();
        let selected: Vec<String> = cells.choose_multiple(&mut rng, 2).cloned().collect();
        let a = &selected[0];
        let b = &selected[1];
        if let Some(pattern) = self.memory.combine_paths(a, b, 0.15)? {
            self.simulate_pattern_drill(&pattern, &format!("{}+{}", a, b))?;
        }
        Ok(())
    }

    fn simulate_pattern_drill(&self, pattern: &[Vec<f64>], origin: &str) -> Result<()> {
        let crypto = SeedCrypto::default();
        let bridge = ScorpioBridge::new("https://cloudflare-eth.com")?;
        let returner = ResonanceReturn::default();
        let recorder = PathRecorder::default();
        let mut rng = rand::thread_rng();

        for vector in pattern {
            let words: Vec<&String> = self.wordlist.choose_multiple(&mut rng, 12).collect();
            let seed = words
                .into_iter()
                .map(|w| w.as_str())
                .collect::<Vec<&str>>()
                .join(" ");
            let details = crypto.seed_to_address(&seed)?;
            let balance = bridge.check_seed(&details.address)?;
            log_address_heat(&details.address, balance)?;
            self.field
                .log_point(&seed, vector, &details.address, balance)?;
            recorder.log_step(origin, vector, &details.seed, &details.address, balance)?;
            if let Some(value) = balance {
                if value > 0.0 {
                    returner.log_hit(&details.seed, &details.address, Some(value))?;
                }
            }
        }
        Ok(())
    }

    fn get_known_cells(&self) -> Result<Vec<String>> {
        let recorder = PathRecorder::default();
        let entries = recorder.load_all()?;
        let cells: HashSet<String> = entries.into_iter().map(|entry| entry.cell).collect();
        Ok(cells.into_iter().collect())
    }

    fn update_signature(&self) -> Result<()> {
        #[derive(Serialize)]
        struct Signature<'a> {
            identity: &'a str,
            mode: &'a str,
            recorded_vectors: usize,
            state: &'a str,
        }

        let vectors = self.field.load_vectors()?;
        let signature = Signature {
            identity: &self.name,
            mode: self.mode.as_str(),
            recorded_vectors: vectors.len(),
            state: "active",
        };
        let json = serde_json::to_string_pretty(&signature)?;
        fs::write(&self.signature_path, json)?;
        Ok(())
    }
}
