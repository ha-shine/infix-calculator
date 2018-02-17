extern crate infix_calculator;

use std::io::{self, Write};
use infix_calculator::RPNQueue;

fn main() {
    let mut buffer = String::new();
    loop {
        buffer.clear();
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut buffer).unwrap();

        let rpn = RPNQueue::from_infix_string(&buffer);
        if rpn.is_err() {
            println!("{}", rpn.err().unwrap());
            continue;
        }
        let mut rpn: RPNQueue = rpn.unwrap();
        println!("RPN Notation: {}", rpn.0.join(", "));

        let result = rpn.calculate();
        if result.is_err() {
            println!("Error: {}", result.err().unwrap());
            continue
        }
        println!("Result: {}", result.unwrap());
    }
}
