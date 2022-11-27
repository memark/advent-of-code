#![allow(dead_code, unused_imports)]

use criterion::{criterion_group, criterion_main, Criterion};
use day_05::{improve_polymer_and_react, react, react_2};
use std::{fs, time::Duration};

fn bench_react(c: &mut Criterion) {
    let file = fs::read_to_string("input").unwrap();
    let file = file.trim();

    c.bench_function("react", |b| {
        b.iter(|| react(file));
    });
}

fn bench_react_2(c: &mut Criterion) {
    let file = fs::read_to_string("input").unwrap();
    let file = file.trim();

    c.bench_function("react_2", |b| {
        b.iter(|| react_2(file));
    });
}

fn bench_improve_polymer_and_react(c: &mut Criterion) {
    let file = fs::read_to_string("input").unwrap();
    let file = file.trim();

    c.bench_function("improve_polymer_and_react", |b| {
        b.iter(|| improve_polymer_and_react(file));
    });
}

// criterion_group!(benches, bench_react, bench_improve_polymer_and_react);
criterion_group!(benches, bench_react, bench_react_2);
// criterion_group! {
//   name = benches;
//   config = Criterion::default()
//     // .sample_size(10)
//     // .measurement_time(Duration::from_secs(30))
//     ;
//   targets = bench_react//, bench_improve_polymer_and_react
// }
criterion_main!(benches);
