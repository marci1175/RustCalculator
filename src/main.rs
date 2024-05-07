/* !
 So instead of 
2, MUL, 3, MUL 9 
 I Should do 
(2,MUL,(3, MUL, 9))

*/

use std::io;
use calculator_recode::Calculator;
use calculator_recode::CalculatorError;
fn main() {
    let mut calculator = Calculator::new();
    
    let mut input_buffer: String = String::new();

    loop {
        match io::stdin().read_line(&mut input_buffer) {
            Ok(_) => {
                match calculator.calculate(&input_buffer) {
                    Ok(answ) => println!("Answer: {answ}"),
                    Err(err) => {
                        err.downcast::<CalculatorError>().unwrap().show_error();
                    },
                }
            },
            Err(err) => {
                println!("{err}")
            },
        };
        
        //Clear input buffer
        input_buffer.clear();
    }
}
