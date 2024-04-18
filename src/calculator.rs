use fltk::{app::{self, App}, enums::{Align, Color, FrameType}, frame::Frame, image::PngImage, prelude::*, window::Window};
use fltk_theme::{ColorTheme, color_themes};
use regex::Regex;

use crate::{button::MyButton, operations::parse_expression};

pub mod settings {
    pub const WINDOW_WIDTH: i32 = 320;
    pub const WINDOW_HEIGHT: i32 = 350;
    pub const BUTTON_WIDTH: i32 = WINDOW_WIDTH / 4;
    pub const BUTTON_HEIGHT: i32 = 50;
    pub const RESULT_SCREEN_HEIGHT: i32 = WINDOW_HEIGHT - (BUTTON_HEIGHT * 5);
    pub const RESULT_HEIGHT: i32 = 60;
    pub const MEMORY_OFFSET: i32 = 20;
    pub const MEMORY_HEIGHT: i32 = RESULT_SCREEN_HEIGHT - RESULT_HEIGHT - MEMORY_OFFSET;
    pub const RESULT_LABEL_SIZE: i32 = 28;
}

#[derive(Debug, PartialEq, Clone)]
pub enum MessageEmit {
    Number(u32),
    Operator(char),
    Equal,
    Clear,
    Delete,
    Dot,
    Parentheses(char),
}

pub struct CalculatorApp {
    calculator: App,
    main_window: Window,
    theme: ColorTheme,
    buttons: Vec<MyButton>,
    result_output: Frame,
    memory_output: Frame
}

impl CalculatorApp {
    pub fn new() -> CalculatorApp {
        CalculatorApp {
            calculator: app::App::default().with_scheme(app::Scheme::Gtk),
            main_window:  Window::default()
                .with_label("Quantum Calculator 2000")
                .with_size(settings::WINDOW_WIDTH, settings::WINDOW_HEIGHT)
                .center_screen(),
            theme: ColorTheme::new(color_themes::BLACK_THEME),
            buttons: vec![],
            result_output: Frame::new(
                0, 
                settings::MEMORY_HEIGHT + settings::MEMORY_OFFSET, 
                settings::WINDOW_WIDTH, 
                settings::RESULT_HEIGHT, 
                ""
            ).with_align(Align::Right | Align::Inside),
            memory_output: Frame::new(
                0, 
                settings::MEMORY_OFFSET, 
                settings::WINDOW_WIDTH, 
                settings::MEMORY_HEIGHT, 
                ""
            ).with_align(Align::Right | Align::Inside)
        }
    }

    pub fn init_gui(&mut self) {
        self.init_outputs();
        self.init_buttons();

        if let Ok(icon) = PngImage::load("./src/assets/logo.png") {
            self.main_window.set_icon(Some(icon));
        } else {
            eprintln!("Error while loading the icon");
        }

        self.main_window.end();
        self.main_window.show();
        self.theme.apply();
    }

    pub fn run(&mut self) {
        let (_s, r) = app::channel::<MessageEmit>();

        while self.calculator.wait() {
            if let Some(msg) = r.recv() {
                let mut output = self.get_trimmed_output();

                match msg {
                    MessageEmit::Number(num) => output = self.handle_message_number(output, num),
                    MessageEmit::Clear => output = self.handle_message_clear(),
                    MessageEmit::Delete => output = self.handle_message_delete(output),
                    MessageEmit::Dot => output = self.handle_message_dot(output),
                    MessageEmit::Operator(op) => output = self.handle_message_operator(output, op),
                    MessageEmit::Parentheses(par) => output = self.handle_message_parentheses(output, par),
                    MessageEmit::Equal => output = self.handle_message_equal(output)
                }
                output = self.format_result_output(output);
                self.result_output.set_label(&output);
            }
        }
                
    }

    fn handle_message_equal(&mut self, mut output: String) -> String {
        output = output
            .trim_end_matches(|c| c == '(' || c == '+' || c == '-' || c == '/' || c == 'x' || c == '.' || c == ' ')
            .to_string();

        let opened_par_count = output.chars().filter(|c| *c == '(').count();
        let mut closed_par_count = output.chars().filter(|c| *c == ')').count();

        while closed_par_count < opened_par_count {
            output.push(')');
            closed_par_count += 1;
        }

        let result: f64 = parse_expression(&output).value;
        // println!()

        output = self.format_result_output(output);
        output.push_str("=  ");
        self.memory_output.set_label(&output);

        result.to_string()
    }

