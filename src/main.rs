use calculator_recode::Calculator;

fn main() {
    let mut calculator = Calculator::new();

    dbg!(calculator.calculate(String::from("(3 + (4 / 8)2) * 2 (34 / 2 (24 + 5))")));
}
