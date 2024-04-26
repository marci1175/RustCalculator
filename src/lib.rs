use std::fmt::Display;

use anyhow::{bail, Error, Result};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
enum Expression {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Power,

    ///This is only used for parsing the equation later
    /// (
    LeftBracket,
    /// )
    RightBracket,

    Brackets(Vec<Expression>),

    Number(f64),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Expression::Addition => "+".to_string(),
            Expression::Subtraction => "-".to_string(),
            Expression::Multiplication => "*".to_string(),
            Expression::Division => "/".to_string(),
            Expression::Power => "^".to_string(),
            Expression::LeftBracket => "(".to_string(),
            Expression::RightBracket => ")".to_string(),
            Expression::Brackets(inner_eq) => format!("({:?})", inner_eq),
            Expression::Number(inner_num) => format!("{}", inner_num),
        })
    }
}

pub struct Calculator {
    //This is the parsed, tokennized input
    calculation: Vec<Expression>,
}

#[derive(Debug, Clone)]
struct CalculatorError {
    /// Error type
    err_type: CalculatorErrorType,
    /// The index where the error occured
    /// This is used for displaying the error
    index: usize,
    /// The erroring input ```experimental```
    input: Vec<Expression>,
}

impl Display for CalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Error type: {}, Index: {}",
            self.err_type, self.index
        ))
    }
}

impl CalculatorError {
    fn new(error_type: CalculatorErrorType, index: usize, input: Vec<Expression>) -> Self {
        Self {
            err_type: error_type,
            index,
            input,
        }
    }

    fn show_error(&self) {
        //Convert self.input to String
        let erroring_input: String = self.input.iter().map(|item| item.to_string()).collect();

        //Print out the user input equation
        println!("[Error occured]\nEquation: \n{erroring_input}");

        // ! Fix this logic cuz this does not work
        let character_count = self.input.iter().take(self.index).map(|item| {
            match item {
                Expression::Number(inner) => {
                    inner.to_string().len()
                },
                _ => 1,
            }
        }).sum();

        //Move cursor to the error
        for _ in 0..character_count {
            print!(" ");
        }

        println!("^\nError: {}", self.err_type)
    }
}

#[derive(Error, Debug, Clone)]
enum CalculatorErrorType {
    #[error("Error while trying to tokenize the input")]
    ParseError,
    #[error("Error occured while calculating the equation")]
    CalculationError,
    #[error("The equation contains invalid formatting, for example brackets left open")]
    SyntaxError,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            calculation: Vec::new(),
        }
    }

    pub fn calculate(&mut self, input: String) -> Result<f64> {
        let formatted_calculation = input.trim().replace(" ", "");

        let _ = Self::calculate_equation(formatted_calculation.clone()).inspect_err(|e| {
            e.downcast_ref::<CalculatorError>().unwrap().show_error();
        });

        todo!()
    }

    fn calculate_equation(formatted_calculation: String) -> Result<f64> {
        //Tokenize
        let token_list = tokenize(formatted_calculation.clone())?;

        //Parse list, i.e introduce bracket items
        let parsed_list = parse(dbg!(token_list))?;

        dbg!(parsed_list);

        todo!()
    }
}

fn tokenize(input: String) -> Result<Vec<Expression>> {
    let mut final_list: Vec<Expression> = Vec::new();

    let mut number_buffer: String = String::new();

    for (index, char) in input.char_indices() {
        //. means we are defining a float, self explnatory
        if char.is_ascii_digit() || char == '.' {
            //Push back char to the buffer
            number_buffer.push(char);
        }
        //If its anything else then we need to push back the buffer, then we should clean it
        //Then we should recognize what type of char is this (I think it would be better if we didnt panic if there was an invalid char)
        //If number buffer is not empty (Contains a number, so it cant start with a non-nuumber character)
        else if !number_buffer.is_empty() {
            //Push back number to the final_list
            final_list.push(Expression::Number(number_buffer.parse::<f64>().unwrap()));

            //Clear buffer
            number_buffer.clear();
        }
        //If number_buffer is empty
        if number_buffer.is_empty() {
            //Recognize char if its an expression
            final_list.push({
                match char {
                    '+' => Expression::Addition,
                    '-' => Expression::Subtraction,
                    '/' | '%' => Expression::Division,
                    '*' => Expression::Multiplication,
                    '^' => Expression::Power,
                    ')' => Expression::RightBracket,
                    '(' => Expression::LeftBracket,
                    _ => {
                        bail!(CalculatorError::new(
                            CalculatorErrorType::ParseError,
                            index,
                            final_list /*This vector may be unfinished*/
                        ))
                    }
                }
            });
        }
    }

    //If num buffer is not empty we should push it back
    if !number_buffer.is_empty() {
        final_list.push(Expression::Number(number_buffer.parse::<f64>().unwrap()));
    }

    Ok(final_list)
}

