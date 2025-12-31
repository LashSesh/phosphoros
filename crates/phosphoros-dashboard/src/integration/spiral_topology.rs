//! 5D Spiral Topology Module
//!
//! Implementation of the axiomatic 5D spiral topology framework based on
//! Sebastian Klemm's "Axiomatic 5D Spiral Topology: A Geometric-Cybernetic Framework"
//!
//! Key concepts:
//! - 5D state space X = R³ × R² (spatial + internal)
//! - Spiral paths with cylindrical coordinates (r, φ, h)
//! - Resonance fields Ψ = {ψ, ρ, ω}
//! - Resonance invariant: D = ψ·ρ·ω

use std::f64::consts::PI;

/// A point in 5D state space
/// s = (x₁, x₂, x₃, x₄, x₅)ᵀ ∈ X
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct State5D {
    /// Spatial component x₁
    pub x1: f64,
    /// Spatial component x₂
    pub x2: f64,
    /// Spatial component x₃ (height)
    pub x3: f64,
    /// Internal component x₄
    pub x4: f64,
    /// Internal component x₅
    pub x5: f64,
}

impl State5D {
    /// Create a new 5D state
    pub fn new(x1: f64, x2: f64, x3: f64, x4: f64, x5: f64) -> Self {
        Self { x1, x2, x3, x4, x5 }
    }

    /// Create zero state
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0, 0.0)
    }

    /// Create from cylindrical coordinates (r, φ, h) and internal (u, v)
    pub fn from_cylindrical(r: f64, phi: f64, h: f64, u: f64, v: f64) -> Self {
        Self {
            x1: r * phi.cos(),
            x2: r * phi.sin(),
            x3: h,
            x4: u,
            x5: v,
        }
    }

    /// Get spatial projection π_spatial: X → R³
    pub fn spatial(&self) -> (f64, f64, f64) {
        (self.x1, self.x2, self.x3)
    }

    /// Get internal projection π_internal: X → R²
    pub fn internal(&self) -> (f64, f64) {
        (self.x4, self.x5)
    }

    /// Get cylindrical coordinates (r, φ, h)
    pub fn cylindrical(&self) -> (f64, f64, f64) {
        let r = (self.x1.powi(2) + self.x2.powi(2)).sqrt();
        let phi = self.x2.atan2(self.x1);
        let h = self.x3;
        (r, phi, h)
    }

    /// Euclidean norm ||s||
    pub fn norm(&self) -> f64 {
        (self.x1.powi(2) + self.x2.powi(2) + self.x3.powi(2)
         + self.x4.powi(2) + self.x5.powi(2)).sqrt()
    }

    /// Euclidean distance between two states
    pub fn distance(&self, other: &State5D) -> f64 {
        ((self.x1 - other.x1).powi(2)
         + (self.x2 - other.x2).powi(2)
         + (self.x3 - other.x3).powi(2)
         + (self.x4 - other.x4).powi(2)
         + (self.x5 - other.x5).powi(2)).sqrt()
    }

    /// Convert to array representation
    pub fn to_array(&self) -> [f64; 5] {
        [self.x1, self.x2, self.x3, self.x4, self.x5]
    }

    /// Create from array
    pub fn from_array(arr: [f64; 5]) -> Self {
        Self::new(arr[0], arr[1], arr[2], arr[3], arr[4])
    }
}

impl std::ops::Add for State5D {
    type Output = State5D;
    fn add(self, rhs: State5D) -> State5D {
        State5D::new(
            self.x1 + rhs.x1,
            self.x2 + rhs.x2,
            self.x3 + rhs.x3,
            self.x4 + rhs.x4,
            self.x5 + rhs.x5,
        )
    }
}

impl std::ops::Sub for State5D {
    type Output = State5D;
    fn sub(self, rhs: State5D) -> State5D {
        State5D::new(
            self.x1 - rhs.x1,
            self.x2 - rhs.x2,
            self.x3 - rhs.x3,
            self.x4 - rhs.x4,
            self.x5 - rhs.x5,
        )
    }
}

