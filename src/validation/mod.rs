use regex::Regex;
use std::collections::HashSet;

pub fn validate_pemdas_syntax(expression: &str) -> bool {
    let regex_pemdas = Regex::new(r"^-?\s*\d+(\s*[+\-*\/^]\s*-?\d+\s*)*$").unwrap();
    let valid_pemdas_syntax = regex_pemdas.is_match(expression);
    valid_pemdas_syntax
}

pub fn validate_syntax(expression: &str) -> bool {
    let re = Regex::new(r"^(\d+(\.\d+)?|[+\-*\/%()])+$").unwrap();
    let is_valid_syntax = re.is_match(expression);
    is_valid_syntax
}

pub fn validate_expression(exprs_tokens: &Vec<&str>) -> bool {
    let mut symbols = HashSet::new();
    symbols.insert("+");
    symbols.insert("-");
    symbols.insert("*");
    symbols.insert("/");
    symbols.insert("^");

    for (index, token) in exprs_tokens.iter().enumerate() {
        if symbols.contains(*token) && index + 1 < exprs_tokens.len() {
            if symbols.contains(exprs_tokens[index + 1]) {
                return false;
            }
        }
    }

    true
}
