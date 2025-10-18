//! Optimization operators for search space reduction

pub mod dk_lock;
pub mod pi_canonical;
pub mod sw_threshold;
pub mod wormdorf_trichter;

pub use dk_lock::{DKConfig, DKLock};
pub use pi_canonical::{PICanonical, PIConfig};
pub use sw_threshold::{SWConfig, SWThreshold};
pub use wormdorf_trichter::{WTConfig, WormdorfTrichter};

/// Combined operator set
#[derive(Debug, Clone)]
pub struct OperatorSet {
    /// Wormdorf-Trichter
    pub wt: Option<WormdorfTrichter>,
    /// SW Threshold
    pub sw: Option<SWThreshold>,
    /// DK Lock
    pub dk: Option<DKLock>,
    /// PI Canonical
    pub pi: Option<PICanonical>,
}

impl OperatorSet {
    /// Create new empty operator set
    pub fn new() -> Self {
        Self {
            wt: None,
            sw: None,
            dk: None,
            pi: None,
        }
    }

    /// Enable all operators with default configs
    pub fn all_default() -> Self {
        Self {
            wt: Some(WormdorfTrichter::new(WTConfig::default())),
            sw: Some(SWThreshold::new(SWConfig::default())),
            dk: Some(DKLock::new(DKConfig::default())),
            pi: Some(PICanonical::new(PIConfig::default())),
        }
    }

    /// Apply all enabled operators in sequence
    pub fn apply(&self, mut v: [f64; 5], score: f64, gradient: [f64; 5]) -> Option<[f64; 5]> {
        // SW threshold first (may gate)
        if let Some(ref sw) = self.sw {
            v = sw.apply(v, score)?;
        }

        // WT contraction
        if let Some(ref wt) = self.wt {
            v = wt.apply(v, gradient, score);
        }

        // DK lock
        if let Some(ref dk) = self.dk {
            v = dk.apply(v);
        }

        // PI canonical form
        if let Some(ref pi) = self.pi {
            v = pi.apply(v);
        }

        Some(v)
    }
}

impl Default for OperatorSet {
    fn default() -> Self {
        Self::new()
    }
}
