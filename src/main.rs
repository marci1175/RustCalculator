use calculator_recode::Calculator;

fn main() {
    let mut calculator = Calculator::new();

    dbg!(calculator.calculate(String::from("(4554 + 4)(4554 + 4)(4554 + 4)")));
}
