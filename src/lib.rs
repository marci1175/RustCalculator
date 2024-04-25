use std::fmt::Display;

use anyhow::{bail, Error, Result};
use thiserror::Error;

#[derive(Debug, Clone)]
enum Expression {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Power,

    ///This is only used for parsing the equation later
    LeftBracket,
    RightBracket,

    Brackets(Vec<Expression>),

    Number(f64),
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
    fn new(error_type: CalculatorErrorType, index: usize) -> Self {
        Self {
            err_type: error_type,
            index,
        }
    }

    fn show_error(&self, erroring_input: &str) {
        //Print out the user input equation
        println!("[Error occured]\nEquation: \n{erroring_input}");

        //Move cursor to the error
        for num in 0..self.index {
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

        let _ = Self::calculate_equation(formatted_calculation).inspect_err(|e| {
            e.downcast_ref::<CalculatorError>()
                .unwrap()
                .show_error(&input);
        });

        todo!()
    }

    fn calculate_equation(formatted_calculation: String) -> Result<f64> {
        //Tokenize
        let mut token_list = tokenize(formatted_calculation.clone())?;

        //Parse list, i.e introduce bracket items
        let parsed_list = parse(&mut token_list)?;

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
            final_list.push(match_foreign_char(char, index)?);
        }
    }

    //If num buffer is not empty we should push it back
    if !number_buffer.is_empty() {
        final_list.push(Expression::Number(number_buffer.parse::<f64>().unwrap()));
    }

    Ok(final_list)
}

/// Insert additional data for example () * <-- () and BracketItems
fn parse(input: &mut Vec<Expression>) -> Result<()> {
    
    //'Format' the input (We are just making out job easier down the road by inserting expressions)
    parse_expressions(input)?;

    //Parse brackets, insert BracketItems, delete all of the Lbrackets and Rbrackets
    parse_brackets(input)?;

    Ok(())
}

fn parse_expressions(input: &mut Vec<Expression>) -> Result<()> {
    Ok(())
}

fn parse_brackets(input: &mut Vec<Expression>) -> Result<()> {
    let mut current_bracket_level = 0;

    for (index, item) in input.iter_mut().enumerate() {
        //If LeftBracket is detected, increase bracket lvl by 1
        if matches!(item, Expression::LeftBracket) {
            current_bracket_level += 1;
        }
        //If LeftBracket is detected, decrease bracket lvl by 1
        else if matches!(item, Expression::RightBracket) {
            current_bracket_level -= 1;
        }

        //If current_bracket_level == 0 that means the initial bracket is closed
    }

    Ok(())
}

fn match_foreign_char(foreign_char: char, index: usize) -> Result<Expression> {
    //We dont wrap it into an Ok(_) because it could be an invalid cahr
    let expression = match foreign_char {
        '+' => Expression::Addition,
        '-' => Expression::Addition,
        '/' | '%' => Expression::Addition,
        '*' => Expression::Addition,
        '^' => Expression::Power,
        ')' => Expression::RightBracket,
        '(' => Expression::LeftBracket,
        _ => {
            bail!(CalculatorError::new(CalculatorErrorType::ParseError, index))
        }
    };

    Ok(expression)
}
