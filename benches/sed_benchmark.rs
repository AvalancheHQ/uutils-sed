use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::io::Cursor;

// Simple benchmarks that test core sed functionality
// These are placeholder benchmarks that demonstrate the structure

fn benchmark_string_operations(c: &mut Criterion) {
    let input = "hello world\n".repeat(1000);

    c.bench_function("string_replace", |b| {
        b.iter(|| {
            let result = black_box(&input).replace("hello", "goodbye");
            black_box(result);
        });
    });
}

fn benchmark_regex_operations(c: &mut Criterion) {
    use regex::Regex;
    let input = "test123 foo456 bar789\n".repeat(500);
    let re = Regex::new(r"[0-9]+").unwrap();

    c.bench_function("regex_find_all", |b| {
        b.iter(|| {
            let count = re.find_iter(black_box(&input)).count();
            black_box(count);
        });
    });
}

fn benchmark_line_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("line_processing");

    for size in [100, 1000, 10000].iter() {
        let input = format!("line {}\n", "x").repeat(*size);

        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| {
                let count = black_box(input).lines().count();
                black_box(count);
            });
        });
    }

    group.finish();
}

fn benchmark_pattern_matching(c: &mut Criterion) {
    let input = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n".repeat(1000);

    c.bench_function("contains_pattern", |b| {
        b.iter(|| {
            let result = black_box(&input).contains("Lorem");
            black_box(result);
        });
    });
}

criterion_group!(
    benches,
    benchmark_string_operations,
    benchmark_regex_operations,
    benchmark_line_processing,
    benchmark_pattern_matching
);
criterion_main!(benches);
