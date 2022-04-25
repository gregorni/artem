use std::num::NonZeroU32;

use crate::benchmarks::util;
use criterion::{criterion_group, Criterion};

/// Benchmarks for outlined output.
///
/// Benchmarks the `outline` options.
fn outline_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("outline");

    //use lower sample size for faster benchmarking
    //it should still take long enough to see relevant changes in performance
    group.sample_size(10);

    let mut options = artem::options::OptionBuilder::new();
    //enable outline
    options.outline(true);

    //test on different resolutions

    group.bench_function("low resolution", |b| {
        b.iter_batched(
            || util::load_low_res_image(),
            |data| artem::convert(data, options.build()),
            criterion::BatchSize::LargeInput,
        );
    });

    group.bench_function("normal resolution", |b| {
        b.iter_batched(
            || util::load_normal_res_image(),
            |data| artem::convert(data, options.build()),
            criterion::BatchSize::LargeInput,
        );
    });

    group.bench_function("high resolution", |b| {
        b.iter_batched(
            || util::load_high_res_image(),
            |data| artem::convert(data, options.build()),
            criterion::BatchSize::LargeInput,
        );
    });

    group.finish();
}

/// Benchmarks for outlined output with 4 threads.
fn outline_threads_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("outline threads (4)");

    //use lower sample size for faster benchmarking
    //it should still take long enough to see relevant changes in performance
    group.sample_size(10);

    let mut options = artem::options::OptionBuilder::new();
    //enable outline
    options.outline(true);
    //set threads to 4 (the current default amount)
    options.threads(NonZeroU32::new(4).unwrap());

    //test on different resolutions

    group.bench_function("low resolution", |b| {
        b.iter_batched(
            || util::load_low_res_image(),
            |data| artem::convert(data, options.build()),
            criterion::BatchSize::LargeInput,
        );
    });

    group.bench_function("normal resolution", |b| {
        b.iter_batched(
            || util::load_normal_res_image(),
            |data| artem::convert(data, options.build()),
            criterion::BatchSize::LargeInput,
        );
    });

    group.bench_function("high resolution", |b| {
        b.iter_batched(
            || util::load_high_res_image(),
            |data| artem::convert(data, options.build()),
            criterion::BatchSize::LargeInput,
        );
    });

    group.finish();
}

criterion_group!(benches, outline_benchmark, outline_threads_benchmark);
