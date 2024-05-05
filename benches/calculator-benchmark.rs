use calculator_recode::Calculator;
use criterion::{criterion_group, criterion_main, Criterion};

fn parsing_test() {
    let mut calculator = Calculator::new().calculate("(((((((1))))) + (3434 * (((((1))))))))");
}

fn bench(c: &mut Criterion) {
    c.bench_function("Calculation time", |function| {
        function.iter(|| parsing_test())
    });
}

fn main() {
    criterion_group!(benches, bench);
    criterion_main!(benches);
}
