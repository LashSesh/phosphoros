//! Quantum Backend Abstraction Layer
//!
//! Provides a unified interface for different quantum computing backends,
//! allowing seamless switching between local simulation and cloud QPUs.

mod traits;
pub mod simulator;

pub use traits::*;

/// Supported quantum features for backend capability checking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuantumFeature {
    /// Basic single-qubit gates
    SingleQubitGates,
    /// Two-qubit entangling gates
    TwoQubitGates,
    /// Parameterized rotation gates
    ParameterizedGates,
    /// Mid-circuit measurements
    MidCircuitMeasurement,
    /// Error mitigation techniques
    ErrorMitigation,
    /// Native Metatron geometry support
    MetatronGeometry,
}

/// Backend registry for managing multiple quantum backends
pub struct BackendRegistry {
    backends: std::collections::HashMap<String, Box<dyn QuantumBackend>>,
    default_backend: String,
}

impl BackendRegistry {
    /// Create a new registry with the local simulator as default
    pub fn new() -> Self {
        let mut backends: std::collections::HashMap<String, Box<dyn QuantumBackend>> =
            std::collections::HashMap::new();

        backends.insert(
            "local".to_string(),
            Box::new(simulator::LocalSimulator::new(13)), // 13 qubits for Metatron
        );

        Self {
            backends,
            default_backend: "local".to_string(),
        }
    }

    /// Get a backend by name
    pub fn get(&self, name: &str) -> Option<&dyn QuantumBackend> {
        self.backends.get(name).map(|b| b.as_ref())
    }

    /// Get the default backend
    pub fn default(&self) -> &dyn QuantumBackend {
        self.backends.get(&self.default_backend).unwrap().as_ref()
    }

    /// Register a new backend
    pub fn register(&mut self, name: &str, backend: Box<dyn QuantumBackend>) {
        self.backends.insert(name.to_string(), backend);
    }

    /// Set the default backend
    pub fn set_default(&mut self, name: &str) -> Result<(), BackendError> {
        if self.backends.contains_key(name) {
            self.default_backend = name.to_string();
            Ok(())
        } else {
            Err(BackendError::NotFound(name.to_string()))
        }
    }
}

impl Default for BackendRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Backend-related errors
#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("Backend not found: {0}")]
    NotFound(String),

    #[error("Feature not supported: {0:?}")]
    FeatureNotSupported(QuantumFeature),

    #[error("Circuit execution failed: {0}")]
    ExecutionError(String),

    #[error("Invalid circuit: {0}")]
    InvalidCircuit(String),

    #[error("Measurement error: {0}")]
    MeasurementError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = BackendRegistry::new();
        assert!(registry.get("local").is_some());
    }

    #[test]
    fn test_default_backend() {
        let registry = BackendRegistry::new();
        let backend = registry.default();
        assert_eq!(backend.name(), "LocalSimulator");
    }
}
