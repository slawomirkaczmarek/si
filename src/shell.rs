use std::fmt::Display;

use anstyle::{AnsiColor, Color, Style};

const STATUS_STYLE: Style = Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Magenta)));
const ERROR_STYLE: Style = Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Red)));

pub fn print_status(status: impl Display, message: impl Display) {
    anstream::println!("{STATUS_STYLE}{:>12}{STATUS_STYLE:#} {}", status, message);
}

pub fn print_error(status: impl Display, message: impl Display) {
    anstream::eprintln!("{ERROR_STYLE}{:>12}{ERROR_STYLE:#} {}", status, message);
}
