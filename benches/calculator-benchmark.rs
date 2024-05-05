use calculator_recode::Calculator;
use criterion::{criterion_group, criterion_main, Criterion};

fn parsing_test2() {
    let mut calculator = Calculator::new().calculate("(2 ^ (2 / 0.1) + 23 * 23 (2 / 7)) + 9(3 / (2 ^ 12 * (23 - 22.9))) (2 ^ (2 / 0.1) + 23 * 23 (2 / 7)) + 9(3 / (2 ^ 12 * (23 - 22.9)))(2 ^ (2 / 0.1) + 23 * 23 (2 / 7)) + 9(3 / (2 ^ 12 * (23 - 22.9)))");
}

fn calculation_test() {
    let mut calculator = Calculator::new().calculate("43 + 234");
}

fn bench(c: &mut Criterion) {
    c.bench_function("Parsing time 2", |function| {
        function.iter(|| parsing_test2())
    });

    c.bench_function("Parsing time", |function| {
        function.iter(|| calculation_test())
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);