impl std::ops::Mul<f64> for State5D {
    type Output = State5D;
    fn mul(self, scalar: f64) -> State5D {
        State5D::new(
            self.x1 * scalar,
            self.x2 * scalar,
            self.x3 * scalar,
            self.x4 * scalar,
            self.x5 * scalar,
        )
    }
}

/// Spiral path type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiralType {
    /// Archimedean spiral: r(t) = a + bt (constant spacing)
    Archimedean,
    /// Logarithmic spiral: r(t) = a·e^(bt) (golden ratio)
    Logarithmic,
    /// Generalized spiral with custom functions
    Generalized,
}

/// Parameters for spiral path generation
#[derive(Debug, Clone, Copy)]
pub struct SpiralParams {
    /// Initial radius offset
    pub a: f64,
    /// Radial growth rate
    pub b: f64,
    /// Height growth rate
    pub c: f64,
    /// Internal x₄ growth rate
    pub alpha: f64,
    /// Internal x₅ growth rate
    pub beta: f64,
    /// Spiral type
    pub spiral_type: SpiralType,
}

impl Default for SpiralParams {
    fn default() -> Self {
        Self {
            a: 0.1,
            b: 0.05,
            c: 0.1,
            alpha: 0.03,
            beta: 0.02,
            spiral_type: SpiralType::Archimedean,
        }
    }
}

/// A 5D spiral path γ: I → X
#[derive(Debug, Clone)]
pub struct SpiralPath {
    /// Spiral parameters
    pub params: SpiralParams,
    /// Discrete samples (spiral sequence)
    pub samples: Vec<State5D>,
}

impl SpiralPath {
    /// Create a new spiral path with given parameters
    pub fn new(params: SpiralParams) -> Self {
        Self {
            params,
            samples: Vec::new(),
        }
    }

    /// Generate point at parameter t
    /// γ(t) = (r(t)·cos(φ(t)), r(t)·sin(φ(t)), h(t), x₄(t), x₅(t))
    pub fn evaluate(&self, t: f64) -> State5D {
        let (r, phi, h, x4, x5) = match self.params.spiral_type {
            SpiralType::Archimedean => {
                // r(t) = a + bt, φ(t) = t, h(t) = ct
                let r = self.params.a + self.params.b * t;
                (r, t, self.params.c * t, self.params.alpha * t, self.params.beta * t)
            }
            SpiralType::Logarithmic => {
                // r(t) = a·e^(bt), φ(t) = t
                let r = self.params.a * (self.params.b * t).exp();
                (r, t, self.params.c * t, self.params.alpha * t, self.params.beta * t)
            }
            SpiralType::Generalized => {
                // Custom spiral (default to Archimedean behavior)
                let r = self.params.a + self.params.b * t;
                (r, t, self.params.c * t, self.params.alpha * t, self.params.beta * t)
            }
        };

        State5D::from_cylindrical(r, phi, h, x4, x5)
    }

    /// Generate spiral sequence with n samples over [t_min, t_max]
    pub fn generate_sequence(&mut self, n: usize, t_min: f64, t_max: f64) {
        self.samples.clear();
        let dt = (t_max - t_min) / (n - 1).max(1) as f64;

        for i in 0..n {
            let t = t_min + dt * i as f64;
            self.samples.push(self.evaluate(t));
        }
    }

    /// Get spiral matrix S ∈ R^(n×5)
    pub fn spiral_matrix(&self) -> Vec<[f64; 5]> {
        self.samples.iter().map(|s| s.to_array()).collect()
    }

    /// Compute radial spacing between turns (for Archimedean)
    /// Δr = r(t + 2π) - r(t) = 2πb
    pub fn turn_spacing(&self) -> f64 {
        match self.params.spiral_type {
            SpiralType::Archimedean => 2.0 * PI * self.params.b,
            SpiralType::Logarithmic => {
                // Factor e^(2πb) per turn
                (2.0 * PI * self.params.b).exp()
            }
            SpiralType::Generalized => 2.0 * PI * self.params.b,
        }
    }
}

