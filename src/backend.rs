use core::panic;
use std::fmt::{write, Display};
//(34.0_f32).powf(2.0)
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

    ///^
    Power,

    ///Left bracket, (
    LBracket,
    ///Right bracket, )
    RBracket,

    ///Number
    Num(f64),

    ///Semantic item pointer
    BracketItem(BracketItem),
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Addition => write!(f, "Addition"),
            Operator::Subtraction => write!(f, "Subtraction"),
            Operator::Multiplication => write!(f, "Multiplication"),
            Operator::Division => write!(f, "Division"),
            Operator::LBracket => write!(f, "LBracket"),
            Operator::RBracket => write!(f, "RBracket"),
            Operator::Power => write!(f, "Power"),
            Operator::Num(inner_num) => write!(f, "Num({})", inner_num),
            Operator::BracketItem(inner_eq) => {
                write!(f, "BracketItem({:?})", inner_eq.inner_equation)
            }
        }
    }
}

pub struct Calculator {
    //User input, this should be static
    input: String,

    memory: Option<Vec<Operator>>,

    //answer
    answ: f64,
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
            answ: 0.0,
        }
    }

    fn main(&mut self) {
        //Tokenize
        self.tokenizer();

        //parse
        self.parse();

        //calculate
        self.calculate();

        println!("{self}");
    }

    fn tokenizer(&mut self) {
        //Set memory
        self.memory = Some(parser::tokenize(self.input.clone()));
    }

    fn parse(&mut self) {
        self.memory = Some(parser::parse(&mut self.memory.clone().unwrap()).clone());
    }

    fn calculate(&mut self) {
        self.answ = calculator_engine::calculate(&mut self.memory.clone().unwrap());
    }
}

impl Expr for Operator {
    fn add(rhs: f64, lhs: f64) -> f64 {
        rhs + lhs
    }
    fn sub(rhs: f64, lhs: f64) -> f64 {
        rhs - lhs
    }
    fn multip(rhs: f64, lhs: f64) -> f64 {
        rhs * lhs
    }
    fn div(rhs: f64, lhs: f64) -> f64 {
        rhs / lhs
    }
    fn pow(rhs: f64, lhs: f64) -> f64 {
        rhs.powf(lhs)
    }
}

impl Display for Calculator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Input: {}\nMemory: {:?}\nResult: {}",
            self.input, self.memory, self.answ
        ))
    }
}

trait Expr {
    fn add(rhs: f64, lhs: f64) -> f64;
    fn sub(rhs: f64, lhs: f64) -> f64;
    fn multip(rhs: f64, lhs: f64) -> f64;
    fn div(rhs: f64, lhs: f64) -> f64;
    fn pow(rhs: f64, lhs: f64) -> f64;
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BracketItem {
    inner_equation: Vec<Operator>,
}

impl BracketItem {
    fn new(equation: Vec<Operator>) -> Self {
        BracketItem {
            inner_equation: equation,
        }
    }
}

impl From<&BracketItem> for Vec<Operator> {
    fn from(value: &BracketItem) -> Self {
        let mut inner_equation = value.inner_equation.clone();
        inner_equation.push(Operator::RBracket);
        inner_equation.insert(0, Operator::LBracket);

        inner_equation
    }
}

mod parser {
    use {super::Operator, crate::backend::BracketItem};

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

        parse(&mut final_list.clone());

        final_list
    }

    //Blanket function used to Parse all incoming tokenized data, return &mut so we can start calculating, and there is no need to clone
    pub(crate) fn parse(equation: &mut Vec<Operator>) -> &mut Vec<Operator> {
        modify_equation(filter_brackets(equation), equation)
    }

    fn filter_brackets(equation: &Vec<Operator>) -> Vec<BracketItem> {
        //This checks if the hierarchy should be reseted, im doing this because i cant set the usize to -1
        let mut bracket_level_counter: i32 = 0;

        let mut captured_brackets: Vec<BracketItem> = Vec::new();

        //bracket_level _ocunter is used for cehcking the brackets level (this is a note to self, this is obvious)
        for (left_index, left_item) in equation.iter().enumerate() {
            if *left_item == Operator::LBracket {
                bracket_level_counter = 0;

                let mut temp_vec: Vec<Operator> = Vec::new();

                for item in equation[left_index + 1..equation.len()].iter() {
                    if bracket_level_counter == 0 && *item == Operator::RBracket {
                        captured_brackets.push(BracketItem::new(temp_vec));
                        break;
                    }

                    if *item == Operator::LBracket {
                        bracket_level_counter += 1;
                    }

                    if *item == Operator::RBracket {
                        bracket_level_counter -= 1;
                    }

                    temp_vec.push(item.clone());
                }
            }
        }
        captured_brackets
    }

