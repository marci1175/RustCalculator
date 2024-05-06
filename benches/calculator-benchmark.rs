use calculator_recode::Calculator;
use criterion::{criterion_group, criterion_main, Criterion};

fn parsing_test2() {
    let mut calculator = Calculator::new().calculate("(((42 * 3) - 18) / 2) + ((5 * 6) - (12 / 2)) + ((80 - (3 * 7)) + ((24 / 6) * 5)) - ((9 * 2) - 5) + (((96 / 4) + (7 * 3)) - (5^2)) + (18 - (4 * 2)) + ((120 / 5) + ((3 * 4) - (6^2))) - (15 + (6 / 3)) + (((48 / 3) + (5 * 9)) - (7^2)) + ((12 / 4) * 3) + (((105 - (8 * 6)) + (7^2)) / 3) - (18 + (5 * 2)) + ((144 / 6) + ((4 * 5) - (10^2))) - ((9 * 2) + 3) + ((200 - (9 * 7)) + (6^2)) - ((25 / 5) * 2) + (((81 / 9) + (4 * 8)) - (11^2)) + ((15 / 3) * 4) + ((64 / 4) + (9 * 7)) - ((14 + (6^2)) / 2) + ((160 - (5 * 12)) + (8^2)) - ((30 / 6) * 3) + ((216 / 6) + ((12 * 3) - (9^2))) - ((16 / 4) * 5) + (((150 / 5) + (11 * 4)) - (8^2)) + (36 - (6 * 4)) + (((45 * 2) - (14 / 7)) + (10^2)) / 3 + (((105 - (6 * 9)) + (5^2)) / 3) - (14 + (7 * 2)) + ((128 / 4) + ((6 * 6) - (13^2))) - ((20 + (8 / 2)) * 2) + (((75 / 5) + (8 * 5)) - (9^2)) + ((24 / 3) * 2) + ((180 - (7 * 10)) + (12^2)) - ((40 / 5) * 4) + (((100 / 2) + (13 * 3)) - (10^2)) + ((28 / 4) * 5) + ((256 / 4) + ((15 * 6) - (11^2))) - ((35 + (9 / 3)) * 2)");
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