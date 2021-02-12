use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion, Throughput};
use rand::{rngs::OsRng, RngCore};
use rscompress_transformation::{MoveToFront, RunLength, Transform};

const DATA_SIZE: usize = 10_000;
const FACTORS: [usize; 5] = [2, 4, 8, 16, 32];

fn do_transformation<M: Transform + Default>(data: &[u8]) -> Vec<u8> {
    let mut model: M = Default::default();
    let result = model.transform(data).unwrap();
    return result;
}

fn do_reverse<M: Transform + Default>(data: &[u8]) -> Vec<u8> {
    let mut model: M = Default::default();
    model.reverse(data).unwrap()
}

fn generate_random_data(size: usize) -> Vec<u8> {
    let mut data = vec![0u8; size];
    OsRng.fill_bytes(&mut data);
    data
}

fn criterion_transform(c: &mut Criterion) {
    let mut group = c.benchmark_group("transform");
    for factor in FACTORS.iter() {
        let size = factor * DATA_SIZE;
        let data = generate_random_data(size);
        group.throughput(Throughput::Bytes((factor * DATA_SIZE) as u64));
        group.bench_with_input(
            BenchmarkId::new("Run-Length", size),
            data.as_slice(),
            |b, s| {
                b.iter(|| do_transformation::<RunLength>(s));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("Move-to-Front", size),
            data.as_slice(),
            |b, s| {
                b.iter(|| do_transformation::<MoveToFront>(s));
            },
        );
    }
    group.finish();
}

fn criterion_reverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("reverse");
    for factor in FACTORS.iter() {
        let size = factor * DATA_SIZE;
        let data = generate_random_data(size);
        group.throughput(Throughput::Bytes((factor * DATA_SIZE) as u64));
        group.bench_with_input(
            BenchmarkId::new("Run-Length", size),
            data.as_slice(),
            |b, s| {
                b.iter(|| do_reverse::<RunLength>(s));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("Move-To-Front", size),
            data.as_slice(),
            |b, s| {
                b.iter(|| do_reverse::<MoveToFront>(s));
            },
        );
    }
    group.finish();
}

criterion_group!(transform, criterion_transform, criterion_reverse);
criterion_main!(transform);
