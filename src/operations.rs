#[derive(Debug, Clone, Copy)]
pub struct ParseResult {
    length: usize,
    pub value: f64,
}

pub fn parse_expression(input: &str) -> ParseResult {
    let mut result = parse_term(input);
    let mut term = result;
    let mut iter = input.chars();

    while let Some(char) = iter.nth(term.length) {
        match char {
            '+' => {
                term = parse_term(&input[result.length + 1..]);

                result.length += term.length + 1;
                result.value += term.value;
            },
            '-' => {
                term = parse_term(&input[result.length + 1..]);

                result.length += term.length + 1;
                result.value -= term.value;
            },
            _ => break
        }
    }
    result
}

fn parse_term(input: &str) -> ParseResult {
    let mut result = parse_factor(input);
    let mut factor = result;
    let mut iter = input.chars();

    while let Some(char) = iter.nth(factor.length) {
        match char {
            'x' => {
                factor = parse_factor(&input[result.length + 1..]);

                result.length += factor.length + 1;
                result.value *= factor.value;
            },
            '/' => {
                factor = parse_factor(&input[result.length + 1..]);

                result.length += factor.length + 1;
                result.value /= factor.value;
            },
            _ => break
        }
    }

    result
}

fn parse_factor(input: &str) -> ParseResult {
    let first_char = input.chars().next().unwrap();

    match first_char {
        char if char.is_digit(10) => read_number(input),
        '(' => {
            let closing_par_index = find_closing_parenthese(input);
            let sub_expr = &input[1..closing_par_index];
            let mut result = parse_expression(sub_expr);

            result.length = closing_par_index + 1;
            result
        },
        _ => unreachable!(),
    }
}


fn read_number(input: &str) -> ParseResult {
    let ops = ['+', '-', 'x', '/', '(', ')'];
    let length = input.find(&ops).unwrap_or(input.len());
    let value: f64 = input.split_once(&ops)
        .unwrap_or((&input, &""))
        .0
        .parse()
        .unwrap();

    ParseResult {
        length,
        value
    }
}

fn find_closing_parenthese(input: &str) -> usize {
    let mut opened_parenthese_count: usize = 0;
    let mut closed_parenthese_count: usize = 0;

    for (index, char) in input.chars().enumerate() {
        match char {
            '(' => opened_parenthese_count += 1,
            ')' => closed_parenthese_count += 1,
            _ => (),
        }
        if opened_parenthese_count == closed_parenthese_count {
            return index;
        }
    }
    panic!("Error while parsing parentheses");
}

#[cfg(test)]
mod tests {
    use crate::operations::parse_expression;

    #[test]
    fn parse_expression_tests() {
        let data: Vec<(&str, f64)> = vec![
            ("1+1", 2.0),
            ("1-1", 0.0),
            ("2x(3-2+(2/2))", 4.0),
            ("10/2", 5.0),
            ("1.5+2.3", 3.8),
            ("2.5x4", 10.0),
            ("100/4", 25.0),
            ("2.5-0.5", 2.0),
            ("1.1+2.2-3.0", 0.3),
            ("(2+3)x(1.5+0.5)", 10.0),
            ("3.3/1.1", 3.0),
            ("(2.2+3.3)x2", 11.0),
            ("2x(2.1+2.9/1.45)", 8.2),
            ("0.1+0.2", 0.3),
            ("5-(2x(1.25))", 2.5),
            ("(5-1)x(2+2)", 16.0),
            ("(2.5x4)/(1+1)", 5.0),
            ("1.2+2.3", 3.5),
            ("5x3", 15.0),
            ("10-5", 5.0),
            ("50/2", 25.0),
            ("(5+5)x2", 20.0),
            ("2x(3+3)", 12.0),
            ("(1.5+2.5)x4", 16.0),
            ("(12/4)x(2+1)", 9.0),
            ("3.5x(2+3)x2", 35.0),
            ("(4.5+1.5)x(2x3)", 36.0),
            ("(2.5x(2+3))/(1+1)", 6.25),
            ("2x(2+(3x2))/2", 8.0),
            ("((2+3)x2)x2", 20.0),
            ("(2+(3x(2+1)))", 11.0),
            ("(4/(2/1))+(3x2)", 8.0),
            ("((1.2+1.3)x2)-1", 4.0),
            ("1.1+2.2", 3.3),
            ("(1.5x4.2)+(3.1/2.0)", 7.85),
            ("(0.1+0.2)+0.3", 0.6),
            ("(10/3)x3", 10.0),
            ("2.5x3.2x(1+2)", 24.0),
            ("10x(2.5+0.5)-(3x2)", 24.0),
            ("(2.2+3.3)x(2x2.5)", 27.5),
            ("(4.5-1.5)/(1.5x0.5)", 4.0)
        ];
        
        for (input, expected_result) in data {
            let result = parse_expression(input).value;
            assert!((result - expected_result).abs() < 1e-6, "Failed test for input {input}: got {result} but expected {expected_result}");
        }
    }

}