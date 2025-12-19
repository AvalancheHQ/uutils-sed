use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use sed::sed::compiler::Compiler;
use sed::sed::processor::Processor;
use std::io::Cursor;

fn benchmark_simple_substitution(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_substitution");
    
    let input = "hello world\nhello world\nhello world\n".repeat(100);
    group.throughput(Throughput::Bytes(input.len() as u64));
    
    group.bench_function("s/hello/goodbye/g", |b| {
        b.iter(|| {
            let script = black_box("s/hello/goodbye/g");
            let mut compiler = Compiler::new();
            let compiled = compiler.compile_str(script).unwrap();
            
            let input_cursor = Cursor::new(input.as_bytes());
            let mut output = Vec::new();
            let mut processor = Processor::new(compiled, false, false);
            processor.process(vec![Box::new(input_cursor)], &mut output).ok();
        })
    });
    
    group.finish();
}

fn benchmark_regex_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("regex_patterns");
    
    let input = "test123\nfoo456bar\nabc789def\n".repeat(100);
    group.throughput(Throughput::Bytes(input.len() as u64));
    
    group.bench_function("s/[0-9]+/NUM/g", |b| {
        b.iter(|| {
            let script = black_box("s/[0-9]+/NUM/g");
            let mut compiler = Compiler::new();
            let compiled = compiler.compile_str(script).unwrap();
            
            let input_cursor = Cursor::new(input.as_bytes());
            let mut output = Vec::new();
            let mut processor = Processor::new(compiled, false, false);
            processor.process(vec![Box::new(input_cursor)], &mut output).ok();
        })
    });
    
    group.finish();
}

fn benchmark_address_ranges(c: &mut Criterion) {
    let mut group = c.benchmark_group("address_ranges");
    
    let input = (1..=100).map(|i| format!("line {}\n", i)).collect::<String>();
    group.throughput(Throughput::Bytes(input.len() as u64));
    
    group.bench_function("10,20d", |b| {
        b.iter(|| {
            let script = black_box("10,20d");
            let mut compiler = Compiler::new();
            let compiled = compiler.compile_str(script).unwrap();
            
            let input_cursor = Cursor::new(input.as_bytes());
            let mut output = Vec::new();
            let mut processor = Processor::new(compiled, false, false);
            processor.process(vec![Box::new(input_cursor)], &mut output).ok();
        })
    });
    
    group.finish();
}

fn benchmark_multiple_commands(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiple_commands");
    
    let input = "foo bar baz\ntest line here\nanother line\n".repeat(50);
    group.throughput(Throughput::Bytes(input.len() as u64));
    
    group.bench_function("multiple_substitutions", |b| {
        b.iter(|| {
            let script = black_box("s/foo/FOO/g; s/bar/BAR/g; s/baz/BAZ/g");
            let mut compiler = Compiler::new();
            let compiled = compiler.compile_str(script).unwrap();
            
            let input_cursor = Cursor::new(input.as_bytes());
            let mut output = Vec::new();
            let mut processor = Processor::new(compiled, false, false);
            processor.process(vec![Box::new(input_cursor)], &mut output).ok();
        })
    });
    
    group.finish();
}

fn benchmark_input_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("input_sizes");
    
    let script = "s/test/TEST/g";
    
    for size in [100, 1000, 10000].iter() {
        let input = "test line\n".repeat(*size);
        group.throughput(Throughput::Bytes(input.len() as u64));
        
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| {
                let mut compiler = Compiler::new();
                let compiled = compiler.compile_str(script).unwrap();
                
                let input_cursor = Cursor::new(input.as_bytes());
                let mut output = Vec::new();
                let mut processor = Processor::new(compiled, false, false);
                processor.process(vec![Box::new(input_cursor)], &mut output).ok();
            })
        });
    }
    
    group.finish();
}

fn benchmark_deletion(c: &mut Criterion) {
    let mut group = c.benchmark_group("deletion");
    
    let input = (1..=1000).map(|i| format!("line {}\n", i)).collect::<String>();
    group.throughput(Throughput::Bytes(input.len() as u64));
    
    group.bench_function("delete_every_other", |b| {
        b.iter(|| {
            let script = black_box("1~2d");
            let mut compiler = Compiler::new();
            let compiled = compiler.compile_str(script).unwrap();
            
            let input_cursor = Cursor::new(input.as_bytes());
            let mut output = Vec::new();
            let mut processor = Processor::new(compiled, false, false);
            processor.process(vec![Box::new(input_cursor)], &mut output).ok();
        })
    });
    
    group.finish();
}

fn benchmark_print_commands(c: &mut Criterion) {
    let mut group = c.benchmark_group("print_commands");
    
    let input = "line 1\nline 2\nline 3\n".repeat(100);
    group.throughput(Throughput::Bytes(input.len() as u64));
    
    group.bench_function("print_lines", |b| {
        b.iter(|| {
            let script = black_box("p");
            let mut compiler = Compiler::new();
            let compiled = compiler.compile_str(script).unwrap();
            
            let input_cursor = Cursor::new(input.as_bytes());
            let mut output = Vec::new();
            let mut processor = Processor::new(compiled, false, false);
            processor.process(vec![Box::new(input_cursor)], &mut output).ok();
        })
    });
    
    group.finish();
}

fn benchmark_transliteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("transliteration");
    
    let input = "abcdefghijklmnopqrstuvwxyz\n".repeat(100);
    group.throughput(Throughput::Bytes(input.len() as u64));
    
    group.bench_function("y/abc/ABC/", |b| {
        b.iter(|| {
            let script = black_box("y/abc/ABC/");
            let mut compiler = Compiler::new();
            let compiled = compiler.compile_str(script).unwrap();
            
            let input_cursor = Cursor::new(input.as_bytes());
            let mut output = Vec::new();
            let mut processor = Processor::new(compiled, false, false);
            processor.process(vec![Box::new(input_cursor)], &mut output).ok();
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_simple_substitution,
    benchmark_regex_patterns,
    benchmark_address_ranges,
    benchmark_multiple_commands,
    benchmark_input_sizes,
    benchmark_deletion,
    benchmark_print_commands,
    benchmark_transliteration
);
criterion_main!(benches);
