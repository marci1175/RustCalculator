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
            Expression::Brackets(inner_eq) => {
                let inner_eq_str = inner_eq
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
                    .concat();

                format!("({inner_eq_str})")
            }
            Expression::Number(inner_num) => format!("{}", inner_num),
        })
    }
}

pub struct Calculator {}

#[derive(Debug, Clone, Error)]
pub struct CalculatorError {
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

    pub fn show_error(&self) {
        //Convert self.input to String
        let erroring_input: String = self.input.iter().map(|item| item.to_string()).collect();

        //Print out the user input equation
        println!("[Error occured]\nEquation: \n{erroring_input}");

        for _ in 0..self.index {
            print!(" ")
        }

        println!("^\nError: {}", self.err_type);
        println!("[Arrow may be at the wrong place]");
    }
}

#[derive(Error, Debug, Clone)]
enum CalculatorErrorType {
    #[error("Error while trying to tokenize the input")]
    ParseError,

    ///Specific error codes are wrapped in this enum
    #[error("This equation contains a conceptual error")]
    /*
        0: Tried to divide with 0
    */
    CalculationError(u8),

    #[error("The equation contains invalid formatting, for example brackets left open")]
    SyntaxError,
}

impl Calculator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate(&mut self, input: &str) -> Result<f64> {
        let formatted_calculation = input.trim().replace(" ", "");

        Self::parse_equation(formatted_calculation.clone())
    }

    fn parse_equation(formatted_calculation: String) -> Result<f64> {
        //Tokenize
        let token_list = tokenize(formatted_calculation.clone())?;

        //Parse list, i.e introduce bracket items
        let parsed_list = parse(token_list)?;

        Self::calculate_equation(parsed_list)
    }

    fn calculate_equation(parsed_list: Vec<Expression>) -> Result<f64> {
        let answ = calculate(parsed_list)?;

        if let Expression::Number(number) = answ[0] {
            return Ok(number);
        } else {
            bail!(CalculatorError::new(
                CalculatorErrorType::SyntaxError,
                0,
                answ
            ))
        }
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
            match number_buffer.parse::<f64>() {
                Ok(parsed_number) => {
                    final_list.push(Expression::Number(parsed_number));
                },
                Err(_) => {
                    bail!(CalculatorError::new(CalculatorErrorType::CalculationError(1), index, final_list));
                },
            }

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

    //If num buffer is not empty we should push it back, to save the last number
    if !number_buffer.is_empty() {
        match number_buffer.parse::<f64>() {
            Ok(parsed_number) => {
                final_list.push(Expression::Number(parsed_number));
            },
            Err(_) => {
                bail!(CalculatorError::new(CalculatorErrorType::CalculationError(1), final_list.len() + 1, final_list));
            },
        }
    }

    Ok(final_list)
}

/// Insert additional data for example () * <-- () and BracketItems
fn parse(input: Vec<Expression>) -> Result<Vec<Expression>> {
    //'Format' the input (We are just making out job easier down the road by inserting expressions)
    let parsed_expression = parse_expressions(input)?;

    let parsed_brackets = parse_brackets(parsed_expression)?;

    Ok(parsed_brackets)
}

fn parse_brackets(mut input: Vec<Expression>) -> Result<Vec<Expression>> {
    //Check if there are any brackets
    if !(input.contains(&Expression::LeftBracket) || input.contains(&Expression::RightBracket)) {
        return Ok(input);
    } 

    let mut loop_index = 0;

    //This will count how deep are we in the brackets, which will become useful wehn we are dealing with nested brackets
    let mut bracket_nest_level = 0;

    let mut first_left_bracket_occurence = 0;

    //This buffer is used to capture the inner equation of a future bracket (its not yet inserted)
    let mut eq_buffer: Vec<Expression> = Vec::new();

    while input.len() > loop_index {
        let input_clone = input.clone();

        //reset every temp* variable
        eq_buffer.clear();

        'nestedloop: for (index, item) in input_clone.iter().enumerate() {
            //We check bracket nestedness if its 0 we can modify the input replaing a certain range with a new Bracket item
            match item {
                Expression::LeftBracket => {
                    //if bracket nestedness is 0 that means that this left bracket is the beginning of a new inner equation
                    if bracket_nest_level == 0 {
                        //Set first* occurence
                        first_left_bracket_occurence = index;

                        //We can only increment this variable for obvious reasons
                        bracket_nest_level += 1;

                        //We should clear the buffer if we are starting to capture a bracket
                        eq_buffer.clear();

                        //continue the nestedloop
                        continue 'nestedloop;
                    }

                    bracket_nest_level += 1;
                }
                Expression::RightBracket => {
                    bracket_nest_level -= 1;
                }
                Expression::Brackets(_) => {
                    continue;
                }
                _ => {}
            }

            if *item == Expression::RightBracket {
                if bracket_nest_level == 0 {
                    let bracket_item = Expression::Brackets(parse_brackets(eq_buffer.clone())?);

                    input.drain(first_left_bracket_occurence..=index);

                    input.insert(first_left_bracket_occurence, bracket_item);

                    //Reset loop index to avoid skipping items
                    loop_index = 0;

                    //Break out of loop to work with the updated list
                    break 'nestedloop;
                }
            }

            //Push back item to buffer
            eq_buffer.push(item.clone());
        }

        //check for left open brackets
        if bracket_nest_level != 0 {
            bail!(CalculatorError::new(
                CalculatorErrorType::SyntaxError,
                dbg!(first_left_bracket_occurence),
                input
            ))
        }

        loop_index += 1;
    }

    Ok(input)
}