    // Iterate over captures
    //captured_brackets[0] is the main one containing all in unedited form
    fn modify_equation(
        captured_brackets: Vec<BracketItem>,
        equation: &mut Vec<Operator>,
    ) -> &mut Vec<Operator> {
        for bracket_item in captured_brackets.iter() {
            //Convert Bracketitem to Vec<Operator>
            let bracket_item_as_vec = Into::<Vec<Operator>>::into(bracket_item);

            //Get the position of the first occurence, which will be replaced
            //Search in the vector
            let occurence_pos = equation
                .windows(bracket_item_as_vec.len() /* Use the current bracket_item (from the extracted bracket items), and convert it to a Vec, so it can be searched with */)
                .position(|vector_window| vector_window == bracket_item_as_vec /* If the current BracketItem (as vec) can be found in the main equation reutrn Some(Index of occurence) */);

            //Occurence found
            if let Some(occurence_index) = occurence_pos {
                //Define range from: affected vector's starting point to its end point
                let range = occurence_index..occurence_index + bracket_item_as_vec.len();

                //drain the parts of the vetor, which will be replaced
                equation.drain(range);

                //Last bracket, this is what will get inserted to the current equation's brackets
                /* Because:
                    We have captured all brackets, so we know we wont make the wrong index
                */

                //Insert InnerEquation to the deleted one's place
                equation.insert(
                    occurence_index,
                    Operator::BracketItem(BracketItem::new(bracket_item.inner_equation.clone())),
                );
            }
            //Occurence not found
            else {
                //Search if there is A BracketItem we could iterate over, because it doesnt iter over BracketItem's by default, do iter_mut so we can grant mutability
                for equation_item in equation.iter_mut() {
                    //Bracket found
                    if let Operator::BracketItem(bracket_item_contains) = equation_item {
                        /*
                        Grant mutability and || DONT CLONE ||, so itll be able to modify the original equation
                        This recursion method will also always bring the equation into "scope", therefor this is a pretty good way
                        */
                        parse(&mut bracket_item_contains.inner_equation);
                    }
                }
            }
        }

        //Return modified equation
        equation
    }

    fn extract_tokens(char: char) -> Operator {
        use Operator::{Power, Addition, Division, LBracket, Multiplication, RBracket, Subtraction};

        match char {
            '+' => Addition,
            '-' => Subtraction,
            '*' => Multiplication,
            '/' => Division,
            ')' => RBracket,
            '(' => LBracket,
            '^' => Power,
            _ => panic!("You fucked up lil bro, spread them cheeks! {}", char),
        }
    }
}

mod calculator_engine {
    use super::{Expr, Operator};
    pub(crate) fn calculate(equation: &mut Vec<Operator>) -> f64 {
        let result = calculator(equation);

        #[cfg(not(debug_assertions))]
        {
            if result.len() > 1 {
                panic!("Invalid equation was entered, couldnt finish equation");
            }
        }

        if let Operator::Num(number) = result[0] {
            return number;
        } else {
            panic!("Invalid equation was entered, couldnt finish equation");
        }
    }

