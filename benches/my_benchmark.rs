#[macro_use]
extern crate criterion;

use brainfuck::*;
use criterion::Criterion;
use criterion::black_box;
use std::fs;

fn execute_file(filename: &str) {
    let mut output = Vec::new();
    let contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let commands = contents.parse().unwrap();
    let mut state = State::new();
    state.execute(&commands, &mut std::io::empty(), &mut output);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bottles", |b| b.iter(|| execute_file(black_box("examples/bottles.b"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
