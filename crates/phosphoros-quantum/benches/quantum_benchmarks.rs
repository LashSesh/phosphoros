//! Quantum Operation Benchmarks
//!
//! Comprehensive benchmarks for quantum simulation operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use phosphoros_quantum::backend::simulator::LocalSimulator;
use phosphoros_quantum::backend::{QuantumBackend, QuantumCircuit};
use phosphoros_quantum::algorithms::grover::{GroverSearch, FunctionOracle};
use phosphoros_quantum::algorithms::qaoa::{QAOA, MaxCutProblem};

/// Benchmark single-qubit gate operations
fn bench_single_qubit_gates(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_qubit_gates");

    for n_qubits in [2, 4, 6, 8, 10] {
        group.bench_with_input(
            BenchmarkId::new("hadamard", n_qubits),
            &n_qubits,
            |b, &n| {
                let sim = LocalSimulator::new(n);
                let mut circuit = QuantumCircuit::new(n);
                for q in 0..n {
                    circuit.h(q);
                }
                circuit.measure_all();

                b.iter(|| {
                    sim.run_circuit(black_box(&circuit), 1).unwrap()
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("rx_rotation", n_qubits),
            &n_qubits,
            |b, &n| {
                let sim = LocalSimulator::new(n);
                let mut circuit = QuantumCircuit::new(n);
                for q in 0..n {
                    circuit.rx(q, std::f64::consts::PI / 4.0);
                }
                circuit.measure_all();

                b.iter(|| {
                    sim.run_circuit(black_box(&circuit), 1).unwrap()
                });
            },
        );
    }

    group.finish();
}

/// Benchmark two-qubit gate operations
fn bench_two_qubit_gates(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_qubit_gates");

    for n_qubits in [2, 4, 6, 8] {
        group.bench_with_input(
            BenchmarkId::new("cnot_chain", n_qubits),
            &n_qubits,
            |b, &n| {
                let sim = LocalSimulator::new(n);
                let mut circuit = QuantumCircuit::new(n);

                // Create entangled chain
                circuit.h(0);
                for q in 0..(n - 1) {
                    circuit.cnot(q, q + 1);
                }
                circuit.measure_all();

                b.iter(|| {
                    sim.run_circuit(black_box(&circuit), 1).unwrap()
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("cz_ladder", n_qubits),
            &n_qubits,
            |b, &n| {
                let sim = LocalSimulator::new(n);
                let mut circuit = QuantumCircuit::new(n);

                for q in 0..n {
                    circuit.h(q);
                }
                for q in 0..(n - 1) {
                    circuit.cz(q, q + 1);
                }
                circuit.measure_all();

                b.iter(|| {
                    sim.run_circuit(black_box(&circuit), 1).unwrap()
                });
            },
        );
    }

    group.finish();
}

/// Benchmark Grover's algorithm
fn bench_grover(c: &mut Criterion) {
    let mut group = c.benchmark_group("grover");

    for n_qubits in [3, 4, 5, 6] {
        group.bench_with_input(
            BenchmarkId::new("search_single_target", n_qubits),
            &n_qubits,
            |b, &n| {
                let grover = GroverSearch::default_local(n);
                let target = 1 << (n - 1); // Middle of search space
                let oracle = FunctionOracle::new(|s| s == target, n);

                b.iter(|| {
                    grover.search(black_box(&oracle), 100).unwrap()
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("search_multi_target", n_qubits),
            &n_qubits,
            |b, &n| {
                let grover = GroverSearch::default_local(n);
                // Mark 4 targets
                let oracle = FunctionOracle::new(|s| s % 4 == 0, n);

                b.iter(|| {
                    grover.search(black_box(&oracle), 100).unwrap()
                });
            },
        );
    }

    group.finish();
}

/// Benchmark QAOA MaxCut
fn bench_qaoa(c: &mut Criterion) {
    let mut group = c.benchmark_group("qaoa");
    group.sample_size(20); // Reduce samples due to longer runtime

    // Simple graph for MaxCut
    let mut problem = MaxCutProblem::new(4);
    problem.add_edge(0, 1, 1.0);
    problem.add_edge(1, 2, 1.0);
    problem.add_edge(2, 3, 1.0);
    problem.add_edge(3, 0, 1.0);

    for depth in [1, 2, 3] {
        let problem_clone = problem.clone();
        group.bench_with_input(
            BenchmarkId::new("maxcut_4node", depth),
            &depth,
            move |b, &p| {
                let backend = LocalSimulator::new(4);
                let qaoa = QAOA::new(backend, p);

                b.iter(|| {
                    qaoa.solve_maxcut(black_box(&problem_clone), 50).unwrap()
                });
            },
        );
    }

    group.finish();
}

/// Benchmark circuit construction
fn bench_circuit_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("circuit_construction");

    for n_gates in [10, 50, 100, 500] {
        group.bench_with_input(
            BenchmarkId::new("build_random_circuit", n_gates),
            &n_gates,
            |b, &n| {
                b.iter(|| {
                    let mut circuit = QuantumCircuit::new(8);
                    for i in 0..n {
                        match i % 4 {
                            0 => { circuit.h(i % 8); }
                            1 => { circuit.x(i % 8); }
                            2 => { circuit.rx(i % 8, 0.5); }
                            3 => { circuit.cnot(i % 8, (i + 1) % 8); }
                            _ => {}
                        }
                    }
                    black_box(circuit)
                });
            },
        );
    }

    group.finish();
}

/// Benchmark measurement sampling
fn bench_measurement(c: &mut Criterion) {
    let mut group = c.benchmark_group("measurement");

    let sim = LocalSimulator::new(8);
    let mut circuit = QuantumCircuit::new(8);
    for q in 0..8 {
        circuit.h(q);
    }
    circuit.measure_all();

    for shots in [100, 1000, 10000] {
        group.bench_with_input(
            BenchmarkId::new("sample_8qubit", shots),
            &shots,
            |b, &s| {
                b.iter(|| {
                    sim.run_circuit(black_box(&circuit), s).unwrap()
                });
            },
        );
    }

    group.finish();
}

/// Benchmark resonance triplet calculation
fn bench_resonance(c: &mut Criterion) {
    let mut group = c.benchmark_group("resonance");

    for n_qubits in [4, 8, 13] {
        group.bench_with_input(
            BenchmarkId::new("triplet_calculation", n_qubits),
            &n_qubits,
            |b, &n| {
                let sim = LocalSimulator::new(n);
                let mut circuit = QuantumCircuit::new(n);
                for q in 0..n {
                    circuit.h(q);
                }

                // Run once to set state
                let _ = sim.run_circuit(&circuit, 1);

                b.iter(|| {
                    black_box(sim.resonance_triplet())
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_single_qubit_gates,
    bench_two_qubit_gates,
    bench_grover,
    bench_qaoa,
    bench_circuit_construction,
    bench_measurement,
    bench_resonance,
);

criterion_main!(benches);
