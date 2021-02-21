use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion, Throughput};
use rscompress_transformation::{MoveToFront, RunLength, Transform, BurrowWheeler};

const MIN_DATA_SIZE: usize = 1_000;
const FACTORS: [usize; 5] = [1, 10, 100, 1_000, 10_000];

fn do_transformation<M: Transform + Default>(data: &[u8]) -> Vec<u8> {
    let mut model: M = Default::default();
    let result = model.transform(data).unwrap();
    return result;
}

fn do_reverse<M: Transform + Default>(data: &[u8]) -> Vec<u8> {
    let mut model: M = Default::default();
    model.reverse(data).unwrap()
}

fn criterion_transform(c: &mut Criterion) {
    let mut group = c.benchmark_group("transform");
    let source = include_bytes!("../../testdata/enwik7.raw");
    for factor in FACTORS.iter() {
        let size = factor * MIN_DATA_SIZE;
        let data: Vec<u8> = source.iter().take(size).map(|x| *x).collect();
        group.throughput(Throughput::Bytes(size as u64));
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
        group.bench_with_input(
            BenchmarkId::new("Burrow-Wheeler", size),
            data.as_slice(),
            |b, s| {
                b.iter(|| do_transformation::<BurrowWheeler>(s));
            },
        );
    }
    group.finish();
}

fn criterion_reverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("reverse");
    let source = include_bytes!("../../testdata/enwik7.raw");
    for factor in FACTORS.iter() {
        let size = factor * MIN_DATA_SIZE;
        let data: Vec<u8> = source.iter().take(size).map(|x| *x).collect();
        group.throughput(Throughput::Bytes(size as u64));
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
        group.bench_with_input(
            BenchmarkId::new("Burrow-Wheeler", size),
            data.as_slice(),
            |b, s| {
                b.iter(|| do_reverse::<BurrowWheeler>(s));
            },
        );
    }
    group.finish();
}

criterion_group!(transform, criterion_transform, criterion_reverse);
criterion_main!(transform);