    fn handle_message_parentheses(&self, mut output: String, par: char) -> String {
        if par == ')' {
            let opened_par_count = output.chars().filter(|c| *c == '(').count();
            let closed_par_count = output.chars().filter(|c| *c == ')').count();
            
            if opened_par_count > closed_par_count {
                if let Some(last_char) = output.chars().last() {
                    if self.is_an_operator(last_char) {
                        output.pop();
                    }
                }
                output.push(par);
            }
        } else {
            if output == "0" {
                output = "(".to_string();
            } else {
                output.push(par);
            }
        }

        output
    }

    fn handle_message_dot(&self, mut output: String) -> String {
        if let Some(last_char) = output.chars().last() {
            if self.is_an_operator(last_char) {
                output.insert_str(output.len(), ".");
            } else {
                let last_elem: String = output
                    .split(|c: char| c == 'x' || c == '+' || c == '/' || c == '-')
                    .last()
                    .unwrap()
                    .to_string();
            
                if last_elem
                    .chars()
                    .filter(|c| *c == '.')
                    .count() == 0 
                {
                    output.insert(output.len(), '.');
                }
            }
        }

        output
    }

    fn handle_message_operator(&self, mut output: String, op: char) -> String {
        if let Some(last_char) = output.chars().last() {
            if self.is_an_operator(last_char) {
                output.pop();
            } else if last_char == '(' {
                output.push('0');
            }
            output = format!("{output}{op}");
        }

        output
    }

    fn handle_message_number(&self, mut output: String, num: u32) -> String {
        let output_len = output.len();
                        
        if output == "0" {
            output = format!("{num}");
        } else if output_len <= 100 { // A CHANGER
            output.insert(output_len, char::from_digit(num, 10).unwrap());
        };

        output
    }

    fn handle_message_clear(&mut self) -> String {
        self.memory_output.set_label("");
        "0".to_string()
    }

    fn handle_message_delete(&self, mut output: String) -> String {
        if output != "0" {
            output.pop();
        }

        if output.is_empty() {
            output = "0".to_string();
        }

        output
    }

    fn get_trimmed_output(&self) -> String {
        self.result_output.label().replace(' ', "")
    }    

    fn format_result_output(&mut self, raw_output: String) -> String {
        let mut formated_output = String::from("");
        let segments: Vec<&str> = raw_output
            .split_inclusive(|c: char| c == 'x' || c == '+' || c == '/' || c == '-' || c == '(' || c == ')')
            .collect();

        for elem in segments {
            let trimmed_number = elem.trim_end_matches(|c| c == 'x' || c == '+' || c == '/' || c == '-' || c == '(' || c == ')');

            // Add spaces to integer part if needed every 3 decimals
            formated_output.push_str(&self.format_number(trimmed_number));
            // Add spaces around operators
            formated_output.push_str(&self.add_spaces_around_operators(elem));
        }

        self.replace_patterns_and_add_last_space(formated_output)
    }

    fn replace_patterns_and_add_last_space(&self, mut formated_output: String) -> String {
        if let Some(c) = formated_output.chars().last() {
            if c != ' ' {
                formated_output.push(' ');
            }
        } 

        formated_output = formated_output.replace("  ", " ");
        formated_output = formated_output.replace(" .", " 0.");
        formated_output = formated_output.replace("(.", "(0.");
        formated_output = formated_output.replace(") (", ") x (");
        formated_output = formated_output.replace("()", "(1)");

        let mut regex = Regex::new(r"(?<n>\d) \(").unwrap();
        formated_output = regex.replace_all(&formated_output, r"$n x (").to_string();
        
        regex = Regex::new(r"\) (?<n>\d)").unwrap();
        formated_output = regex.replace_all(&formated_output, r") x $n").to_string();

        formated_output
    }

    fn add_spaces_around_operators(&self, segment: &str) -> String {
        let last_char: char = segment.chars().last().unwrap();

        if self.is_an_operator(last_char) {
            format!(" {last_char} ")
        } else if last_char == '(' {
            format!(" {last_char}")
        } else if last_char == ')' {
            format!("{last_char} ")
        } else {
            "".to_string()
        }
    }

