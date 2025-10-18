//! Benchmark for Metatron embedding operations

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use phosphoros_kryptogenetik::MetatronGeometry;

fn metatron_embedding_benchmark(c: &mut Criterion) {
    c.bench_function("metatron_embedding_10000", |b| {
        let metatron = MetatronGeometry::new();
        b.iter(|| {
            for i in 0..10000 {
                black_box(metatron.embed_object(black_box(i)));
            }
        });
    });
}

criterion_group!(benches, metatron_embedding_benchmark);
criterion_main!(benches);
