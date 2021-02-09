use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion};
use rscompress_checksums::{Adler32, Checksum, CRC32};

fn do_checksum_checks(data: &[u8], model: &mut impl Checksum) -> u32 {
    model.update(data);
    return model.checksum().unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let words = ["Wikipedia", "Awesome-string-baby", "This is great"];
    let mut group = c.benchmark_group("words");
    for word in words.iter() {
        let id = BenchmarkId::new("Adler32", word);
        let mut model = Adler32::new();
        group.bench_with_input(id, &word.as_bytes(), |b, &s| {
            b.iter(|| do_checksum_checks(s, &mut model));
        });
        let id = BenchmarkId::new("CRC32", word);
        let mut model = CRC32::new();
        group.bench_with_input(id, &word.as_bytes(), |b, &s| {
            b.iter(|| do_checksum_checks(s, &mut model));
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
