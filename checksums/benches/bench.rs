use criterion::{criterion_group, criterion_main};
use criterion::{Criterion, BenchmarkId};
use rscompress_checksums::{Adler32, Checksum};

fn do_adler32_checksum(data: &[u8]) -> u32{
    let mut a = Adler32::new();
    a.update(data);
    return a.checksum().unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    let words = [
        "Wikipedia", "Awesome-string-baby", "This is great"
    ];
    let mut group = c.benchmark_group("words");
    for word in words.iter() {
        let id = BenchmarkId::from_parameter(word);
        group.bench_with_input(id, &word.as_bytes(), |b, &s| {
            b.iter(|| do_adler32_checksum(s));
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
