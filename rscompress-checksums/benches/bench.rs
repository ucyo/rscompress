use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion, Throughput};
use rscompress_checksums::{Adler32, Checksum, CRC32};

const MIN_DATA_SIZE: usize = 1_000;
const FACTORS: [usize; 5] = [1, 5, 10, 50, 100];

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("checksums");
    let source = include_bytes!("../../testdata/enwik7.raw");
    for factor in FACTORS.iter() {
        let size = factor * MIN_DATA_SIZE;
        let data: Vec<u8> = source.iter().take(size).map(|x| *x).collect();
        let mut tmp = 0u32;
        group.throughput(Throughput::Bytes(size as u64));

        // Testing Adler32 checksums
        let mut model = Adler32::new();
        group.bench_with_input(
            BenchmarkId::new("Adler32", size),
            data.as_slice(),
            |b, s| {
                b.iter(|| {
                    model.update(&s);
                    tmp = model.checksum().unwrap()
                });
            },
        );

        // Testing Adler32 checksums
        let mut model = CRC32::new();
        group.bench_with_input(BenchmarkId::new("CRC32", size), data.as_slice(), |b, s| {
            b.iter(|| {
                model.update(&s);
                tmp = model.checksum().unwrap()
            });
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
