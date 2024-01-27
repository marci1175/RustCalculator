use crate::Calculator;

#[test]
fn equation_input() {
    Calculator::init("(10-9)(1+21)");
}

#[test]
fn addition() {
    Calculator::init("234+324");
}

#[test]
fn multiplication() {
    Calculator::init("234*324/212");
}
