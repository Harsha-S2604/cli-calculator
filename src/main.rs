mod validation;

use std::io::{Write, stdin, stdout};

fn sanitize_input(input: &mut String) -> &str {
    input.trim() // to remove the white spaces

}

fn main() {
    println!("CLI CALCULATOR");

    loop {
        print!(">>> ");
        let _ = stdout().flush();
        
        let mut input: String = String::from("");
        stdin().read_line(&mut input).expect("Please enter a proper input");
        let input = sanitize_input(&mut input).to_string();

        let valid_pemdas_syntax = validation::validate_pemdas_syntax(&input);
        if !valid_pemdas_syntax {
            println!("(ERROR)::Invalid syntax");
        }
    }
}