fn parse_expressions(mut input: Vec<Expression>) -> Result<Vec<Expression>> {
    //Last Expression we have iter-ed on
    let mut last_expression: Option<Expression> = None;

    let mut index = 0;

    //We need to use a while loop in order to use a vector which is always updated (we shouldnt clone is the point)
    while index < input.len() {
        //This will only get ran when the index is 1 (Because we need to set the last expr. in the first iteration)
        //We should borrow as mutable so we can modify the variable without putting Some() everywhere
        if let Some(last_expression) = last_expression.as_mut() {
            match &input[index] {
                Expression::LeftBracket => {
                    //This means if )( is true then we will insert a * between the two
                    if *last_expression == Expression::RightBracket {
                        input.insert(index, Expression::Multiplication);
                    }

                    //If last item was a number and this one is a ```(``` we insert a * on the same lgoic as in the Number match
                    if matches!(*last_expression, Expression::Number(_)) {
                        input.insert(index, Expression::Multiplication);
                    }
                }
                Expression::RightBracket => {
                    if *last_expression == Expression::LeftBracket {
                        //Insert one index infront of us -> `)` * `(`
                        input.insert(index + 1, Expression::Multiplication);
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
            *last_expression = input[index].clone();
        } else {
            //If there hasnt been any previous expressions then we can set the first one
            last_expression = Some(input[index].clone());
        }

        index += 1;
    }

    Ok(input)
}

fn calculate(mut input: Vec<Expression>) -> Result<Vec<Expression>> {
    //Calculate with the right order of mathematical calculations
    let mut loop_index;

    //First check for ^
    //Reset index
    loop_index = 1;

    while loop_index < input.len() {
        
        if input[loop_index] == Expression::Power {
            let lhs = match input.get(loop_index - 1).ok_or(CalculatorError::new(
                CalculatorErrorType::SyntaxError,
                loop_index,
                input.clone(),
            ))? {
                Expression::Brackets(inner_eq) => Calculator::calculate_equation(inner_eq.clone())?,
                Expression::Number(num) => *num,

                _ => bail!(CalculatorError::new(
                    CalculatorErrorType::SyntaxError,
                    loop_index,
                    input.clone()
                )),
            };

            let rhs = match input.get(loop_index + 1).ok_or(CalculatorError::new(
                CalculatorErrorType::SyntaxError,
                loop_index,
                input.clone(),
            ))? {
                Expression::Brackets(inner_eq) => Calculator::calculate_equation(inner_eq.clone())?,
                Expression::Number(num) => *num,

                _ => bail!(CalculatorError::new(
                    CalculatorErrorType::SyntaxError,
                    loop_index,
                    input.clone()
                )),
            };

            let calc_result = lhs.powf(rhs);

            //Drain calculated parts of the equation
            input.drain(loop_index - 1..=loop_index + 1);

            //Insert answ
            input.insert(loop_index - 1, Expression::Number(calc_result));
            
            //Reset loop index
            loop_index = 1;
            //Break exceuction of this cycle
            continue;
        }

        loop_index += 1;
    }

    //Check for * /
    //Reset index
    loop_index = 1;

    while loop_index < input.len() {
        if input[loop_index] == Expression::Multiplication {
            let lhs = match input.get(loop_index - 1).ok_or(CalculatorError::new(
                CalculatorErrorType::SyntaxError,
                loop_index,
                input.clone(),
            ))? {
                Expression::Brackets(inner_eq) => Calculator::calculate_equation(inner_eq.clone())?,
                Expression::Number(num) => *num,

                _ => bail!(CalculatorError::new(
                    CalculatorErrorType::SyntaxError,
                    loop_index,
                    input.clone()
                )),
            };

            let rhs = match input.get(loop_index + 1).ok_or(CalculatorError::new(
                CalculatorErrorType::SyntaxError,
                loop_index,
                input.clone(),
            ))? {
                Expression::Brackets(inner_eq) => Calculator::calculate_equation(inner_eq.clone())?,
                Expression::Number(num) => *num,

                _ => bail!(CalculatorError::new(
                    CalculatorErrorType::SyntaxError,
                    loop_index,
                    input.clone()
                )),
            };

            let calc_result = lhs * rhs;

            //Drain calculated parts of the equation
            input.drain(loop_index - 1..=loop_index + 1);

            //Insert answ
            input.insert(loop_index - 1, Expression::Number(calc_result));

            //Reset loop index
            loop_index = 1;
            //Break exceuction of this cycle
            continue;
        } else if input[loop_index] == Expression::Division {
            let lhs = match input.get(loop_index - 1).ok_or(CalculatorError::new(
                CalculatorErrorType::SyntaxError,
                loop_index,
                input.clone(),
            ))? {
                Expression::Brackets(inner_eq) => Calculator::calculate_equation(inner_eq.clone())?,
                Expression::Number(num) => *num,

                _ => bail!(CalculatorError::new(
                    CalculatorErrorType::SyntaxError,
                    loop_index,
                    input.clone()
                )),
            };

            let rhs = match input.get(loop_index + 1).ok_or(CalculatorError::new(
                CalculatorErrorType::SyntaxError,
                loop_index,
                input.clone(),
            ))? {
                Expression::Brackets(inner_eq) => Calculator::calculate_equation(inner_eq.clone())?,
                Expression::Number(num) => *num,

                _ => bail!(CalculatorError::new(
                    CalculatorErrorType::SyntaxError,
                    loop_index,
                    input.clone()
                )),
            };

            if rhs == 0. {
                bail!(CalculatorError::new(
                    CalculatorErrorType::CalculationError(0),
                    loop_index + 1,
                    input
                ))
            }

            let calc_result = lhs / rhs;

            //Drain calculated parts of the equation
            input.drain(loop_index - 1..=loop_index + 1);

            //Insert answ
            input.insert(loop_index - 1, Expression::Number(calc_result));

            //Reset loop index
            loop_index = 1;
            //Break exceuction of this cycle
            continue;
        }
        loop_index += 1;
    }

    //Check for + -
    //Reset index
    loop_index = 1;

    while loop_index < input.len() {
        if input[loop_index] == Expression::Addition {
            let lhs = match input.get(loop_index - 1).ok_or(CalculatorError::new(
                CalculatorErrorType::SyntaxError,
                loop_index,
                input.clone(),
            ))? {
                Expression::Brackets(inner_eq) => Calculator::calculate_equation(inner_eq.clone())?,
                Expression::Number(num) => *num,

                _ => bail!(CalculatorError::new(
                    CalculatorErrorType::SyntaxError,
                    loop_index,
                    input.clone()
                )),
            };

            let rhs = match input.get(loop_index + 1).ok_or(CalculatorError::new(
                CalculatorErrorType::SyntaxError,
                loop_index,
                input.clone(),
            ))? {
                Expression::Brackets(inner_eq) => Calculator::calculate_equation(inner_eq.clone())?,
                Expression::Number(num) => *num,

                _ => bail!(CalculatorError::new(
                    CalculatorErrorType::SyntaxError,
                    loop_index,
                    input.clone()
                )),
            };

            let calc_result = lhs + rhs;

            //Drain calculated parts of the equation
            input.drain(loop_index - 1..=loop_index + 1);

            //Insert answ
            input.insert(loop_index - 1, Expression::Number(calc_result));

            //Reset loop index
            loop_index = 1;
            //Break exceuction of this cycle
            continue;

        } else if input[loop_index] == Expression::Subtraction {
            let lhs = match input.get(loop_index - 1).ok_or(CalculatorError::new(
                CalculatorErrorType::SyntaxError,
                loop_index,
                input.clone(),
            ))? {
                Expression::Brackets(inner_eq) => Calculator::calculate_equation(inner_eq.clone())?,
                Expression::Number(num) => *num,

                _ => bail!(CalculatorError::new(
                    CalculatorErrorType::SyntaxError,
                    loop_index,
                    input.clone()
                )),
            };

            let rhs = match input.get(loop_index + 1).ok_or(CalculatorError::new(
                CalculatorErrorType::SyntaxError,
                loop_index,
                input.clone(),
            ))? {
                Expression::Brackets(inner_eq) => Calculator::calculate_equation(inner_eq.clone())?,
                Expression::Number(num) => *num,

                _ => bail!(CalculatorError::new(
                    CalculatorErrorType::SyntaxError,
                    loop_index,
                    input.clone()
                )),
            };

            let calc_result = lhs - rhs;

            //Drain calculated parts of the equation
            input.drain(loop_index - 1..=loop_index + 1);

            //Insert answ
            input.insert(loop_index - 1, Expression::Number(calc_result));

            //Reset loop index
            loop_index = 1;
            //Break exceuction of this cycle
            continue;
        }

        loop_index += 1;
    }

    Ok(input)
}
