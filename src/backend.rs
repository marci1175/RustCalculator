use std::fmt::Display;
use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    ///+
    Addition,
    
    ///-
    Subtraction,

    ///* 
    Multiplication,
    
    ///÷
    Division,

    ///Left bracket, (
    LBracket,
    ///Right bracket, )
    RBracket,

    ///Number
    Num(f64),
}

pub struct Calculator {
    //User input, this should be static
    input: String,

    memory: Option<Vec<Operator>>,
}

impl Calculator {
    ///Calculator new
    pub fn init(input: impl ToString) {
        //Make new calculator instance
        Calculator::new(input.to_string()).main();
    }

    fn new(user_input: String) -> Self {
        //Allocate memory for struct xD
        Calculator { input: user_input, memory: None /*Init mem with none*/ }
    }

    fn main(&mut self) {
        //Tokenize
        self.tokenizer();

        println!("{self}");
    }

    fn tokenizer(&mut self) {
        //Set memory
        self.memory = Some(tokenizer::tokenize(self.input.clone()));
    }
}

impl Expr for Calculator {
    fn add(rhs: f64, lhs: f64) -> f64 {
        rhs + lhs
    }
    fn div(rhs: f64, lhs: f64) -> f64 {
        rhs - lhs
    }
    fn multip(rhs: f64, lhs: f64) -> f64 {
        rhs * lhs
    }
    fn sub(rhs: f64, lhs: f64) -> f64 {
        rhs / lhs
    }
}
impl Display for Calculator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &format!("Input: {}\nMemory: {:?}", self.input, self.memory)
        )
    }
}
trait Expr {
    fn add(rhs: f64, lhs: f64) -> f64;
    fn sub(rhs: f64, lhs: f64) -> f64;
    fn multip(rhs: f64, lhs: f64) -> f64;
    fn div(rhs: f64, lhs: f64) -> f64;
}

#[derive(Debug)]
pub struct LeftBracket {
    level: usize,
}

impl LeftBracket {
    fn new(level : usize) -> LeftBracket {
        LeftBracket { level }
    }
}

#[derive(Debug)]
pub struct RightBracket {
    level: usize,
}

impl RightBracket {
    fn new(level : usize) -> RightBracket {
        RightBracket { level }
    }
}

#[derive(Debug)]
pub enum SemanticItem {
    Bracketed(LeftBracket, Box<crate::backend::SemanticItem>, RightBracket),
    Normal(Vec<Operator>, Option<Box<crate::backend::SemanticItem>>),
}

mod tokenizer {
    use crate::backend::{LeftBracket, RightBracket, SemanticItem};

    use super::Operator;

    pub(crate) fn tokenize(string : String) -> Vec<Operator> {
        let mut final_list: Vec<Operator> = Vec::new();

        //It doesnt really matter if this is a string or not, i just want to push it back, plus we also check if its a number nefore pushing back
        let mut current_number: String = String::new();

        for char in string.chars() {
            if char.is_ascii_digit() || char == '.' {
                current_number.push(char);
            } else if !current_number.is_empty() {
                //Push to final number
                final_list.push(Operator::Num(current_number.parse().unwrap()));

                //clear temporary string
                current_number.clear();
                
                //Check for mathematcial expression, will crash if there was an invalid character
                let extracted_token = extract_tokens(char);
                
                //Check for (123(123)), it will convert it to a (123*(123)) which will be simplified to (123*123) then to 123*123 = _Result_
                if let Some(last_item_in_list) = final_list.last() {

                    //Check if after number there is a (
                    if extracted_token == Operator::LBracket && matches!(last_item_in_list, Operator::Num(_))  {
                        //Insert the *
                        final_list.insert(final_list.len(), Operator::Multiplication);
                    
                    }

                }

                //Only push back expression after we checked for inserting the * because this way we can use just .last()
                final_list.push(extracted_token.clone());
                
            }
            //Chat gpt implementation was flawed as hell so dont mind me adding this
            else if current_number.is_empty() {
                //This will check for token right after we just pushed back another token

                let extracted_token = extract_tokens(char);
                
                //If (30)(30) is present insert a * for parsing more complex equations
                if let Some(last_item_in_list) = final_list.last() {
                    
                    //Check for extracted token and last_item in the list eg:       |
                    //                                                              V
                    //                                                         (123)*(123)
                    if extracted_token == Operator::LBracket && last_item_in_list == &Operator::RBracket {
                        //Insert the *
                        final_list.insert(final_list.len(), Operator::Multiplication);
                    }

                }
                

                final_list.push(extracted_token);
            }

        }

        if !current_number.is_empty() {
            final_list.push(Operator::Num(current_number.parse().unwrap()));
        }

        sort(final_list.clone());

        final_list
    }

    pub(crate) fn extract_equation<T>(bounds: std::ops::Range<usize>, vector: Vec<T>) -> Vec<T>
    where
        T: Copy,
      {
        let mut extracted_vector: Vec<T> = Vec::new();

        for index in bounds {
            extracted_vector.push(vector[index]);
        };

        return extracted_vector;
    }

    pub(crate) fn sort(equation: Vec<Operator>) {
        
        //This checks if the hierarchy should be reseted, im doing this because i cant set the usize to -1
        let mut was_decreased: bool = false;

        let mut lbracket_counter = 0;
        let mut rbracket_counter = 0;

        for (hierarchy_index, item) in equation.iter().enumerate() {
            if *item == Operator::LBracket {
                for (index, item) in equation.iter().rev().enumerate() {
                    if *item == Operator::RBracket {
                        dbg!(SemanticItem::Bracketed(LeftBracket::new(hierarchy_index), Box::new(SemanticItem::Normal(extract_equation(hierarchy_index..hierarchy_index, equation.clone()), None)), RightBracket::new(index)));
                    }
                }
            }
        }

        dbg!(lbracket_counter);
        dbg!(rbracket_counter);

    }

    fn extract_tokens(char: char) -> Operator {
        use Operator::{Addition, Division, Multiplication, Subtraction, LBracket, RBracket};

        match char {
            '+' => Addition,
            '-' => Subtraction,
            '*' => Multiplication,
            '/' => Division,
            ')' => RBracket,
            '(' => LBracket,
            _ => panic!("You fucked up lil bro, spread them cheeks! {}", char)
        }
    }

}