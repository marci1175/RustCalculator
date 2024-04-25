use std::env;

use calculator_recode::Calculator;

fn main() {
    let mut calculator = Calculator::new();

    dbg!(calculator.calculate(String::from("(1) * (2)")));
}
