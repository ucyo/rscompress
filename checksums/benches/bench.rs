use criterion::{criterion_group, criterion_main};
use criterion::{Criterion, BenchmarkId};
use rscompress_checksums::{Adler32, Checksum};

fn do_adler32_checksum(data: &str) -> u32{
    let mut a = Adler32::new();
    a.update(data.as_bytes());
    return a.checksum().unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    let word = "Wikipedia";
    let id = BenchmarkId::new("word", word);
    c.bench_with_input(id, &word, |b, &s| {
        b.iter(|| do_adler32_checksum(s));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
