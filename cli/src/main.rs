use std::fmt::{Arguments, Display};

use clap::{Parser, Subcommand};
use termion::color;
use trex_parser::{error::Result, Color, Format, Regex, Style};

const _TOML: &'static str = include_str!("../Cargo.toml");

/// ────────────████████{n}
/// ──────────███▄███████{n}
/// ──────────███████████{n}
/// ──────────███████████{n}
/// ──────────██████     {n}
/// ──────────█████████{n}
/// █───────███████{n}
/// ██────████████████{n}
/// ███──██████████──█{n}
/// ███████████████             Terminal Regular Expression{n}
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Parses a regular expression
    Parse {
        expression: String,
        #[arg(short, long)]
        ignore_case: bool,
        #[arg(short, long)]
        multiline: bool,
    },
}

fn to_termion_color(color: &Color) -> Box<dyn termion::color::Color> {
    match color {
        Color::Reset => Box::new(termion::color::Reset),
        Color::Black => Box::new(termion::color::Black),
        Color::Red => Box::new(termion::color::Red),
        Color::Yellow => Box::new(termion::color::Yellow),
        Color::Blue => Box::new(termion::color::Blue),
        Color::Magenta => Box::new(termion::color::Magenta),
        Color::Cyan => Box::new(termion::color::Cyan),
        Color::White => Box::new(termion::color::White),
        Color::LightBlack => Box::new(termion::color::LightBlack),
        Color::LightRed => Box::new(termion::color::LightRed),
        Color::LightGreen => Box::new(termion::color::LightGreen),
        Color::LightYellow => Box::new(termion::color::LightYellow),
        Color::LightBlue => Box::new(termion::color::LightBlue),
        Color::LightMagenta => Box::new(termion::color::LightMagenta),
        Color::LightCyan => Box::new(termion::color::LightCyan),
        Color::LightWhite => Box::new(termion::color::LightWhite),
        _ => panic!("unexpected value"),
    }
}

fn to_termion_style(format: &Format) -> Box<dyn Display> {
    match format {
        Format::Reset => Box::new(termion::style::Reset),
        Format::Bold => Box::new(termion::style::Bold),
        Format::Dim => Box::new(termion::style::Faint),
        Format::Underline => Box::new(termion::style::Framed),
        Format::Reverse => Box::new(termion::style::Invert),
        Format::Italic => Box::new(termion::style::Italic),
        _ => panic!("unexpected value"),
    }
}

fn format_fg(style: &Style, arg: &Arguments<'_>) -> String {
    match style.foreground {
        Color::Inherit => format_bg(style, arg),
        color => format!(
            "{}{}",
            termion::color::Fg(to_termion_color(&color).as_ref()),
            format_bg(style, arg)
        ),
    }
}

fn format_bg(style: &Style, arg: &Arguments<'_>) -> String {
    match style.background {
        Color::Inherit => format_style(style, arg),
        color => format!(
            "{}{}",
            termion::color::Bg(to_termion_color(&color).as_ref()),
            format_style(style, arg)
        ),
    }
}

fn format_style(style: &Style, arg: &Arguments<'_>) -> String {
    match style.format {
        Format::Inherit => format!("{arg}"),
        format => format!("{}{}", to_termion_style(&format), format!("{arg}")),
    }
}

fn termion_style(style: &Style, arg: &Arguments<'_>) -> String {
    format_fg(style, arg)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { expression, .. } => {
            let re: Regex = expression.parse()?;
            println!("{}", re.with_style(termion_style));
            Ok(())
        }
    }
}
