use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion, Throughput};
use pprof::criterion::{Output, PProfProfiler};
use rscompress_coding::arithmetic::{FenwickStatistics, Statistics};

const MIN_DATA_SIZE: usize = 1_000;
const FACTORS: [usize; 5] = [1, 5, 10, 50, 100];

fn statistics(c: &mut Criterion) {
    let mut group = c.benchmark_group("statistics");
    let source = include_bytes!("../../testdata/enwik7.raw");
    for factor in FACTORS.iter() {
        let size = factor * MIN_DATA_SIZE;
        let data: Vec<u8> = source.iter().take(size).map(|x| *x).collect();
        group.throughput(Throughput::Bytes(size as u64));

        let mut f = FenwickStatistics::<u8>::new();
        group.bench_with_input(
            BenchmarkId::new("Fenwick-u8", size),
            data.as_slice(),
            |b, s| b.iter(|| f.feed(s)),
        );
    }
}

criterion_group! {
    name = stats;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = statistics
}
criterion_main!(stats);
