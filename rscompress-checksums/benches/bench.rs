use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion, Throughput};
use rscompress_checksums::{Adler32, Checksum, CRC32};

const MIN_DATA_SIZE: usize = 1_000;
const FACTORS: [usize; 5] = [1, 10, 100, 1_000, 10_000];

fn do_checksum<M: Checksum + Default>(data: &[u8]) -> u32 {
    let mut model: M = Default::default();
    model.update(&data);
    model.checksum().unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("checksums");
    let source = include_bytes!("../../testdata/enwik7.raw");
    for factor in FACTORS.iter() {
        let size = factor * MIN_DATA_SIZE;
        let data: Vec<u8> = source.iter().take(size).map(|x| *x).collect();
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::new("Adler32", size),
            data.as_slice(),
            |b, s| {
                b.iter(|| do_checksum::<Adler32>(s));
            },
        );
        group.bench_with_input(BenchmarkId::new("CRC32", size), data.as_slice(), |b, s| {
            b.iter(|| do_checksum::<CRC32>(s));
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
