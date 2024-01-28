//I WILL NAME MY BINARY HOW I WANT IT
#![allow(non_snake_case)]

use backend::Calculator;

#[cfg(test)]
mod tests;

mod backend;

fn main() {
    #[cfg(not(debug_assertions))]
    {
        loop {
            let mut input: String = String::new();
    
            std::io::stdin().read_line(&mut input).expect("Failed to get input");
        
            Calculator::init(input);
        }
    }
    
    Calculator::init("123+(124142-123(123))+(43)");
}
