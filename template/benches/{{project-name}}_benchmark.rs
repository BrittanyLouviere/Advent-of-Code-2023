use {{project-name}}::{get_input, part_1, part_2};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = get_input();
    c.bench_function("part one", |b| b.iter(|| part_1::solve(black_box(&input))));
    c.bench_function("part two", |b| b.iter(|| part_2::solve(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
