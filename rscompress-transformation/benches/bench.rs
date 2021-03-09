use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion, Throughput};
use rscompress_transformation::{BurrowWheeler, MoveToFront, RunLength, Transform};

const MIN_DATA_SIZE: usize = 1_000;
const FACTORS: [usize; 5] = [1, 5, 10, 50, 100];

fn criterion_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("roundtrip");
    let source = include_bytes!("../../testdata/enwik7.raw");
    for factor in FACTORS.iter() {
        let size = factor * MIN_DATA_SIZE;
        let data: Vec<u8> = source.iter().take(size).map(|x| *x).collect();
        let mut tmp: Vec<u8> = Vec::with_capacity(size);
        group.throughput(Throughput::Bytes(size as u64));

        // Testing Run Length Transformation
        let mut model = RunLength::new();
        group.bench_with_input(
            BenchmarkId::new("Run-Length (T)", size),
            data.as_slice(),
            |b, s| {
                b.iter(|| tmp = model.transform(s).unwrap());
            },
        );
        group.bench_with_input(
            BenchmarkId::new("Run-Length (R)", size),
            tmp.as_slice(),
            |b, s| {
                b.iter(|| model.reverse(s).unwrap());
            },
        );

        // Testing Move To Front Transformation
        let mut model = MoveToFront::new();
        group.bench_with_input(
            BenchmarkId::new("Move-To-Front (T)", size),
            data.as_slice(),
            |b, s| {
                b.iter(|| tmp = model.transform(s).unwrap());
            },
        );
        group.bench_with_input(
            BenchmarkId::new("Move-To-Front (R)", size),
            tmp.as_slice(),
            |b, s| {
                b.iter(|| model.reverse(s).unwrap());
            },
        );

        // Testing Burrow Wheeler Transformation
        let mut model = BurrowWheeler::new();
        group.bench_with_input(
            BenchmarkId::new("Burrow-Wheeler (T)", size),
            data.as_slice(),
            |b, s| {
                b.iter(|| tmp = model.transform(s).unwrap());
            },
        );
        group.bench_with_input(
            BenchmarkId::new("Burrow-Wheeler (R)", size),
            tmp.as_slice(),
            |b, s| {
                b.iter(|| model.reverse(s).unwrap());
            },
        );
    }
    group.finish();
}

criterion_group!(transform, criterion_roundtrip);
criterion_main!(transform);
