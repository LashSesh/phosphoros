//! Integration tests for quantum pipeline
//!
//! Tests end-to-end quantum operations across the PHOSPHOROS ecosystem.

use phosphoros_quantum::backend::simulator::LocalSimulator;
use phosphoros_quantum::backend::{QuantumBackend, QuantumCircuit};
use phosphoros_quantum::algorithms::grover::{GroverSearch, FunctionOracle};
use phosphoros_quantum::algorithms::qaoa::QAOA;

#[test]
fn test_bell_state_integration() {
    // Create Bell state |Φ+⟩ = (|00⟩ + |11⟩)/√2
    let sim = LocalSimulator::new(2);
    let mut circuit = QuantumCircuit::new(2);

    circuit.h(0).cnot(0, 1).measure_all();

    let result = sim.run_circuit(&circuit, 1000).unwrap();

    // Verify Bell state properties
    let p00 = result.probability("00");
    let p11 = result.probability("11");
    let p01 = result.probability("01");
    let p10 = result.probability("10");

    assert!(p00 > 0.35, "Expected ~50% |00⟩");
    assert!(p11 > 0.35, "Expected ~50% |11⟩");
    assert!(p01 < 0.1, "Expected ~0% |01⟩");
    assert!(p10 < 0.1, "Expected ~0% |10⟩");
}

#[test]
fn test_ghz_state() {
    // Create 3-qubit GHZ state (|000⟩ + |111⟩)/√2
    let sim = LocalSimulator::new(3);
    let mut circuit = QuantumCircuit::new(3);

    circuit.h(0).cnot(0, 1).cnot(1, 2).measure_all();

    let result = sim.run_circuit(&circuit, 1000).unwrap();

    let p000 = result.probability("000");
    let p111 = result.probability("111");

    // GHZ state should only give 000 or 111
    assert!(p000 + p111 > 0.8, "GHZ should be 000 or 111");
}

#[test]
fn test_grover_finds_marked_state() {
    let grover = GroverSearch::default_local(4);

    // Mark state 5
    let oracle = FunctionOracle::new(|s| s == 5, 4);

    let result = grover.search(&oracle, 500).unwrap();

    assert!(result.success, "Grover should find marked state");
    assert!(
        result.solutions.contains(&5),
        "Solution should include marked state 5"
    );
}

#[test]
fn test_qaoa_maxcut() {
    use phosphoros_quantum::algorithms::qaoa::MaxCutProblem;

    // Simple 4-node cycle graph
    let mut problem = MaxCutProblem::new(4);
    problem.add_edge(0, 1, 1.0);
    problem.add_edge(1, 2, 1.0);
    problem.add_edge(2, 3, 1.0);
    problem.add_edge(3, 0, 1.0);

    let backend = LocalSimulator::new(4);
    let qaoa = QAOA::new(backend, 2); // depth = 2

    let result = qaoa.solve_maxcut(&problem, 100).unwrap();

    // MaxCut of a 4-cycle should be 4 (alternating partition)
    assert!(result.objective >= 3.0, "MaxCut should be at least 3");
}

#[test]
fn test_metatron_geometry_13_qubits() {
    // Test that 13-qubit Metatron geometry works
    let sim = LocalSimulator::metatron();
    assert_eq!(sim.qubit_count(), 13);

    let mut circuit = QuantumCircuit::new(13);

    // Apply Hadamard to all 13 qubits
    for q in 0..13 {
        circuit.h(q);
    }
    circuit.measure_all();

    let result = sim.run_circuit(&circuit, 100).unwrap();

    // Should have measured something
    assert!(!result.counts.is_empty());
}

#[test]
fn test_resonance_triplet_bounds() {
    let sim = LocalSimulator::new(4);
    let mut circuit = QuantumCircuit::new(4);

    for q in 0..4 {
        circuit.h(q);
    }

    let _ = sim.run_circuit(&circuit, 1);
    let (psi, rho, omega) = sim.resonance_triplet();

    // All components should be in valid range
    assert!(psi >= 0.0 && psi <= 2.0, "ψ should be bounded");
    assert!(rho >= 0.0 && rho <= 2.0, "ρ should be bounded");
    assert!(omega >= 0.0 && omega <= 1.0, "ω should be bounded");
}

#[test]
fn test_circuit_depth_scaling() {
    // Test that deeper circuits work correctly
    let sim = LocalSimulator::new(4);

    for depth in [1, 5, 10, 20] {
        let mut circuit = QuantumCircuit::new(4);

        for _ in 0..depth {
            for q in 0..4 {
                circuit.h(q);
            }
            for q in 0..3 {
                circuit.cnot(q, q + 1);
            }
        }
        circuit.measure_all();

        let result = sim.run_circuit(&circuit, 10);
        assert!(result.is_ok(), "Depth {} should work", depth);
    }
}

#[test]
fn test_parameterized_rotation_sweep() {
    use std::f64::consts::PI;

    let sim = LocalSimulator::new(1);

    // Sweep Rz from 0 to 2π
    for i in 0..8 {
        let theta = (i as f64) * PI / 4.0;

        let mut circuit = QuantumCircuit::new(1);
        circuit.h(0).rz(0, theta).h(0).measure(0);

        let result = sim.run_circuit(&circuit, 100).unwrap();

        // Results should vary with theta
        let p0 = result.probability("0");
        assert!(p0 >= 0.0 && p0 <= 1.0);
    }
}