/// Resonance fields Ψ = {ψ, ρ, ω}
/// These map states to real values: ψ, ρ, ω: X → R
#[derive(Debug, Clone)]
pub struct ResonanceFields {
    /// Weight for ψ component
    pub w_psi: f64,
    /// Weight for ρ component
    pub w_rho: f64,
    /// Weight for ω component
    pub w_omega: f64,
}

impl Default for ResonanceFields {
    fn default() -> Self {
        Self {
            w_psi: 1.0,
            w_rho: 1.0,
            w_omega: 1.0,
        }
    }
}

impl ResonanceFields {
    /// Create new resonance fields with equal weights
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with custom weights
    pub fn with_weights(w_psi: f64, w_rho: f64, w_omega: f64) -> Self {
        Self { w_psi, w_rho, w_omega }
    }

    /// Compute ψ field value at state s
    /// ψ(s) = f(spatial_norm) - activation/intensity
    pub fn psi(&self, s: &State5D) -> f64 {
        let (x, y, z) = s.spatial();
        let spatial_norm = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        // Gaussian-like activation
        (-spatial_norm / 2.0).exp()
    }

    /// Compute ρ field value at state s
    /// ρ(s) = g(internal) - coherence
    pub fn rho(&self, s: &State5D) -> f64 {
        let (u, v) = s.internal();
        // Coherence based on internal coordinates
        let phase = (u.powi(2) + v.powi(2)).sqrt();
        (1.0 + phase.cos()) / 2.0
    }

    /// Compute ω field value at state s
    /// ω(s) = h(mixed) - rhythm/oscillation
    pub fn omega(&self, s: &State5D) -> f64 {
        let (r, phi, _h) = s.cylindrical();
        // Oscillation based on angular position
        let rhythm = (3.0 * phi).sin().abs();
        (rhythm + r.tanh()) / 2.0
    }

    /// Compute resonance D = ψ·ρ·ω (the invariant)
    pub fn resonance(&self, s: &State5D) -> f64 {
        self.psi(s) * self.rho(s) * self.omega(s)
    }

    /// Compute resonance distance between two states
    /// d_R(s, s') = w_ψ|ψ(s) - ψ(s')| + w_ρ|ρ(s) - ρ(s')| + w_ω|ω(s) - ω(s')|
    pub fn resonance_distance(&self, s1: &State5D, s2: &State5D) -> f64 {
        self.w_psi * (self.psi(s1) - self.psi(s2)).abs()
            + self.w_rho * (self.rho(s1) - self.rho(s2)).abs()
            + self.w_omega * (self.omega(s1) - self.omega(s2)).abs()
    }

    /// Check if two states are resonant (distance < epsilon)
    pub fn are_resonant(&self, s1: &State5D, s2: &State5D, epsilon: f64) -> bool {
        self.resonance_distance(s1, s2) < epsilon
    }
}

/// Spiral segment weight function
/// w(K_i) = f(ψ(S_i), ρ(S_i), ω(S_i))
pub fn segment_weight(fields: &ResonanceFields, s: &State5D) -> f64 {
    // Default weight function: product of field values
    fields.psi(s) * fields.rho(s) * fields.omega(s)
}

/// Spiral action functional
/// W((S_i)_i=1^n) = Σ w(K_i)
pub fn spiral_action(fields: &ResonanceFields, sequence: &[State5D]) -> f64 {
    if sequence.len() < 2 {
        return 0.0;
    }

    sequence.iter()
        .take(sequence.len() - 1)
        .map(|s| segment_weight(fields, s))
        .sum()
}

/// Cybernetic feedback dynamics
/// S_{k+1} = Φ(S_k) = S_k + α·F(S_k)
/// where F(s) = λ_ψ∇ψ(s) + λ_ρ∇ρ(s) + λ_ω∇ω(s)
#[derive(Debug, Clone)]
pub struct SpiralDynamics {
    /// Learning/adaptation rate
    pub alpha: f64,
    /// Weight for ψ gradient
    pub lambda_psi: f64,
    /// Weight for ρ gradient
    pub lambda_rho: f64,
    /// Weight for ω gradient
    pub lambda_omega: f64,
    /// Resonance fields
    pub fields: ResonanceFields,
}