    fn format_number(&self, number: &str) -> String {
        match number.split_once('.') {
            Some((integer_part, decimal_part)) => {
                let formated_integer_part: String = self.format_integer_part(integer_part);

                format!("{formated_integer_part}.{decimal_part}")
            },
            None => {
                let formated_integer_part: String = self.format_integer_part(number);

                format!("{formated_integer_part}")
            } 
        }
    }

    fn format_integer_part(&self, integer_part: &str) -> String {
        let mut formated_integer: String = String::from(integer_part);
        let mut add_space_index = formated_integer.len() as i8 - 3;

        while add_space_index > 0 {
            formated_integer.insert(add_space_index as usize, ' ');
            add_space_index -= 3;
        } 

        formated_integer
    }

    fn init_outputs(&mut self) {
        self.result_output.set_frame(FrameType::FlatBox);
        self.result_output.set_label_color(Color::from_rgb(200, 200, 200));
        self.result_output.set_label_size(settings::RESULT_LABEL_SIZE);
        self.result_output.set_label("0 ");
        self.memory_output.set_frame(FrameType::FlatBox);
    }

    fn init_buttons(&mut self) {
        let sequence = "()Cd789/456x123-.0=+";
        let mut index: usize = 0;
        let step_x: i32 = settings::WINDOW_WIDTH / 4;
        let step_y: i32 = settings::BUTTON_HEIGHT;
        let offset_y: i32 = settings::WINDOW_HEIGHT - (settings::BUTTON_HEIGHT * 5);
    
        for y in 0..5 {
            for x in 0..4 {
                if index >= sequence.chars().count() {
                    break;
                }
    
                let pos_x = x * step_x;
                let pos_y = y * step_y + offset_y;
                let character = sequence.chars().nth(index).unwrap();
                
                self.buttons.push(MyButton::new(character, (pos_x, pos_y)));
                index += 1;
            }
        }
    }

    fn is_an_operator(&self, op: char) -> bool {
        op == 'x' || op == '+' || op == '-' || op == '/'
    }
}


#[cfg(test)]
mod tests {
    use fltk::prelude::WidgetExt;
    use super::CalculatorApp;

    #[test]
    fn get_trimmed_output_tests() {
        let mut calculator = CalculatorApp::new();

        let data = vec![
            (String::from("1 123 456 + 156 - 15 / .23 x 9 156 + 0 "), String::from("1123456+156-15/.23x9156+0")),
            (String::from("0 + 5 - 1 123 / "), String::from("0+5-1123/")),
            (String::from("0 "), String::from("0")),
        ];

        for (input, expected_output) in data {
            calculator.result_output.set_label(&input);
            
            assert_eq!(calculator.get_trimmed_output(), expected_output);
        }
    }

    #[test]
    fn handle_message_number_tests() {
        let calculator = CalculatorApp::new();

        let data = vec![
            (String::from("0"), 1, String::from("1")),
            (String::from("1"), 1, String::from("11")),
            (String::from("15+"), 1, String::from("15+1")),
            (String::from("15-"), 1, String::from("15-1")),
            (String::from("15."), 1, String::from("15.1")),
            (String::from("0"), 0, String::from("0")),
        ];

        for (input, num, expected_output) in data {
            let output = calculator.handle_message_number(input, num);
            
            assert_eq!(output, expected_output);
        }
    }
    
    #[test]
    fn handle_message_clear_tests() {
        let mut calculator = CalculatorApp::new();

        let data = vec![
            (String::from("0"), String::from("0")),
            (String::from("1564"), String::from("0")),
            (String::from("15+"), String::from("0")),
            (String::from("0.226x"), String::from("0")),
        ];

        for (input, expected_output) in data {
            calculator.result_output.set_label(&input);
            
            assert_eq!(calculator.handle_message_clear(), expected_output);
        }
    }
    
    #[test]
    fn handle_message_delete_tests() {
        let calculator = CalculatorApp::new();

        let data = vec![
            (String::from("0"), String::from("0")),
            (String::from("1564"), String::from("156")),
            (String::from("15+"), String::from("15")),
            (String::from("0.226x"), String::from("0.226")),
            (String::from("0.226"), String::from("0.22")),
            (String::from("0.2"), String::from("0.")),
            (String::from("1"), String::from("0")),
        ];

        for (input, expected_output) in data {
            assert_eq!(calculator.handle_message_delete(input), expected_output);
        }
    }
    
