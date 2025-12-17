use codspeed_criterion_compat::{BenchmarkId, Criterion, criterion_group, criterion_main};
use sed::sed::command::ProcessingContext;
use sed::sed::compiler::compile;
use sed::sed::processor::process_all_files;
use sed::sed::script_line_provider::ScriptValue;
use std::io::Write;
use tempfile::NamedTempFile;

fn create_temp_file_with_content(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file.flush().unwrap();
    file
}

fn benchmark_simple_substitution(c: &mut Criterion) {
    let mut group = c.benchmark_group("substitution");

    // Test with different input sizes
    for size in [10, 100, 1000].iter() {
        let content = "hello world\n".repeat(*size);
        let file = create_temp_file_with_content(&content);
        let file_path = file.path().to_path_buf();

        group.bench_with_input(BenchmarkId::new("simple", size), size, |b, _| {
            b.iter(|| {
                let scripts = vec![ScriptValue::StringVal("s/hello/goodbye/g".to_string())];
                let mut context = ProcessingContext::default();
                context.quiet = true;
                let executable = compile(scripts, &mut context).unwrap();
                let _ = process_all_files(executable, vec![file_path.clone()], &mut context);
            });
        });
    }

    group.finish();
}

fn benchmark_address_matching(c: &mut Criterion) {
    let mut group = c.benchmark_group("address_matching");

    let content = (1..=1000)
        .map(|i| format!("line {}\n", i))
        .collect::<String>();
    let file = create_temp_file_with_content(&content);
    let file_path = file.path().to_path_buf();

    group.bench_function("line_range", |b| {
        b.iter(|| {
            let scripts = vec![ScriptValue::StringVal("10,100d".to_string())];
            let mut context = ProcessingContext::default();
            context.quiet = true;
            let executable = compile(scripts, &mut context).unwrap();
            let _ = process_all_files(executable, vec![file_path.clone()], &mut context);
        });
    });

    group.bench_function("regex_address", |b| {
        b.iter(|| {
            let scripts = vec![ScriptValue::StringVal("/line [0-9]+/d".to_string())];
            let mut context = ProcessingContext::default();
            context.quiet = true;
            let executable = compile(scripts, &mut context).unwrap();
            let _ = process_all_files(executable, vec![file_path.clone()], &mut context);
        });
    });

    group.finish();
}

fn benchmark_script_compilation(c: &mut Criterion) {
    let mut group = c.benchmark_group("compilation");

    group.bench_function("simple_script", |b| {
        b.iter(|| {
            let scripts = vec![ScriptValue::StringVal("s/hello/goodbye/g".to_string())];
            let mut context = ProcessingContext::default();
            let _ = compile(scripts, &mut context);
        });
    });

    group.bench_function("complex_script", |b| {
        b.iter(|| {
            let scripts = vec![ScriptValue::StringVal(
                "1,10s/foo/bar/g; /pattern/d; s/[0-9]+/NUM/g".to_string(),
            )];
            let mut context = ProcessingContext::default();
            let _ = compile(scripts, &mut context);
        });
    });

    group.bench_function("multiple_substitutions", |b| {
        b.iter(|| {
            let scripts = vec![ScriptValue::StringVal(
                "s/a/A/g; s/b/B/g; s/c/C/g; s/d/D/g; s/e/E/g".to_string(),
            )];
            let mut context = ProcessingContext::default();
            let _ = compile(scripts, &mut context);
        });
    });

    group.finish();
}

fn benchmark_regex_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("regex");

    let content = "The quick brown fox jumps over the lazy dog\n".repeat(100);
    let file = create_temp_file_with_content(&content);
    let file_path = file.path().to_path_buf();

    group.bench_function("simple_regex", |b| {
        b.iter(|| {
            let scripts = vec![ScriptValue::StringVal("s/fox/cat/g".to_string())];
            let mut context = ProcessingContext::default();
            context.quiet = true;
            let executable = compile(scripts, &mut context).unwrap();
            let _ = process_all_files(executable, vec![file_path.clone()], &mut context);
        });
    });

    group.bench_function("complex_regex", |b| {
        b.iter(|| {
            let scripts = vec![ScriptValue::StringVal("s/\\b[a-z]+\\b/WORD/g".to_string())];
            let mut context = ProcessingContext::default();
            context.quiet = true;
            let executable = compile(scripts, &mut context).unwrap();
            let _ = process_all_files(executable, vec![file_path.clone()], &mut context);
        });
    });

    group.bench_function("backreferences", |b| {
        b.iter(|| {
            let scripts = vec![ScriptValue::StringVal(
                "s/\\(\\w+\\) \\(\\w+\\)/\\2 \\1/g".to_string(),
            )];
            let mut context = ProcessingContext::default();
            context.quiet = true;
            let executable = compile(scripts, &mut context).unwrap();
            let _ = process_all_files(executable, vec![file_path.clone()], &mut context);
        });
    });

    group.finish();
}

fn benchmark_delete_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("delete");

    let content = (1..=1000)
        .map(|i| format!("line {}\n", i))
        .collect::<String>();
    let file = create_temp_file_with_content(&content);
    let file_path = file.path().to_path_buf();

    group.bench_function("delete_range", |b| {
        b.iter(|| {
            let scripts = vec![ScriptValue::StringVal("10,20d".to_string())];
            let mut context = ProcessingContext::default();
            context.quiet = true;
            let executable = compile(scripts, &mut context).unwrap();
            let _ = process_all_files(executable, vec![file_path.clone()], &mut context);
        });
    });

    group.bench_function("delete_pattern", |b| {
        b.iter(|| {
            let scripts = vec![ScriptValue::StringVal("/line [0-9]+/d".to_string())];
            let mut context = ProcessingContext::default();
            context.quiet = true;
            let executable = compile(scripts, &mut context).unwrap();
            let _ = process_all_files(executable, vec![file_path.clone()], &mut context);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_simple_substitution,
    benchmark_address_matching,
    benchmark_script_compilation,
    benchmark_regex_operations,
    benchmark_delete_operations
);
criterion_main!(benches);
