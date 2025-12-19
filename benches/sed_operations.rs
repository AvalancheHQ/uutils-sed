use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sed::sed::compiler::compile;
use sed::sed::command::ProcessingContext;
use sed::sed::script_line_provider::ScriptValue;
use std::collections::HashMap;

fn create_context() -> ProcessingContext {
    ProcessingContext {
        all_output_files: false,
        debug: false,
        regex_extended: false,
        follow_symlinks: false,
        in_place: false,
        in_place_suffix: None,
        length: 70,
        quiet: true, // Suppress output in benchmarks
        posix: false,
        separate: false,
        sandbox: false,
        unbuffered: false,
        null_data: false,
        input_name: "<stdin>".to_string(),
        line_number: 0,
        last_address: false,
        last_line: false,
        last_file: false,
        stop_processing: false,
        saved_regex: None,
        input_action: None,
        hold: Default::default(),
        parsed_block_nesting: 0,
        label_to_command_map: HashMap::new(),
        substitution_made: false,
        append_elements: Vec::new(),
    }
}

fn bench_simple_substitution(c: &mut Criterion) {
    c.bench_function("compile simple substitution", |b| {
        b.iter(|| {
            let mut context = create_context();
            let scripts = vec![ScriptValue::StringVal(black_box("s/foo/bar/".to_string()))];
            let _ = compile(scripts, &mut context);
        })
    });
}

fn bench_complex_substitution(c: &mut Criterion) {
    c.bench_function("compile complex substitution with regex", |b| {
        b.iter(|| {
            let mut context = create_context();
            let scripts = vec![ScriptValue::StringVal(black_box(
                "s/[0-9]\\+/NUMBER/g".to_string(),
            ))];
            let _ = compile(scripts, &mut context);
        })
    });
}

fn bench_deletion(c: &mut Criterion) {
    c.bench_function("compile deletion command", |b| {
        b.iter(|| {
            let mut context = create_context();
            let scripts = vec![ScriptValue::StringVal(black_box("1,10d".to_string()))];
            let _ = compile(scripts, &mut context);
        })
    });
}

fn bench_multi_command(c: &mut Criterion) {
    c.bench_function("compile multiple commands", |b| {
        b.iter(|| {
            let mut context = create_context();
            let scripts = vec![
                ScriptValue::StringVal(black_box("s/foo/bar/".to_string())),
                ScriptValue::StringVal(black_box("s/baz/qux/g".to_string())),
                ScriptValue::StringVal(black_box("/pattern/d".to_string())),
            ];
            let _ = compile(scripts, &mut context);
        })
    });
}

fn bench_address_range(c: &mut Criterion) {
    c.bench_function("compile address range", |b| {
        b.iter(|| {
            let mut context = create_context();
            let scripts = vec![ScriptValue::StringVal(black_box(
                "/start/,/end/s/old/new/".to_string(),
            ))];
            let _ = compile(scripts, &mut context);
        })
    });
}

fn bench_transliteration(c: &mut Criterion) {
    c.bench_function("compile transliteration", |b| {
        b.iter(|| {
            let mut context = create_context();
            let scripts = vec![ScriptValue::StringVal(black_box(
                "y/abc/xyz/".to_string(),
            ))];
            let _ = compile(scripts, &mut context);
        })
    });
}

fn bench_extended_regex(c: &mut Criterion) {
    c.bench_function("compile with extended regex", |b| {
        b.iter(|| {
            let mut context = create_context();
            context.regex_extended = true;
            let scripts = vec![ScriptValue::StringVal(black_box(
                "s/[a-z]+@[a-z]+\\.[a-z]+/EMAIL/g".to_string(),
            ))];
            let _ = compile(scripts, &mut context);
        })
    });
}

criterion_group!(
    benches,
    bench_simple_substitution,
    bench_complex_substitution,
    bench_deletion,
    bench_multi_command,
    bench_address_range,
    bench_transliteration,
    bench_extended_regex
);
criterion_main!(benches);
