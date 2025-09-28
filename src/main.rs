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

fn get_result(a: f64, b: f64, ops: &str) -> f64 {
    match ops {
        "+" => add(a, b),
        "-" => sub(a, b),
        "*" => multiply(a, b),
        "/" => divide(a, b),
        &_ => todo!(),
    }
}

static OPS_PREC: Lazy<HashMap<&'static str, u8>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("+", 0);
    m.insert("-", 0);
    m.insert("*", 1);
    m.insert("/", 1);
    m.insert("^", 2);
    m.insert("(", 3);
    m
});

fn sanitize_input(input: &mut String) -> &str {
    input.trim() // to remove the white spaces
}

fn tokenize_input(input: &str) -> Vec<&str> {
    let re = Regex::new(r"([+-]?\d+(\.\d+)?|[+\-*\/%()^])").unwrap();
    let tokenized: Vec<&str> = re
        .captures_iter(input)
        .map(|cap| cap.get(0).unwrap().as_str())
        .collect();
    tokenized
}

fn main() {
    println!("CLI CALCULATOR");

    loop {
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

        for token in tokens {
            if OPS_PREC.contains_key(token) {
                if ops_stack.len() == 0 {
                    ops_stack.push(token.to_string());
                } else {
                    let symbol_prec = OPS_PREC.get(token).unwrap();
                    let mut last_op = &ops_stack[ops_stack.len() - 1];
                    let mut stack_prec = OPS_PREC.get(last_op.as_str()).unwrap();

                    let mut idx = ops_stack.len() - 1;
                    while idx >= 0 && symbol_prec <= stack_prec {
                        let curr_op = ops_stack.pop().unwrap();
                        let first_num = num_stack.pop().unwrap();
                        let second_num = num_stack.pop().unwrap();
                        let result = get_result(second_num, first_num, &curr_op);
                        num_stack.push(result);

                        idx -= 1;
                        last_op = &ops_stack[idx];
                        stack_prec = OPS_PREC.get(last_op.as_str()).unwrap();
                    }

                    ops_stack.push(token.to_string());
                }
            } else if let Ok(num) = token.parse::<f64>() {
                num_stack.push(num);
            } else {
                println!("(ERROR):: Syntax error");
                continue;
            }
        }

        while ops_stack.len() != 0 {
            let op = ops_stack.pop().unwrap();
            let first_num = num_stack.pop().unwrap();
            let second_num = num_stack.pop().unwrap();

            let result = get_result(second_num, first_num, &op);
            num_stack.push(result);
        }
        
        if num_stack.len() == 1 {
            println!("{0}", num_stack[0]);
        }

    }
}
