use codspeed_criterion_compat::{criterion_group, criterion_main, BenchmarkId, Criterion};
use sed::sed::compiler::compile;
use sed::sed::processor::process_all_files;
use sed::sed::command::ProcessingContext;
use sed::sed::script_line_provider::ScriptValue;
use std::collections::HashMap;
use std::io::Cursor;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use std::io::Write;

fn create_default_context() -> ProcessingContext {
    ProcessingContext {
        all_output_files: false,
        debug: false,
        regex_extended: false,
        follow_symlinks: false,
        in_place: false,
        in_place_suffix: None,
        length: 70,
        quiet: true, // Suppress output for benchmarking
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

fn create_temp_file_with_content(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file.flush().unwrap();
    file
}

fn bench_simple_substitution(c: &mut Criterion) {
    let mut group = c.benchmark_group("substitution");
    
    let test_cases = vec![
        ("small", "line1\nline2\nline3\nline4\nline5\n"),
        ("medium", &"test line\n".repeat(100)),
        ("large", &"test line with some content\n".repeat(1000)),
    ];

    for (size, input) in test_cases {
        group.bench_with_input(BenchmarkId::new("simple", size), &input, |b, &input| {
            b.iter(|| {
                let file = create_temp_file_with_content(input);
                let script = vec![ScriptValue::StringVal("s/test/example/g".to_string())];
                let mut context = create_default_context();
                let executable = compile(script, &mut context).unwrap();
                let files = vec![PathBuf::from(file.path())];
                process_all_files(executable, files, &mut context).unwrap();
            });
        });
    }

    group.finish();
}

fn bench_deletion(c: &mut Criterion) {
    let mut group = c.benchmark_group("deletion");
    
    let input = (1..=100).map(|i| format!("line {}\n", i)).collect::<String>();

    group.bench_function("delete_first_line", |b| {
        b.iter(|| {
            let file = create_temp_file_with_content(&input);
            let script = vec![ScriptValue::StringVal("1d".to_string())];
            let mut context = create_default_context();
            let executable = compile(script, &mut context).unwrap();
            let files = vec![PathBuf::from(file.path())];
            process_all_files(executable, files, &mut context).unwrap();
        });
    });

    group.bench_function("delete_pattern", |b| {
        b.iter(|| {
            let file = create_temp_file_with_content(&input);
            let script = vec![ScriptValue::StringVal("/line 5/d".to_string())];
            let mut context = create_default_context();
            let executable = compile(script, &mut context).unwrap();
            let files = vec![PathBuf::from(file.path())];
            process_all_files(executable, files, &mut context).unwrap();
        });
    });

    group.bench_function("delete_range", |b| {
        b.iter(|| {
            let file = create_temp_file_with_content(&input);
            let script = vec![ScriptValue::StringVal("10,20d".to_string())];
            let mut context = create_default_context();
            let executable = compile(script, &mut context).unwrap();
            let files = vec![PathBuf::from(file.path())];
            process_all_files(executable, files, &mut context).unwrap();
        });
    });

    group.finish();
}

fn bench_regex_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("regex");
    
    let input = (1..=100)
        .map(|i| format!("email{}@example.com\nuser{}\ndata{}\n", i, i, i))
        .collect::<String>();

    group.bench_function("simple_pattern", |b| {
        b.iter(|| {
            let file = create_temp_file_with_content(&input);
            let script = vec![ScriptValue::StringVal("/email/p".to_string())];
            let mut context = create_default_context();
            context.quiet = true;
            let executable = compile(script, &mut context).unwrap();
            let files = vec![PathBuf::from(file.path())];
            process_all_files(executable, files, &mut context).unwrap();
        });
    });

    group.bench_function("complex_pattern", |b| {
        b.iter(|| {
            let file = create_temp_file_with_content(&input);
            let script = vec![ScriptValue::StringVal("/[a-z]+[0-9]+@.*\\.com/p".to_string())];
            let mut context = create_default_context();
            context.quiet = true;
            let executable = compile(script, &mut context).unwrap();
            let files = vec![PathBuf::from(file.path())];
            process_all_files(executable, files, &mut context).unwrap();
        });
    });

    group.finish();
}

fn bench_compilation(c: &mut Criterion) {
    let mut group = c.benchmark_group("compilation");

    group.bench_function("simple_script", |b| {
        b.iter(|| {
            let script = vec![ScriptValue::StringVal("s/foo/bar/g".to_string())];
            let mut context = create_default_context();
            compile(script, &mut context).unwrap();
        });
    });

    group.bench_function("complex_script", |b| {
        b.iter(|| {
            let script = vec![ScriptValue::StringVal(
                "1d; 2,5s/test/example/g; /pattern/d; $a\\final line".to_string()
            )];
            let mut context = create_default_context();
            compile(script, &mut context).unwrap();
        });
    });

    group.bench_function("multiple_commands", |b| {
        b.iter(|| {
            let script = vec![
                ScriptValue::StringVal("s/foo/bar/".to_string()),
                ScriptValue::StringVal("s/test/example/".to_string()),
                ScriptValue::StringVal("/pattern/d".to_string()),
            ];
            let mut context = create_default_context();
            compile(script, &mut context).unwrap();
        });
    });

    group.finish();
}

fn bench_transformations(c: &mut Criterion) {
    let mut group = c.benchmark_group("transformations");
    
    let input = "abcdefghijklmnopqrstuvwxyz\n".repeat(50);

    group.bench_function("transliteration", |b| {
        b.iter(|| {
            let file = create_temp_file_with_content(&input);
            let script = vec![ScriptValue::StringVal("y/abcdefghijklmnopqrstuvwxyz/ABCDEFGHIJKLMNOPQRSTUVWXYZ/".to_string())];
            let mut context = create_default_context();
            let executable = compile(script, &mut context).unwrap();
            let files = vec![PathBuf::from(file.path())];
            process_all_files(executable, files, &mut context).unwrap();
        });
    });

    group.finish();
}

fn bench_complex_scripts(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_scripts");
    
    // Benchmark using the hanoi script input if available
    let hanoi_input = "3\n";
    
    group.bench_function("hanoi_towers", |b| {
        b.iter(|| {
            let file = create_temp_file_with_content(hanoi_input);
            let hanoi_script = r#"
# Tower of Hanoi in sed
:a
$!{
    N
    ba
}
s/\n/ /g
s/^/                                /
:b
s/\(.*\)\n\([0-9]*\)$/\1 \2/
tb
s/  *$//
s/\([0-9][0-9]*\) \([0-9][0-9]*\)/\1\n\2/
P
D
"#;
            let script = vec![ScriptValue::StringVal(hanoi_script.to_string())];
            let mut context = create_default_context();
            context.quiet = false;
            let executable = compile(script, &mut context).unwrap();
            let files = vec![PathBuf::from(file.path())];
            process_all_files(executable, files, &mut context).unwrap();
        });
    });

    group.finish();
}

fn bench_line_addressing(c: &mut Criterion) {
    let mut group = c.benchmark_group("addressing");
    
    let input = (1..=1000).map(|i| format!("line {}\n", i)).collect::<String>();

    group.bench_function("single_line", |b| {
        b.iter(|| {
            let file = create_temp_file_with_content(&input);
            let script = vec![ScriptValue::StringVal("500p".to_string())];
            let mut context = create_default_context();
            context.quiet = true;
            let executable = compile(script, &mut context).unwrap();
            let files = vec![PathBuf::from(file.path())];
            process_all_files(executable, files, &mut context).unwrap();
        });
    });

    group.bench_function("range", |b| {
        b.iter(|| {
            let file = create_temp_file_with_content(&input);
            let script = vec![ScriptValue::StringVal("100,200p".to_string())];
            let mut context = create_default_context();
            context.quiet = true;
            let executable = compile(script, &mut context).unwrap();
            let files = vec![PathBuf::from(file.path())];
            process_all_files(executable, files, &mut context).unwrap();
        });
    });

    group.bench_function("last_line", |b| {
        b.iter(|| {
            let file = create_temp_file_with_content(&input);
            let script = vec![ScriptValue::StringVal("$p".to_string())];
            let mut context = create_default_context();
            context.quiet = true;
            let executable = compile(script, &mut context).unwrap();
            let files = vec![PathBuf::from(file.path())];
            process_all_files(executable, files, &mut context).unwrap();
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_simple_substitution,
    bench_deletion,
    bench_regex_operations,
    bench_compilation,
    bench_transformations,
    bench_complex_scripts,
    bench_line_addressing
);
criterion_main!(benches);
