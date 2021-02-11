use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion, Throughput};
use rand::{rngs::OsRng, RngCore};
use rscompress_checksums::{Adler32, Checksum, CRC32};

const DATA_SIZE: usize = 10_000;
const FACTORS: [usize; 5] = [2, 4, 8, 16, 32];

fn do_checksum<M: Checksum + Default>(data: &[u8]) -> u32 {
    let mut model: M = Default::default();
    model.update(&data);
    model.checksum().unwrap()
}

fn generate_random_data(size: usize) -> Vec<u8> {
    let mut data = vec![0u8; size];
    OsRng.fill_bytes(&mut data);
    data
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("checksums");
    for factor in FACTORS.iter() {
        let size = factor * DATA_SIZE;
        let data = generate_random_data(size);
        group.throughput(Throughput::Bytes((factor * DATA_SIZE) as u64));
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
