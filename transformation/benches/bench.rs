use criterion::{criterion_group, criterion_main};
use criterion::{BenchmarkId, Criterion, Throughput};
use rscompress_transformation::{RunLength, Transform};
use rand::{rngs::OsRng, RngCore};

const DATA_SIZE: usize = 10_000;

fn do_transformation<M: Transform+Default>(data: &[u8]) -> Vec<u8> {
    let mut model: M = Default::default();
    let result = model.transform(data).unwrap();
    return result
}

fn do_reverse<M: Transform+Default>(data: &[u8]) -> Vec<u8> {
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
    let factors: [usize; 5] = [2, 4, 8, 16, 24];
    for factor in factors.iter() {
        let size = factor * DATA_SIZE;
        let data = generate_random_data(size);
        group.throughput(Throughput::Bytes((factor * DATA_SIZE) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), data.as_slice(), |b, s| {
            b.iter(|| do_transformation::<RunLength>(s));
        });
    }
}

fn criterion_reverse(c: &mut Criterion) {
    let data = generate_random_data(DATA_SIZE);
    let id = BenchmarkId::new("reverse", data.len());
    c.bench_with_input(id, &data, |b, s| {
        b.iter(|| do_reverse::<RunLength>(s))
    });
}

criterion_group!(transform, criterion_transform);
criterion_group!(reverse, criterion_reverse);
criterion_main!(transform, reverse);
