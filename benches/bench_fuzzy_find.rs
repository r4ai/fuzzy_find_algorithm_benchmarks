use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fuzzy_find_algorithm_benchmarks::{
    fuzzy_matcher_clangd, fuzzy_matcher_skimv2, get_data, jarowinkler, ngrammatic,
    rust_fuzzy_search, setup_corpus,
};

fn criterion_benchmark(c: &mut Criterion) {
    let data = get_data();
    let mut group = c.benchmark_group("fuzzy search algorithm benchmarks");

    group.bench_function("jarowinkler", |b| {
        b.iter(|| jarowinkler(black_box("VSCode"), black_box(&data), black_box(50)))
    });

    group.bench_function("fuzzy_matcher SkimV2", |b| {
        b.iter(|| fuzzy_matcher_skimv2(black_box("VSCode"), black_box(&data), black_box(0)))
    });

    group.bench_function("fuzzy_matcher clangd", |b| {
        b.iter(|| fuzzy_matcher_clangd(black_box("VSCode"), black_box(&data), black_box(0)))
    });

    let mut corups = setup_corpus(&data);
    group.bench_function("ngrammatic", |b| {
        b.iter(|| ngrammatic(black_box("VSCode"), black_box(0), black_box(&mut corups)))
    });

    group.bench_function("rust_fuzzy_saerch", |b| {
        b.iter(|| rust_fuzzy_search(black_box("VSCode"), black_box(&data), black_box(50)))
    });

    group.bench_function("sublime_fuzzy", |b| {
        b.iter(|| fuzzy_matcher_clangd(black_box("VSCode"), black_box(&data), black_box(50)))
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(500).warm_up_time(Duration::from_secs(5));
    targets = criterion_benchmark
);
criterion_main!(benches);
