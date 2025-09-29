mod validation;

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn sub(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    a / b
}

fn power(num: f64, power: f64) -> f64 {
    num.powf(power)
}

fn apply_op(a: f64, b: f64, ops: &str) -> f64 {
    match ops {
        "+" => add(a, b),
        "-" => sub(a, b),
        "*" => multiply(a, b),
        "/" => divide(a, b),
        "^" => power(a, b),
        &_ => todo!(),
    }
}

static OPS_PREC: Lazy<HashMap<&'static str, u8>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("+", 1);
    m.insert("-", 1);
    m.insert("*", 2);
    m.insert("/", 2);
    m.insert("^", 3);
    m.insert("(", 4);
    m
});

fn sanitize_input(input: &mut String) -> &str {
    input.trim() // to remove the white spaces
}

fn tokenize_input(input: &str) -> Vec<&str> {
    let re = Regex::new(r"-?\d+(?:\.\d+)?|[+\-*/%()^]|[^+\-*/%()^\s]+").unwrap();
    let tokenized: Vec<&str> = re
        .captures_iter(input)
        .map(|cap| cap.get(0).unwrap().as_str())
        .collect();
    tokenized
}

fn main() {
    println!("CLI CALCULATOR");

    'outer: loop {
        print!(">>> ");
        let mut ops_stack: Vec<String> = Vec::new();
        let mut num_stack: Vec<f64> = Vec::new();

        let _ = stdout().flush();

        let mut input: String = String::from("");
        stdin()
            .read_line(&mut input)
            .expect("Please enter a proper input");
        let input = sanitize_input(&mut input);
        let tokens = tokenize_input(input);

        println!("{:#?}", tokens);

        for token in tokens {
            if token == ")" {
                while let Some(op) = ops_stack.pop() {
                    if op == "(" {
                        break;
                    }

                    let rhs = num_stack.pop();
                    let lhs = num_stack.pop();

                    match (lhs, rhs) {
                        (Some(a), Some(b)) => {
                            let result = apply_op(a, b, &op);
                            num_stack.push(result);
                        },
                        _ => {
                            eprintln!("(ERROR):: Syntax Error");
                        }
                    }
                }
            } else if OPS_PREC.contains_key(token) {
                if ops_stack.len() == 0 || token == "(" {
                    ops_stack.push(token.to_string());
                } else {
                    let mut top_stack = &ops_stack[ops_stack.len() - 1];
                    let mut stack_prec = OPS_PREC.get(top_stack.as_str()).unwrap_or(&0);
                    let curr_prec = OPS_PREC.get(token).unwrap_or(&0);

                    if *stack_prec == 0 || *curr_prec == 0 {
                        eprintln!("(ERROR):: Invalid symbol");
                        continue 'outer;
                    }

                    while let Some(op) = ops_stack.last() {
                        stack_prec = OPS_PREC.get(op.as_str()).unwrap_or(&0);
                        
                        if stack_prec < curr_prec || op == "(" {
                            break;
                        }

                        let rhs = num_stack.pop();
                        let lhs = num_stack.pop();

                        match (lhs, rhs) {
                            (Some(a), Some(b)) => {
                                let operator = ops_stack.pop().unwrap();
                                let result = apply_op(a, b, &operator);
                                num_stack.push(result);
                            }
                            _ => {
                                eprintln!("(ERROR):: Syntax Error");
                                continue 'outer;
                            }
                        }
                    }

                    ops_stack.push(token.to_string());
                }
            } else if let Ok(num) = token.parse::<f64>() {
                num_stack.push(num);
            } else {
                eprintln!("(ERROR):: Syntax Error");
                continue 'outer;
            }
        }

        // do remaining ops
        while let Some(op) = ops_stack.pop() {
            let rhs = num_stack.pop();
            let lhs = num_stack.pop();

            match (lhs, rhs) {
                (Some(a), Some(b)) => {
                    let result = apply_op(a, b, &op);
                    num_stack.push(result);
                },

                _ => {
                    eprintln!("(ERROR):: Syntax Error");
                    continue 'outer;
                }
            }
        }
    
        println!("{:#?}", num_stack);
        if num_stack.len() == 1 {
            println!("{0}", num_stack[0]);
        } else {
            eprintln!("(ERROR):: Syntax Error)");
        }
    }
}
