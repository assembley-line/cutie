use std::fmt::format;
use cutie::color::Color;
use cutie::string::Cute;
use cutie::style::Style;

fn main() {
    const PADDING: usize = 20;
    const CONNECTOR: &str = "->";
    let styles_to_demo: Vec<Style> = vec![Style::Bold, Style::Italic, Style::Underline, Style::Strikethrough, Style::Dim, Style::Inverse];
    let colors_to_demo: Vec<Color> = vec![Color::Red, Color::Green, Color::Yellow, Color::Blue, Color::Magenta, Color::Cyan, Color::White, Color::Black];

    println!("{}", "Styles:".bold().underline());
    for style in &styles_to_demo {
        let style_str = format!("{:?}", style);
        println!("{:>PADDING$} {CONNECTOR} {}", format!(".{}()", style_str.to_lowercase()), style_str.style(style.clone()))
    }
    println!();
    println!("{}", "Colors:".bold().underline());
    for color in &colors_to_demo {
        let color_str = format!("{:?}", color);
        println!("{:>PADDING$} {CONNECTOR} {}", format!(".{}()", color_str.to_lowercase()), color_str.foreground(color.clone()))
    }
    println!();
    println!("{}", "Backgrounds:".bold().underline());
    for color in colors_to_demo {
        let color_str = format!("{:?}", color);
        println!("{:>PADDING$} {CONNECTOR} {}", format!(".over_{}()", color_str.to_lowercase()), color_str.background(color.clone()))
    }
}