/// Insert additional data for example () * <-- () and BracketItems
fn parse(input: Vec<Expression>) -> Result<Vec<Expression>> {
    //'Format' the input (We are just making out job easier down the road by inserting expressions)
    let mut parsed_expression = parse_expressions(input)?;

    //Parse brackets, insert BracketItems, delete all of the Lbrackets and Rbrackets
    //I want to borrow as mut so we can call the function from itself, and still modify it
    parse_brackets(&mut parsed_expression)?;

    Ok(parsed_expression)
}

fn parse_expressions(mut input: Vec<Expression>) -> Result<Vec<Expression>> {
    //Last Expression we have iter-ed on
    let mut last_expression: Option<Expression> = None;

    //We need to clone so we can borrow as mutable later, this doesnt really affect us since we dont modify anything after the for index (Cloning is fine because the infromation will not change in a way which will impact us)
    for (index, expression) in input.clone().iter().enumerate() {
        //This will only get ran when the index is 1 (Because we need to set the last expr. in the first iteration)
        //We should borrow as mutable so we can modify the variable without putting Some() everywhere
        if let Some(last_expression) = last_expression.as_mut() {
            match &expression {
                Expression::LeftBracket => {
                    //This means if )( is true then we will insert a * between the two
                    if *last_expression == Expression::RightBracket {
                        input.insert(index, Expression::Multiplication);
                    }

                    //If the last_expression was a number we need to add a * for the caluclator to multiply it later (and not crash)
                    if matches!(*last_expression, Expression::Number(_)) {
                        input.insert(index, Expression::Multiplication);
                    }
                }
                Expression::Number(_) => {
                    //If the last_expression was a number we need to add a * for the caluclator to multiply it later (and not crash)
                    if *last_expression == Expression::RightBracket {
                        input.insert(index, Expression::Multiplication);
                    }
                }

                _ => {}
            }

            //Save the last expression
            *last_expression = expression.clone();
        } else {
            //If there hasnt been any previous expressions then we can set the first one
            last_expression = Some(expression.clone());
        }
    }

    Ok(input)
}

///This function only moves the list of expressions contained by Lbracket and Rbracket into a ```Bracket(Vec<Expression>)```
fn parse_brackets(input: &mut Vec<Expression>) -> Result<()> {
    match (
        input.contains(&Expression::LeftBracket),
        input.contains(&Expression::RightBracket),
    ) {
        (true, true) => { /*Continue code excecution*/ }
        (false, false) => {
            return Ok(());
        }

        _ => {
            //A syntax error has happened, we need to search for the bracket (either Right or Left) which was presumably left in
            let bracket_pos = input
                .iter()
                .position(|item| {
                    *item == Expression::LeftBracket || *item == Expression::RightBracket
                })
                //We can safely unwrap here because the match statement requires a ```Some(_)``` case
                .unwrap();

            //Throw error
            bail!(CalculatorError::new(
                CalculatorErrorType::SyntaxError,
                bracket_pos,
                input.clone()
            ))
        }
    }

    let mut current_bracket_level = 0;

    //This is only used for error displaying more specifically for displaying syntax errors, this is ued to show the last opened lbracket which is assumed to be left open
    let mut last_lbracket_index = 0;

    //This is where the __first__ ( ```Lbracket``` occured, so this is where we should start moving the equation into the bracket enum
    let mut first_lbracket_occurence: Option<usize> = None;
    
    //We can clone here for the same reason as before
    for (index, item) in input.clone().iter().enumerate() {
        //If LeftBracket is detected, increase bracket lvl by 1
        if matches!(item, Expression::LeftBracket) {
            last_lbracket_index = index;
            
            first_lbracket_occurence = Some(index);

            current_bracket_level += 1;
        }
        //If LeftBracket is detected, decrease bracket lvl by 1
        else if matches!(item, Expression::RightBracket) {
            current_bracket_level -= 1;
        }

        //Check if a bracket is being captured
        if let Some(first_lbracket_occurence) = first_lbracket_occurence {
            //If current_bracket_level == 0 that means the initial bracket is closed
            //Move the inner equation to the Bracket(.  .  .)
            if current_bracket_level == 0 {
                //We use the ```first_lbracket_occurence``` and ```index``` to get the range we should be extracting
                let extracted_eq = input[first_lbracket_occurence + 1..index].to_vec();
                
                dbg!(&input);

                //Remove the values from the equation
                input.drain(first_lbracket_occurence..index + 1);

                //Insert new value
                input.insert(first_lbracket_occurence, Expression::Brackets(extracted_eq));
            }
        }
        
    }

    if current_bracket_level != 0 {
        //Implement getting valid index
        bail!(CalculatorError::new(
            CalculatorErrorType::SyntaxError,
            last_lbracket_index,
            input.clone()
        ));
    }

    Ok(())
}
