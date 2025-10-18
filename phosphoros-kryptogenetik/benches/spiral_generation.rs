//! Benchmark for Triton Spiral generation

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use phosphoros_kryptogenetik::TritonSpiralGenerator;

fn spiral_generation_benchmark(c: &mut Criterion) {
    c.bench_function("spiral_generation_1000", |b| {
        b.iter(|| {
            let mut spiral = TritonSpiralGenerator::new(black_box(42));
            for _ in 0..1000 {
                black_box(spiral.generate_next());
            }
        });
    });
}

criterion_group!(benches, spiral_generation_benchmark);
criterion_main!(benches);