impl Default for SpiralDynamics {
    fn default() -> Self {
        Self {
            alpha: 0.1,
            lambda_psi: 1.0,
            lambda_rho: 1.0,
            lambda_omega: 1.0,
            fields: ResonanceFields::default(),
        }
    }
}

impl SpiralDynamics {
    /// Create new spiral dynamics
    pub fn new(alpha: f64) -> Self {
        Self {
            alpha,
            ..Default::default()
        }
    }

    /// Compute numerical gradient of ψ at state s
    fn grad_psi(&self, s: &State5D) -> State5D {
        let eps = 1e-6;
        let mut grad = [0.0; 5];
        let arr = s.to_array();

        for i in 0..5 {
            let mut s_plus = arr;
            let mut s_minus = arr;
            s_plus[i] += eps;
            s_minus[i] -= eps;

            let psi_plus = self.fields.psi(&State5D::from_array(s_plus));
            let psi_minus = self.fields.psi(&State5D::from_array(s_minus));
            grad[i] = (psi_plus - psi_minus) / (2.0 * eps);
        }

        State5D::from_array(grad)
    }

    /// Compute numerical gradient of ρ at state s
    fn grad_rho(&self, s: &State5D) -> State5D {
        let eps = 1e-6;
        let mut grad = [0.0; 5];
        let arr = s.to_array();

        for i in 0..5 {
            let mut s_plus = arr;
            let mut s_minus = arr;
            s_plus[i] += eps;
            s_minus[i] -= eps;

            let rho_plus = self.fields.rho(&State5D::from_array(s_plus));
            let rho_minus = self.fields.rho(&State5D::from_array(s_minus));
            grad[i] = (rho_plus - rho_minus) / (2.0 * eps);
        }

        State5D::from_array(grad)
    }

    /// Compute numerical gradient of ω at state s
    fn grad_omega(&self, s: &State5D) -> State5D {
        let eps = 1e-6;
        let mut grad = [0.0; 5];
        let arr = s.to_array();

        for i in 0..5 {
            let mut s_plus = arr;
            let mut s_minus = arr;
            s_plus[i] += eps;
            s_minus[i] -= eps;

            let omega_plus = self.fields.omega(&State5D::from_array(s_plus));
            let omega_minus = self.fields.omega(&State5D::from_array(s_minus));
            grad[i] = (omega_plus - omega_minus) / (2.0 * eps);
        }

        State5D::from_array(grad)
    }

    /// Compute feedback force F(s) = λ_ψ∇ψ + λ_ρ∇ρ + λ_ω∇ω
    pub fn feedback_force(&self, s: &State5D) -> State5D {
        let g_psi = self.grad_psi(s);
        let g_rho = self.grad_rho(s);
        let g_omega = self.grad_omega(s);

        State5D::new(
            self.lambda_psi * g_psi.x1 + self.lambda_rho * g_rho.x1 + self.lambda_omega * g_omega.x1,
            self.lambda_psi * g_psi.x2 + self.lambda_rho * g_rho.x2 + self.lambda_omega * g_omega.x2,
            self.lambda_psi * g_psi.x3 + self.lambda_rho * g_rho.x3 + self.lambda_omega * g_omega.x3,
            self.lambda_psi * g_psi.x4 + self.lambda_rho * g_rho.x4 + self.lambda_omega * g_omega.x4,
            self.lambda_psi * g_psi.x5 + self.lambda_rho * g_rho.x5 + self.lambda_omega * g_omega.x5,
        )
    }

    /// Apply dynamics operator Φ(s) = s + α·F(s)
    pub fn step(&self, s: &State5D) -> State5D {
        let f = self.feedback_force(s);
        *s + f * self.alpha
    }

    /// Run dynamics for n steps, return trajectory
    pub fn evolve(&self, initial: State5D, n_steps: usize) -> Vec<State5D> {
        let mut trajectory = Vec::with_capacity(n_steps + 1);
        trajectory.push(initial);

        let mut current = initial;
        for _ in 0..n_steps {
            current = self.step(&current);
            trajectory.push(current);
        }

        trajectory
    }