    fn calculator(equation: &mut Vec<Operator>) -> &mut Vec<Operator> {
        //Loop this way, so i can pass in &mut without any cloning
        let mut index = 0;
        // First check for * and /
        while index < equation.len() {
            //dbg
            #[cfg(debug_assertions)]
            {
                dbg!(equation.clone());
            }

            //Handle ^
            if equation[index] == Operator::Power {
                let left_operand = match &mut equation[index - 1] {
                    Operator::Num(inner_num) => *inner_num,
                    Operator::BracketItem(inner_bracket) => {
                        calculate(&mut inner_bracket.inner_equation)
                    }
                    _ => unimplemented!(),
                };

                let right_operand = match &mut equation[index + 1] {
                    Operator::Num(inner_num) => *inner_num,
                    Operator::BracketItem(inner_bracket) => {
                        calculate(&mut inner_bracket.inner_equation)
                    }
                    _ => unimplemented!(),
                };

                let result = Operator::pow(left_operand, right_operand);

                //drain vector on a specific place
                let range = index - 1..=index + 1;
                equation.drain(range);

                equation.insert(index - 1, Operator::Num(result));

                index = 0;
            }

            index += 1;
        }

        //Loop this way, so i can pass in &mut without any cloning
        let mut index = 0;
        // First check for * and /
        while index < equation.len() {
            //dbg
            #[cfg(debug_assertions)]
            {
                dbg!(equation.clone());
            }

            //Handle *
            if equation[index] == Operator::Multiplication {
                let left_operand = match &mut equation[index - 1] {
                    Operator::Num(inner_num) => *inner_num,
                    Operator::BracketItem(inner_bracket) => {
                        calculate(&mut inner_bracket.inner_equation)
                    }
                    _ => unimplemented!(),
                };

                let right_operand = match &mut equation[index + 1] {
                    Operator::Num(inner_num) => *inner_num,
                    Operator::BracketItem(inner_bracket) => {
                        calculate(&mut inner_bracket.inner_equation)
                    }
                    _ => unimplemented!(),
                };

                let result = Operator::multip(left_operand, right_operand);

                //drain vector on a specific place
                let range = index - 1..=index + 1;
                equation.drain(range);

                equation.insert(index - 1, Operator::Num(result));

                index = 0;
            }
            //Handle :
            else if equation[index] == Operator::Division {
                let left_operand = match &mut equation[index - 1] {
                    Operator::Num(inner_num) => *inner_num,
                    Operator::BracketItem(inner_bracket) => {
                        calculate(&mut inner_bracket.inner_equation)
                    }
                    _ => unimplemented!(),
                };

                let right_operand = match &mut equation[index + 1] {
                    Operator::Num(inner_num) => *inner_num,
                    Operator::BracketItem(inner_bracket) => {
                        calculate(&mut inner_bracket.inner_equation)
                    }
                    _ => unimplemented!(),
                };

                let result = Operator::div(left_operand, right_operand);

                //drain vector on a specific place
                let range = index - 1..=index + 1;
                equation.drain(range);

                equation.insert(index - 1, Operator::Num(result));

                index = 0;
            }

            index += 1;
        }

        //Loop this way, so i can pass in &mut without any cloning
        let mut index = 0;
        // First check for * and /
        while index < equation.len() {
            //dbg
            #[cfg(debug_assertions)]
            {
                dbg!(equation.clone());
            }

            //Handle +
            if equation[index] == Operator::Addition {
                let left_operand = match &mut equation[index - 1] {
                    Operator::Num(inner_num) => *inner_num,
                    Operator::BracketItem(inner_bracket) => {
                        calculate(&mut inner_bracket.inner_equation)
                    }
                    _ => unimplemented!(),
                };

                let right_operand = match &mut equation[index + 1] {
                    Operator::Num(inner_num) => *inner_num,
                    Operator::BracketItem(inner_bracket) => {
                        calculate(&mut inner_bracket.inner_equation)
                    }
                    _ => unimplemented!(),
                };

                let result = Operator::add(left_operand, right_operand);

                //drain vector on a specific place
                let range = index - 1..=index + 1;
                equation.drain(range);

                equation.insert(index - 1, Operator::Num(result));

                index = 0;
            }
            //Handle -
            else if equation[index] == Operator::Subtraction {
                let left_operand = match &mut equation[index - 1] {
                    Operator::Num(inner_num) => *inner_num,
                    Operator::BracketItem(inner_bracket) => {
                        calculate(&mut inner_bracket.inner_equation)
                    }
                    _ => unimplemented!(),
                };

                let right_operand = match &mut equation[index + 1] {
                    Operator::Num(inner_num) => *inner_num,
                    Operator::BracketItem(inner_bracket) => {
                        calculate(&mut inner_bracket.inner_equation)
                    }
                    _ => unimplemented!(),
                };

                let result = Operator::sub(left_operand, right_operand);

                //drain vector on a specific place
                let range = index - 1..=index + 1;
                equation.drain(range);

                equation.insert(index - 1, Operator::Num(result));

                index = 0;
            }

            index += 1;
        }

        equation
    }
}
