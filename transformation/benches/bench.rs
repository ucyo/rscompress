use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion};
use rscompress_transformation::{RunLength, Transform};

fn do_transformation(data: &[u8], model: &mut impl Transform) -> Vec<u8> {
    todo!()
}

fn criterion_benchmark(c: &mut Criterion) {
    todo!()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
