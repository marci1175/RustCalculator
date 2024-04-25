use calculator_recode::Calculator;

fn main() {
    let mut calculator = Calculator::new();
    dbg!(calculator.input(String::from("(1) * (2)")));
}
