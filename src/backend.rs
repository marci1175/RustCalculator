use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
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

    ///Semantic item pointer
    BracketItem(BracketItem),
}

impl Operator {
    
}

pub struct Calculator {
    //User input, this should be static
    input: String,

    memory: Option<Vec<Operator>>,
}

impl Calculator {
    ///Calculator new
    pub fn init(input: impl ToString) {
        //Make new calculator instance, remove all the whitespace
        Calculator::new(input.to_string().trim().replace(" ", "")).main();
    }

    fn new(user_input: String) -> Self {
        //Allocate memory for struct xD
        Calculator {
            input: user_input,
            memory: None, /*Init mem with none*/
        }
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
        f.write_str(&format!("Input: {}\nMemory: {:?}", self.input, self.memory))
    }
}
trait Expr {
    fn add(rhs: f64, lhs: f64) -> f64;
    fn sub(rhs: f64, lhs: f64) -> f64;
    fn multip(rhs: f64, lhs: f64) -> f64;
    fn div(rhs: f64, lhs: f64) -> f64;
}

#[derive(Debug, Clone, PartialEq)]
pub struct LeftBracket {
    level: usize,
}

impl LeftBracket {
    fn new(level: usize) -> LeftBracket {
        LeftBracket { level }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RightBracket {
    level: usize,
}

impl RightBracket {
    fn new(level: usize) -> RightBracket {
        RightBracket { level }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BracketItem {
    level: usize,
    inner_equation: Vec<Operator>,
}

impl Iterator for BracketItem {
    type Item = Operator;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_equation.iter().next().cloned()
    }

}

impl BracketItem {
    fn new(equation: Vec<Operator>, equation_level: usize) -> Self {
        BracketItem { level: equation_level, inner_equation: equation }
    }
}

///Used for determining the edits we want to make to the orginal vector
// pub struct DesiredEdits {
//     range: std::ops::Range<usize>,
//     to_replace_with: BracketItem,
// }

// impl DesiredEdits {
//     fn new(range: std::ops::Range<usize>, to_replace_with: BracketItem) -> Self {
//         Self {
//             range,
//             to_replace_with,
//         }
//     }
// }


mod tokenizer {
    use crate::backend::{BracketItem, LeftBracket, RightBracket};

    use super::Operator;

    pub(crate) fn tokenize(string: String) -> Vec<Operator> {
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
                    if extracted_token == Operator::LBracket
                        && matches!(last_item_in_list, Operator::Num(_))
                    {
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
                    if extracted_token == Operator::LBracket
                        && last_item_in_list == &Operator::RBracket
                    {
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

    pub(crate) fn remove_parts<T>(bounds: std::ops::Range<usize>, vector: Vec<T>) -> Vec<T>
    where
        T: Clone,
    {
        let mut extracted_vector: Vec<T> = Vec::new();

        for index in bounds {
            extracted_vector.remove(index);
        }

        return extracted_vector;
    }

    pub(crate) fn sort(equation: Vec<Operator>) {
        //This checks if the hierarchy should be reseted, im doing this because i cant set the usize to -1
        let mut was_decreased: bool = false;

        let mut bracket_level_counter: i32 = 0;

        let mut right_bracket_counter = 0;

        let mut captured_brackets: Vec<BracketItem> = Vec::new();
        
        //123+(124142-123(123))
        
        //1 + (2) + (3)

        //bracket_level _ocunter is used for cehcking the brackets level (this is a note to self, this is obvious)
        
        // let mut 

        for (left_index, left_item) in equation.iter().enumerate() {
            if *left_item == Operator::LBracket {
                
                //Reset right bracket counter
                right_bracket_counter = 0;

                //Search for the Eq closeure => ")", we assume the input is correct
                for (right_index, right_item) in equation.iter().enumerate().rev() {
                    
                    //Found it! :)
                    if *right_item == Operator::RBracket {
                        
                        //Check for bracket level
                        if right_bracket_counter == bracket_level_counter {
                            //Leave out the ( and )s
                            captured_brackets.push(BracketItem::new(equation[left_index + 1..=right_index - 1].iter().cloned().collect::<Vec<_>>(), bracket_level_counter as usize))
                        }

                        //Increase bracket level counter
                        right_bracket_counter += 1;
                    }

                }
                
                bracket_level_counter += 1;
            }
            if *left_item == Operator::RBracket {
                bracket_level_counter -= 1;
            }
        }

        dbg!(captured_brackets);

    }

    fn extract_tokens(char: char) -> Operator {
        use Operator::{Addition, Division, LBracket, Multiplication, RBracket, Subtraction};

        match char {
            '+' => Addition,
            '-' => Subtraction,
            '*' => Multiplication,
            '/' => Division,
            ')' => RBracket,
            '(' => LBracket,
            _ => panic!("You fucked up lil bro, spread them cheeks! {}", char),
        }
    }
}
