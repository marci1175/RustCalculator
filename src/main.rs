//I WILL NAME MY BINARY HOW I WANT IT
#![allow(non_snake_case)]
#![feature(array_windows)]
use backend::Calculator;

#[cfg(test)]
mod tests;

mod backend;

fn main() {
    #[cfg(not(debug_assertions))]
    {
        loop {
            let mut input: String = String::new();

            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to get input");

            Calculator::init(input);
        }
    }

    Calculator::init("9 * 2 / (3 / 3)");
}
