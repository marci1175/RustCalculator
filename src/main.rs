//I WILL NAME MY BINARY HOW I WANT IT
#![allow(non_snake_case)]

use backend::Calculator;

#[cfg(test)]
mod tests;

mod backend;

fn main() {
    Calculator::init("(123+124142)");
}

