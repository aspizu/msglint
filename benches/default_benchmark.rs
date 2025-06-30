use std::hint::black_box;

use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};
use msglint::{
    messages::parse_message,
    problems::Problems,
};

pub fn default_benchmark(c: &mut Criterion) {
    c.bench_function("parse_message", |b| {
        b.iter(|| {
            let text = black_box("feet(scopeeeeeeeeeeeeee): hello worldeeeeeeeeeeeeeeeee\n\neeeeeeeeeeeeeeeeeeeee\neeeeeeeeee\neeeeeeee\neeeeeeeeee\neeeeeeeeeee\n\nBREAKING CHANGE: SHUT THE HELL UP\n");
            let mut problems = black_box(Problems::new());
            let _message = parse_message(text, &mut problems);
        })
    });
}

criterion_group!(benches, default_benchmark);
criterion_main!(benches);
