use std::fmt::Display;

#[derive(Debug)]
pub enum Operators {
    Addition,
    Subtraction,
    Multiplication,
    Division,

    //Left bracket
    Lbracket,
    //Right bracket
    RBracket,

    Num(f64),
}

pub struct Calculator {
    input: Box<String>,

    memory: Option<Box<Vec<Operators>>>,
}

impl Calculator {
    ///Calculator new
    pub fn new(input: impl ToString) {
        //Make new calculator instance
        Calculator::init(input.to_string()).main();
    }

    fn init(input: String) -> Self {
        //Allocate memory for struct xD
        Calculator { input: Box::new(input), memory: None /*Init mem with none*/ }
    }

    fn main(&mut self) {
        //Tokenize
        self.tokenizer();

        println!("{self}");
    }

    fn tokenizer(&mut self) {
        //Set memory
        self.memory = Some(Box::new(Tokenizer::tokenize(*self.input.clone())));
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
            &format!("Input: {}\nMemory: {:?}", *self.input, self.memory)
        )
    }
}

trait Expr {
    fn add(rhs: f64, lhs: f64) -> f64;
    fn sub(rhs: f64, lhs: f64) -> f64;
    fn multip(rhs: f64, lhs: f64) -> f64;
    fn div(rhs: f64, lhs: f64) -> f64;
}

struct Tokenizer {}

impl Tokenizer {
    fn tokenize(string : String) -> Vec<Operators> {
        let mut final_list: Vec<Operators> = Vec::new();

        //It doesnt really matter if this is a string or not, i just want to push it back, plus we also check if its a number nefore pushing back
        let mut current_number: String = String::new();

        for char in string.chars() {
            if char.is_digit(10) {
                current_number.push(char);
            } else if !current_number.is_empty() {
                //Push to final number
                final_list.push(Operators::Num(current_number.parse().unwrap()));

                //clear temporary string
                current_number.clear();
                
                //Check for mathematcial expression, will crash if there was an invalid character
                final_list.push(Tokenizer::extract_tokens(char));
            }
            //Chat gpt implementation was flawed as hell so dont mind me adding this
            else if current_number.is_empty() {

                //This will check for token right after we just pushed back another token
                final_list.push(Tokenizer::extract_tokens(char));
            }

        }

        if !current_number.is_empty() {
            final_list.push(Operators::Num(current_number.parse().unwrap()));
        }

        final_list
    }

    fn extract_tokens(char: char) -> Operators {
        use Operators::{Addition, Division, Multiplication, Subtraction, Lbracket, RBracket};

        match char {
            '+' => Addition,
            '-' => Subtraction,
            '*' => Multiplication,
            '/' => Division,
            ')' => RBracket,
            '(' => Lbracket,
            _ => panic!("You fucked up lil bro, spread them cheeks! {}", char)
        }
    }
}
