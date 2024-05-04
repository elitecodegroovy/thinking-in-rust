use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashSet;

fn insert_benchmark(c: &mut Criterion) {
    c.bench_function("HashSet Insertion", |b| {
        let mut set = HashSet::new();
        b.iter(|| {
            set.insert(black_box(42));
        });
    });
}

criterion_group!(benches, insert_benchmark);
criterion_main!(benches);
// cargo bench
// output
//HashSet Insertion       time:   [7.6920 ns 7.7035 ns 7.7163 ns]
// Found 3 outliers among 100 measurements (3.00%)
//   3 (3.00%) high mild