    #[test]
    fn handle_message_dot_tests() {
        let calculator = CalculatorApp::new();

        let data = vec![
            (String::from("0"), String::from("0.")),
            (String::from("1564"), String::from("1564.")),
            (String::from("15+"), String::from("15+.")),
            (String::from("0.226x"), String::from("0.226x.")),
            (String::from("0.226"), String::from("0.226")),
            (String::from("10.15+0.226"), String::from("10.15+0.226")),
            (String::from("10.15+0.226/1."), String::from("10.15+0.226/1.")),
            (String::from("10.15+26"), String::from("10.15+26.")),
            (String::from("0.2"), String::from("0.2")),
            (String::from("1"), String::from("1.")),
        ];

        for (input, expected_output) in data {
            assert_eq!(calculator.handle_message_dot(input), expected_output);
        }
    }

    #[test]
    fn handle_message_operator_tests() {
        let calculator = CalculatorApp::new();

        let data = vec![
            (String::from("0"), '+', String::from("0+")),
            (String::from("156+"), '-', String::from("156-")),
            (String::from("15+"), 'x', String::from("15x")),
            (String::from("0.226/"), '/', String::from("0.226/")),
            (String::from("0.226"), '+', String::from("0.226+")),
            (String::from("0."), '+', String::from("0.+")),
            (String::from("1"), '+', String::from("1+")),
        ];

        for (input, op, expected_output) in data {
            assert_eq!(calculator.handle_message_operator(input, op), expected_output);
        }
    }

    #[test]
    fn format_result_output_tests() {
        let mut calculator = CalculatorApp::new();

        let data = vec![
            (String::from("0"), String::from("0 ")),
            (String::from("156+"), String::from("156 + ")),
            (String::from("15-"), String::from("15 - ")),
            (String::from("15x"), String::from("15 x ")),
            (String::from("1(15"), String::from("1 x (15 ")),
            (String::from("15)1"), String::from("15) x 1 ")),
            (String::from("0.226/"), String::from("0.226 / ")),
            (String::from("0.226/15+1-"), String::from("0.226 / 15 + 1 - ")),
            (String::from("(0.226/(15+1))-"), String::from(" (0.226 / (15 + 1) ) - ")),
            (String::from("0.226666666"), String::from("0.226666666 ")),
            (String::from("0.0"), String::from("0.0 ")),
            (String::from("()"), String::from(" (1) ")),
            (String::from("1000000"), String::from("1 000 000 ")),
            (String::from("1000000.55"), String::from("1 000 000.55 ")),
            (String::from("1000000.55555555"), String::from("1 000 000.55555555 ")),
            (String::from("1000000.55555555+.155555"), String::from("1 000 000.55555555 + 0.155555 ")),
            (String::from("100000.55555555+.155555/123456+1500.1568"), String::from("100 000.55555555 + 0.155555 / 123 456 + 1 500.1568 ")),
            (String::from("100000.55555555+(.155555/123456)+1500.1568"), String::from("100 000.55555555 + (0.155555 / 123 456) + 1 500.1568 ")),
        ];

        for (input, expected_output) in data {
            let output = calculator.format_result_output(input);

            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn handle_message_parentheses_tests() {
        let calculator = CalculatorApp::new();

        let data = vec![
            (String::from("0"), '(', String::from("(")),
            (String::from("156+"), '(', String::from("156+(")),
            (String::from("15-"), '(', String::from("15-(")),
            (String::from("(15-"), ')', String::from("(15)")),
            (String::from("(15x"), '(', String::from("(15x(")),
            (String::from("0.226/"), '(', String::from("0.226/(")),
            (String::from("0.226/15+1-"), ')', String::from("0.226/15+1-")),
            (String::from("(0.226666666"), ')', String::from("(0.226666666)")),
            (String::from("((0.0"), ')', String::from("((0.0)")),
            (String::from("(1000000)"), ')', String::from("(1000000)")),
            (String::from("((1000000.55"), ')', String::from("((1000000.55)")),
            (String::from("(2-"), ')', String::from("(2)")),

        ];

        for (input, par, expected_output) in data {
            assert_eq!(calculator.handle_message_parentheses(input, par), expected_output);
        }
    }
}
