#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

/// A vector/queue of strings to represent Reverse Polish Notation
pub struct RPNQueue(pub Vec<String>);

lazy_static! {
    static ref PRECEDENCE: HashMap<String, u8> = {
        let mut result = HashMap::new();
        result.insert("+".to_string(), 1);
        result.insert("-".to_string(), 1);
        result.insert("*".to_string(), 2);
        result.insert("/".to_string(), 2);
        result
    };
}

impl RPNQueue {
    /// Constructs a new `Result<RPNQueue>` using a string with infix notation
    /// with [shunting-yard algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm).
    /// Supports decimals and round brackets only.
    ///
    /// # Examples
    /// ```
    /// use infix_calculator::RPNQueue;
    ///
    /// let queue = RPNQueue::from_infix_string(&"1.0 + 3 - (4 / 5)");
    /// ```
    pub fn from_infix_string(input: &str) -> Result<Self, String> {
        let mut output = RPNQueue(Vec::new());
        let mut stack = Vec::new();
        let mut buffer = String::new();
        for token in input.chars() {
            match token {
                white_space if white_space.is_whitespace() => {
                    if !buffer.is_empty() {
                        output.0.push(buffer);
                        buffer = String::new();
                    }
                }
                '+' | '-' | '*' | '/' => {
                    if !buffer.is_empty() {
                        output.0.push(buffer);
                        buffer = String::new();
                    }
                    while !stack.is_empty() && PRECEDENCE.get(stack.last().unwrap()).unwrap_or(&0) >
                        PRECEDENCE.get(&token.to_string()).unwrap() {
                        let op = stack.pop().unwrap();
                        output.0.push(op);
                    }
                    stack.push(token.to_string());
                }
                '(' => stack.push(token.to_string()),
                ')' => {
                    if !buffer.is_empty() {
                        output.0.push(buffer);
                        buffer = String::new();
                    }
                    while !stack.is_empty() && stack.last().unwrap() != "(" {
                        let popped = stack.pop().unwrap();
                        output.0.push(popped)
                    }
                    stack.pop();
                }
                '.' | '0'...'9' => {
                    buffer.push(token);
                }
                invalid => {
                    return Err(format!("Invalid token: {}", invalid))
                }
            }
        }

        while !stack.is_empty() {
            output.0.push(stack.pop().unwrap());
        }
        Ok(output)
    }

    /// Calculate result for given RPNQueue.
    ///
    /// # Example
    /// ```
    /// use infix_calculator::RPNQueue;
    ///
    /// let mut queue = RPNQueue::from_infix_string(&"1.0 + 3 - (4 / 5)").unwrap();
    /// assert_eq!(queue.calculate().unwrap(), 3.2);
    /// ```
    pub fn calculate(&mut self) -> Result<f64, String> {
        let mut numbers = Vec::new();
        for x in self.0.iter() {
            match x.as_ref() {
                "+" | "-" | "*" | "/" => {
                    let second = numbers.pop().ok_or("not enough input".to_string())?;
                    let first = numbers.pop().ok_or("not enough input".to_string())?;

                    let result = compute_result(first, second, x)?;
                    numbers.push(result);
                }
                number => {
                    let number: f64 = number.parse::<f64>().or(Err(format!("Invalid token: {}", number)))?;
                    numbers.push(number);
                }
            }
        }

        let result = numbers.pop().ok_or("not enough input".to_string())?;
        Ok(result)
    }
}

fn compute_result(first: f64, second: f64, op: &str) -> Result<f64, String> {
    match op {
        "+" => Ok(first + second),
        "-" => Ok(first - second),
        "*" => Ok(first * second),
        "/" => Ok(first / second),
        _ => Err(format!("invalid operator: {}", op))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_result_works() {
        assert_eq!(compute_result(5.0, 5.0, "+").unwrap(), 10.0);
        assert_eq!(compute_result(5.0, 5.0, "-").unwrap(), 0.0);
        assert_eq!(compute_result(5.0, 5.0, "*").unwrap(), 25.0);
        assert_eq!(compute_result(5.0, 5.0, "/").unwrap(), 1.0);
        assert!(compute_result(5.0, 5.0, "o").is_err());
    }
}