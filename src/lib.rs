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

        // // ! Fix this logic cuz this does not work
        // let character_count = self.input.iter().take(self.index).map(|item| {
        //     match item {
        //         Expression::Number(inner) => {
        //             inner.to_string().len()
        //         },
        //         _ => 1,
        //     }
        // }).sum();
        // //Move cursor to the error
        // for _ in 0..character_count {
        //     print!(" ");
        // }

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
        let parsed_list = parse(token_list)?;

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
    let parsed_expression = parse_expressions(input)?;

    let parsed_brackets = parse_brackets(parsed_expression)?;

    Ok(parsed_brackets)
}

fn parse_brackets(mut input: Vec<Expression>) -> Result<Vec<Expression>> {
    let mut loop_index = 0;

    //This will count how deep are we in the brackets, which will become useful wehn we are dealing with nested brackets
    let mut bracket_nest_level = 0;

    let mut first_left_bracket_occurence = 0;

    //This buffer is used to capture the inner equation of a future bracket (its not yet inserted)
    let mut eq_buffer: Vec<Expression> = Vec::new();

    'mainloop: while input.len() > loop_index {
        let input_clone = input.clone();

        //reset every temp* variable
        eq_buffer.clear();

        'nestedloop: for (index, item) in input_clone.iter().enumerate() {
            // if *item == Expression::LeftBracket {
            //     //Increase bracket level
            //     bracket_nest_level += 1;
            //     let inner = input_clone[/* We add one to the index since that index is where the first ( appreared */ index + 1..input.len()].iter().take_while(|p| **p != Expression::RightBracket).cloned().collect::<Vec<_>>();
            //     dbg!(&input_clone);
            //     //Get which index the RightBracket appeared at; 
            //     // ! we sould add ```index``` whenever we are using it
            //     let rbracket_index = dbg!(input_clone[index..input.len()].iter().position(|p| *p == Expression::RightBracket));
            //     if let Some(rbracket_index) = rbracket_index {
            //         //We can decrease the value of this variable
            //         bracket_nest_level -= 1;
            //         //If this is true then it means that the nested brackets are contained in this bracket; however the brackets still contain an unparsed list 
            //         if bracket_nest_level == 0 {
            //             input.drain(index..=rbracket_index + index);
            //             //parse_brackets(inner)?
            //             input.insert(index, Expression::Brackets(inner));
            //             //Break out of the loop
            //             break;
            //         }
            //     }
            // else {
            //     //Rbracket was not closed
            //     bail!(CalculatorError::new(CalculatorErrorType::SyntaxError, index, input));
            // }

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
                },
                Expression::RightBracket => {
                    bracket_nest_level -= 1;
                },
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