use fltk::app;
use fltk::enums::{Color, Key, Shortcut};
use fltk::prelude::{ButtonExt, WidgetExt};
use fltk::{button::Button, prelude::WidgetBase};

use crate::calculator::{settings, MessageEmit};

pub struct MyButton {
    // button: Button
}

impl MyButton {
    pub fn new(c: char, position: (i32, i32)) -> MyButton {
        let (s, _r) = app::channel::<MessageEmit>();
        let mut value: String = c.to_string();
        let mut key: Key = Key::from_char(c);
       
        if c == 'd' {
            value = "@<-".to_string();
            key = Key::BackSpace;
        }
        
        let mut button = Button::new(
            position.0, 
            position.1,
            settings::BUTTON_WIDTH, 
            settings::BUTTON_HEIGHT, 
            value.as_str()
        );

        let message = match c {
            '0'..='9' => {
                button.set_color(button.color().darker());
                button.set_color(button.color().darker());
                button.set_color(button.color().darker());
                button.set_label_color(Color::from_rgb(200, 200, 200));
                MessageEmit::Number(c.to_digit(10).unwrap() as u32)
            },
            '+' | '-' | 'x' | '/' => {
                if c == 'x' {
                    key = Key::from_char('*');
                }
                MessageEmit::Operator(c)
            },
            '=' => {
                button.set_color(Color::from_rgb(222,113,40));
                button.set_label_color(Color::from_rgb(50, 50, 50));
                button.set_selection_color(Color::from_rgb(107, 82, 65));
                key = Key::Enter;
                MessageEmit::Equal
            },
            'C' => MessageEmit::Clear,
            'd' => MessageEmit::Delete,
            '.' => MessageEmit::Dot,
            '(' | ')' => MessageEmit::Parentheses(c),
            _ => panic!("Unexpected button type: {}", c),
        };

        button.set_shortcut(Shortcut::None | key);
        button.visible_focus(false);
        button.emit(s, message);

        MyButton {}
    }
}