    /// Check for convergence (proof of resonance)
    /// Returns true if |ψ_new - ψ_old| < epsilon
    pub fn proof_of_resonance(&self, s_old: &State5D, s_new: &State5D, epsilon: f64) -> bool {
        let d_old = self.fields.resonance(s_old);
        let d_new = self.fields.resonance(s_new);
        (d_new - d_old).abs() < epsilon
    }
}

/// Spiral coordinate transformation
/// Maps discrete sets to spiral ordering
#[derive(Debug, Clone)]
pub struct SpiralCoordinates {
    /// Sorted states along spiral path
    pub states: Vec<State5D>,
    /// Spiral parameters used
    pub params: SpiralParams,
}

impl SpiralCoordinates {
    /// Create new spiral coordinate system
    pub fn new(params: SpiralParams) -> Self {
        Self {
            states: Vec::new(),
            params,
        }
    }

    /// Assign spiral coordinates to a set of states
    /// Orders states by their projection onto the spiral path
    pub fn assign(&mut self, states: Vec<State5D>) {
        let _path = SpiralPath::new(self.params);

        // Sort states by their "spiral position" (angle + radius-weighted)
        let mut indexed: Vec<(usize, f64)> = states.iter()
            .enumerate()
            .map(|(i, s)| {
                let (r, phi, _) = s.cylindrical();
                // Spiral index: combine angle and radius
                let idx = phi + r / self.params.b.max(0.01);
                (i, idx)
            })
            .collect();

        indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        self.states = indexed.iter()
            .map(|(i, _)| states[*i])
            .collect();
    }

    /// Get the spiral matrix representation
    pub fn matrix(&self) -> Vec<[f64; 5]> {
        self.states.iter().map(|s| s.to_array()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state5d_creation() {
        let s = State5D::new(1.0, 2.0, 3.0, 4.0, 5.0);
        assert_eq!(s.spatial(), (1.0, 2.0, 3.0));
        assert_eq!(s.internal(), (4.0, 5.0));
    }

    #[test]
    fn test_cylindrical_conversion() {
        let s = State5D::from_cylindrical(1.0, 0.0, 1.0, 0.5, 0.5);
        assert!((s.x1 - 1.0).abs() < 1e-10);
        assert!(s.x2.abs() < 1e-10);
    }

    #[test]
    fn test_spiral_path_archimedean() {
        let params = SpiralParams {
            spiral_type: SpiralType::Archimedean,
            a: 0.0,
            b: 1.0 / (2.0 * PI),
            c: 0.1,
            alpha: 0.0,
            beta: 0.0,
        };
        let mut path = SpiralPath::new(params);
        path.generate_sequence(100, 0.0, 4.0 * PI);

        // After 2π, radius should increase by 1
        let s0 = path.evaluate(0.0);
        let s1 = path.evaluate(2.0 * PI);
        let (r0, _, _) = s0.cylindrical();
        let (r1, _, _) = s1.cylindrical();
        assert!((r1 - r0 - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_resonance_fields() {
        let fields = ResonanceFields::default();
        let s = State5D::new(0.0, 0.0, 0.0, 0.0, 0.0);

        // At origin, ψ should be maximum (1.0)
        assert!((fields.psi(&s) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_spiral_dynamics() {
        let dynamics = SpiralDynamics::new(0.01);
        let initial = State5D::new(1.0, 0.0, 0.0, 0.5, 0.5);

        let trajectory = dynamics.evolve(initial, 10);
        assert_eq!(trajectory.len(), 11);
    }

    #[test]
    fn test_resonance_invariant() {
        let fields = ResonanceFields::default();
        let s = State5D::new(0.5, 0.5, 0.5, 0.3, 0.3);

        let d = fields.resonance(&s);
        let d_check = fields.psi(&s) * fields.rho(&s) * fields.omega(&s);
        assert!((d - d_check).abs() < 1e-10, "D = ψ·ρ·ω must hold");
    }
}
