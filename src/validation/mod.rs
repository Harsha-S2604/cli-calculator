use regex::Regex;

pub fn validate_pemdas_syntax(expression: &str) -> bool {
    let regex_pemdas = Regex::new(r"^-?\s*\d+(\s*[+\-*\/^]\s*-?\d+\s*)*$").unwrap();
    let valid_pemdas_syntax = regex_pemdas.is_match(expression);
    valid_pemdas_syntax
}
