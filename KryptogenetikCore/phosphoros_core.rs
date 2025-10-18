// ============================================================================
// PHOSPHOROS - Kryptogenetik Core Library
// ============================================================================
// Prä-holographisches System zur 5D-Skalarprojektion
// Integration von Triton, Metatron-IUL und Gabriel Cells
//
// Author: PHOSPHOROS Project Team
// Version: 1.0.0
// ============================================================================

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// ============================================================================
// CORE TYPES - 5D Geometrie und Spektrale Signaturen
// ============================================================================

/// 5-dimensionaler Punkt im Informationsraum
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point5D {
    pub coords: [f64; 5],
}

impl Point5D {
    pub fn new(x: f64, y: f64, z: f64, w: f64, v: f64) -> Self {
        Self { coords: [x, y, z, w, v] }
    }

    pub fn zero() -> Self {
        Self { coords: [0.0; 5] }
    }

    pub fn norm(&self) -> f64 {
        self.coords.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let n = self.norm();
        if n < 1e-12 {
            return *self;
        }
        Self {
            coords: self.coords.map(|x| x / n),
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.coords.iter().zip(other.coords.iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    pub fn distance(&self, other: &Self) -> f64 {
        self.coords.iter().zip(other.coords.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut coords = [0.0; 5];
        for i in 0..5 {
            coords[i] = self.coords[i] + other.coords[i];
        }
        Self { coords }
    }

    pub fn scale(&self, scalar: f64) -> Self {
        Self {
            coords: self.coords.map(|x| x * scalar),
        }
    }
}

/// Spektrale Signatur σ = (ψ, ρ, ω) eines Punktes
#[derive(Debug, Clone, Copy)]
pub struct SpectralSignature {
    pub psi: f64,   // Kohärenz/Semantik
    pub rho: f64,   // Dichte/Struktur
    pub omega: f64, // Frequenz/Phase
}

impl SpectralSignature {
    pub fn new(psi: f64, rho: f64, omega: f64) -> Self {
        Self {
            psi: psi.clamp(0.0, 1.0),
            rho: rho.clamp(0.0, 1.0),
            omega: omega.clamp(0.0, 1.0),
        }
    }

    /// Resonanz-Score D = ψ·ρ·ω
    pub fn resonance(&self) -> f64 {
        self.psi * self.rho * self.omega
    }

    /// Gesamtenergie (alternative Metrik)
    pub fn energy(&self) -> f64 {
        (self.psi.powi(2) + self.rho.powi(2) + self.omega.powi(2)).sqrt()
    }
}

// ============================================================================
// METATRON GEOMETRY - 13-Node Sacred Geometry → 5D Projektion
// ============================================================================

pub struct MetatronGeometry {
    nodes_3d: [[f64; 3]; 13],
    projection_5d: [Point5D; 13],
}

impl MetatronGeometry {
    pub fn new() -> Self {
        let sqrt3 = 3.0_f64.sqrt();
        
        // 13 kanonische Metatron-Knoten
        let nodes_3d = [
            [0.0, 0.0, 0.0],              // 0: Center
            [1.0, 0.0, 0.0],              // 1: H1
            [0.5, sqrt3/2.0, 0.0],        // 2: H2
            [-0.5, sqrt3/2.0, 0.0],       // 3: H3
            [-1.0, 0.0, 0.0],             // 4: H4
            [-0.5, -sqrt3/2.0, 0.0],      // 5: H5
            [0.5, -sqrt3/2.0, 0.0],       // 6: H6
            [0.5, 0.5, 0.5],              // 7: Q1
            [0.5, 0.5, -0.5],             // 8: Q2
            [0.5, -0.5, 0.5],             // 9: Q3
            [0.5, -0.5, -0.5],            // 10: Q4
            [-0.5, 0.5, 0.5],             // 11: Q5
            [-0.5, 0.5, -0.5],            // 12: Q6
        ];

        // Projektion nach 5D: [x, y, z, r, φ]
        let projection_5d = nodes_3d.map(|node| {
            let x = node[0];
            let y = node[1];
            let z = node[2];
            let r = (x*x + y*y + z*z).sqrt();
            let phi = y.atan2(x);
            Point5D::new(x, y, z, r, phi).normalize()
        });

        Self { nodes_3d, projection_5d }
    }

    /// Embette beliebiges Objekt via Metatron-Topologie
    pub fn embed_object(&self, obj_hash: u64) -> Point5D {
        let weights = self.hash_to_weights(obj_hash);
        
        let mut embedding = Point5D::zero();
        for (i, weight) in weights.iter().enumerate() {
            let node = self.projection_5d[i];
            embedding = embedding.add(&node.scale(*weight));
        }
        
        embedding.normalize()
    }

    /// Hash → 13 Gewichte (Probability Distribution)
    fn hash_to_weights(&self, h: u64) -> [f64; 13] {
        let mut weights = [0.0; 13];
        for i in 0..13 {
            let seed = h.wrapping_add(i as u64 * 0x9e3779b9);
            weights[i] = ((seed as f64 * 0.0001).sin()).abs();
        }
        
        let sum: f64 = weights.iter().sum();
        weights.iter_mut().for_each(|w| *w /= sum + 1e-10);
        weights
    }

    pub fn get_node_5d(&self, index: usize) -> Point5D {
        self.projection_5d[index]
    }
}

// ============================================================================
// GABRIEL CELL - Metatron Resonite (ψ, ρ, ω Dynamik)
// ============================================================================

pub struct GabrielCell {
    pub id: String,
    pub psi: f64,
    pub rho: f64,
    pub omega: f64,
    pub output: f64,
    pub learn_rate: f64,
    history: Vec<f64>,
}

impl GabrielCell {
    pub fn new(id: String) -> Self {
        Self {
            id,
            psi: 1.0,
            rho: 1.0,
            omega: 1.0,
            output: 1.0,
            learn_rate: 0.12,
            history: Vec::new(),
        }
    }

    /// Evaluiere Cell auf 5D-Embedding
    pub fn evaluate(&mut self, embedding: &Point5D) -> f64 {
        let input = embedding.coords.iter().sum::<f64>();
        
        // GabrielCell Aktivierung
        self.psi = (1.0 - self.learn_rate) * self.psi + self.learn_rate * input;
        self.output = self.psi * self.rho * self.omega;
        self.history.push(self.output);
        
        (self.output / 10.0).clamp(0.0, 1.0)
    }

    /// Hebbian-like Feedback
    pub fn feedback(&mut self, target: f64) {
        let err = target - self.output;
        self.psi += self.learn_rate * err;
        self.rho += self.learn_rate * err.tanh();
        self.omega += self.learn_rate * err.sin();
        
        self.psi = self.psi.clamp(0.01, 10.0);
        self.rho = self.rho.clamp(0.01, 10.0);
        self.omega = self.omega.clamp(0.01, 10.0);
    }

    pub fn get_signature(&self) -> SpectralSignature {
        SpectralSignature::new(
            self.psi / 10.0,
            self.rho / 10.0,
            self.omega / 10.0,
        )
    }
}

// ============================================================================
// INFOGENOM - Netzwerk von Gabriel Cells
// ============================================================================

pub struct Infogenom {
    pub id: String,
    cells: Vec<GabrielCell>,
    coupling_strength: f64,
}

impl Infogenom {
    pub fn new(id: String, num_cells: usize) -> Self {
        let cells = (0..num_cells)
            .map(|i| GabrielCell::new(format!("{}:cell_{}", id, i)))
            .collect();

        Self {
            id,
            cells,
            coupling_strength: 0.5,
        }
    }

    /// Evaluiere alle Cells mit Coupling
    pub fn evaluate(&mut self, embedding: &Point5D) -> Vec<f64> {
        let mut scores = Vec::new();
        
        for i in 0..self.cells.len() {
            let score = self.cells[i].evaluate(embedding);
            scores.push(score);
            
            // Couple zu Nachbarn
            if i > 0 {
                let neighbor_output = self.cells[i-1].output;
                self.cells[i].feedback(neighbor_output * self.coupling_strength);
            }
        }
        
        scores
    }

    /// Gewichtete Gesamtresonanz
    pub fn total_resonance(&mut self, embedding: &Point5D) -> f64 {
        let scores = self.evaluate(embedding);
        scores.iter().sum::<f64>() / scores.len() as f64
    }

    /// Contract Keyspace (Solve-Coagula)
    pub fn contract_keyspace(&mut self, keyspace: Vec<Point5D>, threshold: f64) -> Vec<Point5D> {
        keyspace.into_iter()
            .filter(|emb| self.total_resonance(emb) >= threshold)
            .collect()
    }
}

// ============================================================================
// TRITON SPIRAL GENERATOR - 5D Golden Spiral mit Ouroboros
// ============================================================================

pub struct TritonSpiralGenerator {
    step: usize,
    radius_base: f64,
    phi_golden: f64,
    alpha_learning: f64,
    current_pos: Point5D,
    ouroboros_momentum: Point5D,
    best_signature: Option<SpectralSignature>,
    projection_matrix: [[f64; 5]; 5],
}

impl TritonSpiralGenerator {
    pub fn new(seed: u64) -> Self {
        let phi_golden = std::f64::consts::PI * (3.0 - 5.0_f64.sqrt());
        
        // Deterministisch generierte Projektionsmatrix
        let mut projection_matrix = [[0.0; 5]; 5];
        let mut h = seed;
        for i in 0..5 {
            for j in 0..5 {
                h = h.wrapping_mul(0x9e3779b97f4a7c15);
                projection_matrix[i][j] = ((h as f64 / u64::MAX as f64) - 0.5) * 2.0;
            }
        }

        Self {
            step: 0,
            radius_base: 0.015,
            phi_golden,
            alpha_learning: 0.12,
            current_pos: Point5D::zero(),
            ouroboros_momentum: Point5D::zero(),
            best_signature: None,
            projection_matrix,
        }
    }

    /// Generiere nächsten Spiral-Punkt
    pub fn generate_next(&mut self) -> Point5D {
        let r = self.radius_base * ((self.step + 1) as f64).sqrt();
        let theta = self.step as f64 * self.phi_golden;

        // 2D Basis
        let u = [r * theta.cos(), r * theta.sin()];

        // Erweitert auf 5D
        let extended = [
            u[0],
            u[1],
            0.5 * (theta * 0.5).sin(),
            0.5 * (theta * 0.5).cos(),
            (self.step as f64 / 100.0) - 0.5,
        ];

        // Projektion mit Matrix
        let mut base_coords = [0.0; 5];
        for i in 0..5 {
            for j in 0..5 {
                base_coords[i] += extended[j] * self.projection_matrix[i][j];
            }
        }

        // Ouroboros-Drift anwenden
        let ouroboros_drift = self.ouroboros_momentum.scale(self.alpha_learning);
        let mut coords = Point5D { coords: base_coords };
        coords = coords.add(&ouroboros_drift);
        coords = coords.normalize();

        self.current_pos = coords;
        self.step += 1;

        coords
    }

    /// Update Ouroboros-Momentum via Feedback
    pub fn update_ouroboros(&mut self, signature: SpectralSignature) {
        let gradient = Point5D::new(
            signature.psi - 0.5,
            signature.rho - 0.5,
            signature.omega - 0.5,
            signature.resonance() - 0.5,
            (self.step as f64 * 0.1).sin() * signature.resonance(),
        );

        let decay = 0.9;
        let old = self.ouroboros_momentum.scale(decay);
        let new = gradient.scale(1.0 - decay);
        self.ouroboros_momentum = old.add(&new);

        // Track Best
        if self.best_signature.is_none() || 
           signature.resonance() > self.best_signature.unwrap().resonance() {
            self.best_signature = Some(signature);
        }
    }

    pub fn get_best_resonance(&self) -> f64 {
        self.best_signature.map(|s| s.resonance()).unwrap_or(0.0)
    }
}

// ============================================================================
// INFORMATION ALCHEMY EVALUATOR - Spektralfeld-Messung
// ============================================================================

pub struct InformationAlchemyEvaluator {
    target_embedding: Option<Point5D>,
    coherence_window: usize,
    history: Vec<Point5D>,
}

impl InformationAlchemyEvaluator {
    pub fn new(target_embedding: Option<Point5D>) -> Self {
        Self {
            target_embedding,
            coherence_window: 8,
            history: Vec::new(),
        }
    }

    /// Evaluiere spektrale Signatur eines Punktes
    pub fn evaluate(&mut self, point: &Point5D) -> SpectralSignature {
        // ψ (Kohärenz): Ähnlichkeit zu Zielvektor
        let psi = if let Some(target) = &self.target_embedding {
            let dot = point.dot(target);
            let norm_product = point.norm() * target.norm() + 1e-12;
            (dot / norm_product).max(0.0)
        } else {
            1.0 - point.coords.iter()
                .map(|&x| (x - 0.5).powi(2))
                .sum::<f64>()
                .sqrt()
        };

        // ρ (Dichte): Strukturelle Kohärenz über Historie
        let rho = if self.history.len() >= 2 {
            let recent: Vec<_> = self.history.iter()
                .rev()
                .take(self.coherence_window)
                .collect();
            let distances: Vec<_> = recent.iter()
                .map(|p| point.distance(p))
                .collect();
            let mean_dist = distances.iter().sum::<f64>() / distances.len() as f64;
            1.0 / (1.0 + mean_dist)
        } else {
            point.norm()
        };

        // ω (Frequenz): Phasische Passung
        let omega = if self.history.len() >= 2 {
            let last = &self.history[self.history.len() - 1];
            let prev = &self.history[self.history.len() - 2];
            
            let direction = point.add(&last.scale(-1.0));
            let prev_direction = last.add(&prev.scale(-1.0));
            
            let corr = direction.dot(&prev_direction) / 
                      (direction.norm() * prev_direction.norm() + 1e-12);
            (corr + 1.0) / 2.0
        } else {
            0.5
        };

        self.history.push(*point);
        if self.history.len() > 2 * self.coherence_window {
            self.history.drain(0..self.coherence_window);
        }

        SpectralSignature::new(psi, rho, omega)
    }
}

// ============================================================================
// SOLVE COAGULA DECISION - Merkaba Gate Logic
// ============================================================================

pub struct SolveCoagulaDecision {
    phi_threshold: f64,
    theta_dynamic: f64,
    theta_ema_gamma: f64,
    spike_delta: f64,
    gate_open: bool,
    phase_history: Vec<(String, f64, f64)>,
}

impl SolveCoagulaDecision {
    pub fn new() -> Self {
        Self {
            phi_threshold: 0.5,
            theta_dynamic: 0.5,
            theta_ema_gamma: 0.2,
            spike_delta: 0.05,
            gate_open: false,
            phase_history: Vec::new(),
        }
    }

    /// Evaluate Phase (flüssig/fest) basierend auf Ordnungsmaß Φ
    pub fn evaluate_phase(&mut self, signature: SpectralSignature) -> String {
        let d = signature.resonance();
        let kappa = 5.0;
        let theta_adaptive = 0.125;
        
        // Ordnungsmaß Φ = σ(κ[D - Θ])
        let phi = 1.0 / (1.0 + (-kappa * (d - theta_adaptive)).exp());
        
        let phase = if phi >= self.phi_threshold { "fest" } else { "flüssig" };
        
        self.phase_history.push((phase.to_string(), phi, d));
        phase.to_string()
    }

    /// Proof of Resonance Check
    pub fn check_proof_of_resonance(&self, signature: SpectralSignature) -> bool {
        signature.psi >= 0.1 && 
        signature.rho >= 0.1 && 
        signature.omega >= 0.1 && 
        signature.omega <= 0.9
    }

    /// Merkaba-Gate Evaluierung
    pub fn check_merkaba_gate(&mut self, signature: SpectralSignature) -> bool {
        let por_condition = self.check_proof_of_resonance(signature);
        let phase = self.evaluate_phase(signature);
        let phase_condition = phase == "fest";
        let spike_condition = signature.resonance() > (self.theta_dynamic + self.spike_delta);
        
        // Gate-Entscheidung
        self.gate_open = por_condition && phase_condition && spike_condition;
        
        // Update dynamische Schwelle (EMA)
        self.theta_dynamic = (1.0 - self.theta_ema_gamma) * self.theta_dynamic + 
                            self.theta_ema_gamma * signature.resonance();
        
        self.gate_open
    }

    pub fn is_gate_open(&self) -> bool {
        self.gate_open
    }
}

// ============================================================================
// QDASH EXPLORER - Vollständige Exploration Engine
// ============================================================================

pub struct QDASHExplorer {
    infogenom: Infogenom,
    spiral: TritonSpiralGenerator,
    evaluator: InformationAlchemyEvaluator,
    decision: SolveCoagulaDecision,
    max_steps: usize,
    exploration_path: Vec<(Point5D, SpectralSignature)>,
    crystals: Vec<TICCrystal>,
}

pub struct TICCrystal {
    pub id: String,
    pub center: Point5D,
    pub stability: f64,
    pub formation_step: usize,
}

impl QDASHExplorer {
    pub fn new(infogenom: Infogenom, seed: u64, max_steps: usize) -> Self {
        Self {
            infogenom,
            spiral: TritonSpiralGenerator::new(seed),
            evaluator: InformationAlchemyEvaluator::new(None),
            decision: SolveCoagulaDecision::new(),
            max_steps,
            exploration_path: Vec::new(),
            crystals: Vec::new(),
        }
    }

    /// Führe Exploration aus
    pub fn explore(&mut self) -> ExplorationResult {
        for _i in 0..self.max_steps {
            let coords = self.spiral.generate_next();
            let signature = self.evaluator.evaluate(&coords);
            
            let gate_open = self.decision.check_merkaba_gate(signature);
            
            if gate_open {
                self.attempt_crystallization(&coords, signature);
            }
            
            self.spiral.update_ouroboros(signature);
            self.exploration_path.push((coords, signature));
        }

        ExplorationResult {
            steps: self.exploration_path.len(),
            best_resonance: self.spiral.get_best_resonance(),
            num_crystals: self.crystals.len(),
            final_position: self.spiral.current_pos,
        }
    }

    fn attempt_crystallization(&mut self, center: &Point5D, signature: SpectralSignature) {
        if signature.resonance() > 0.7 {
            let crystal = TICCrystal {
                id: format!("TIC_{}", self.crystals.len()),
                center: *center,
                stability: signature.resonance(),
                formation_step: self.exploration_path.len(),
            };
            self.crystals.push(crystal);
        }
    }

    pub fn get_crystals(&self) -> &[TICCrystal] {
        &self.crystals
    }
}

pub struct ExplorationResult {
    pub steps: usize,
    pub best_resonance: f64,
    pub num_crystals: usize,
    pub final_position: Point5D,
}

// ============================================================================
// HIGH-LEVEL BRIDGE API - Integration für PHOSPHOROS
// ============================================================================

pub struct PhosphorosCore {
    metatron: MetatronGeometry,
    infogenoms: HashMap<String, Infogenom>,
}

impl PhosphorosCore {
    pub fn new() -> Self {
        Self {
            metatron: MetatronGeometry::new(),
            infogenoms: HashMap::new(),
        }
    }

    /// Embette BIP39 Seed-Phrase in 5D
    pub fn embed_seed_phrase(&self, words: &[&str]) -> Vec<Point5D> {
        words.iter()
            .map(|word| {
                let hash = self.hash_word(word);
                self.metatron.embed_object(hash)
            })
            .collect()
    }

    /// Erstelle Infogenom
    pub fn create_infogenom(&mut self, id: String, num_cells: usize) {
        self.infogenoms.insert(id.clone(), Infogenom::new(id, num_cells));
    }

    /// Exploriere mit QDASH
    pub fn explore_keyspace(&mut self, 
                           infogenom_id: &str, 
                           seed: u64, 
                           max_steps: usize) -> Option<ExplorationResult> {
        let infogenom = self.infogenoms.remove(infogenom_id)?;
        let mut explorer = QDASHExplorer::new(infogenom, seed, max_steps);
        let result = explorer.explore();
        
        // Return infogenom
        self.infogenoms.insert(infogenom_id.to_string(), explorer.infogenom);
        
        Some(result)
    }

    fn hash_word(&self, word: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        word.hash(&mut hasher);
        hasher.finish()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point5d_operations() {
        let p1 = Point5D::new(1.0, 0.0, 0.0, 0.0, 0.0);
        let p2 = Point5D::new(0.0, 1.0, 0.0, 0.0, 0.0);
        
        assert!((p1.norm() - 1.0).abs() < 1e-10);
        assert!((p1.dot(&p2)).abs() < 1e-10);
        assert!((p1.distance(&p2) - 2.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_spectral_signature() {
        let sig = SpectralSignature::new(0.8, 0.9, 0.7);
        assert!((sig.resonance() - 0.504).abs() < 1e-3);
    }

    #[test]
    fn test_metatron_geometry() {
        let metatron = MetatronGeometry::new();
        let center = metatron.get_node_5d(0);
        assert!(center.norm() > 0.0);
    }

    #[test]
    fn test_gabriel_cell() {
        let mut cell = GabrielCell::new("test".to_string());
        let point = Point5D::new(0.5, 0.5, 0.5, 0.5, 0.5);
        
        let output = cell.evaluate(&point);
        assert!(output >= 0.0 && output <= 1.0);
    }

    #[test]
    fn test_triton_spiral() {
        let mut spiral = TritonSpiralGenerator::new(42);
        let p1 = spiral.generate_next();
        let p2 = spiral.generate_next();
        
        assert!(p1.distance(&p2) > 0.0);
    }

    #[test]
    fn test_phosphoros_core() {
        let mut core = PhosphorosCore::new();
        
        let words = vec!["abandon", "ability", "able"];
        let embeddings = core.embed_seed_phrase(&words);
        
        assert_eq!(embeddings.len(), 3);
        
        core.create_infogenom("test_infogenom".to_string(), 4);
        let result = core.explore_keyspace("test_infogenom", 42, 100);
        
        assert!(result.is_some());
        let res = result.unwrap();
        assert!(res.steps > 0);
    }
}
