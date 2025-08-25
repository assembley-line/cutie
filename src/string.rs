use std::fmt::{Display, Formatter};
use crate::color::Color;
use crate::style::Style;

#[derive(Debug)]
pub struct CuteString {
    string: String,
    design: Design
}

#[derive(Debug)]
struct Design {
    colors: Colors,
    styles: Vec<Style>
}

#[derive(Debug)]
struct Colors {
    foreground: Option<Color>,
    background: Option<Color>,
}

pub trait Decorate {
    fn add_style(self, style: Style) -> CuteString;
    fn add_foreground(self, foreground: Color) -> CuteString;
    fn add_background(self, background: Color) -> CuteString;
}

pub trait Cute {
    fn foreground(self, color: Color) -> CuteString;
    fn background(self, color: Color) -> CuteString;
    fn style(self, style: Style) -> CuteString;

    fn bold(self) -> CuteString;
    fn dim(self) -> CuteString;
    fn italic(self) -> CuteString;
    fn underline(self) -> CuteString;
    fn inverse(self) -> CuteString;
    fn strikethrough(self) -> CuteString;

    fn black(self) -> CuteString;
    fn red(self) -> CuteString;
    fn green(self) -> CuteString;
    fn yellow(self) -> CuteString;
    fn blue(self) -> CuteString;
    fn magenta(self) -> CuteString;
    fn cyan(self) -> CuteString;
    fn white(self) -> CuteString;

    fn bright_black(self) -> CuteString;
    fn bright_red(self) -> CuteString;
    fn bright_green(self) -> CuteString;
    fn bright_yellow(self) -> CuteString;
    fn bright_blue(self) -> CuteString;
    fn bright_magenta(self) -> CuteString;
    fn bright_cyan(self) -> CuteString;
    fn bright_white(self) -> CuteString;

    fn extended_color(self, id: u8) -> CuteString;
    fn truecolor(self, r: u8, g: u8, b: u8) -> CuteString;

    fn over_black(self) -> CuteString;
    fn over_red(self) -> CuteString;
    fn over_green(self) -> CuteString;
    fn over_yellow(self) -> CuteString;
    fn over_blue(self) -> CuteString;
    fn over_magenta(self) -> CuteString;
    fn over_cyan(self) -> CuteString;
    fn over_white(self) -> CuteString;

    fn over_bright_black(self) -> CuteString;
    fn over_bright_red(self) -> CuteString;
    fn over_bright_green(self) -> CuteString;
    fn over_bright_yellow(self) -> CuteString;
    fn over_bright_blue(self) -> CuteString;
    fn over_bright_magenta(self) -> CuteString;
    fn over_bright_cyan(self) -> CuteString;
    fn over_bright_white(self) -> CuteString;

    fn over_extended_color(self, id: u8) -> CuteString;
    fn over_truecolor(self, r: u8, g: u8, b: u8) -> CuteString;
}

impl Decorate for CuteString {
    fn add_style(self, style: Style) -> CuteString {
        let mut new_styles = self.design.styles.clone();
        new_styles.push(style);
        CuteString {
            string: self.string,
            design: Design {
                styles: new_styles,
                colors: Colors {
                    foreground: self.design.colors.foreground,
                    background: self.design.colors.background,
                }
            }
        }
    }
    fn add_foreground(self, foreground: Color) -> CuteString {
        CuteString {
            string: self.string,
            design: Design {
                styles: self.design.styles,
                colors: Colors {
                    foreground: Some(foreground),
                    background: self.design.colors.background,
                }
            }
        }
    }
    fn add_background(self, background: Color) -> CuteString {
        CuteString {
            string: self.string,
            design: Design {
                styles: self.design.styles,
                colors: Colors {
                    foreground: self.design.colors.foreground,
                    background: Some(background),
                }
            }
        }
    }
}

impl Decorate for String {
    fn add_style(self, style: Style) -> CuteString {
        CuteString {
            string: self,
            design: Design {
                styles: vec![style],
                colors: Colors {
                    foreground: None,
                    background: None,
                }
            }
        }
    }
    fn add_foreground(self, foreground: Color) -> CuteString {
        CuteString {
            string: self,
            design: Design {
                styles: vec![],
                colors: Colors {
                    foreground: Some(foreground),
                    background: None,
                }
            }
        }
    }
    fn add_background(self, background: Color) -> CuteString {
        CuteString {
            string: self,
            design: Design {
                styles: vec![],
                colors: Colors {
                    foreground: None,
                    background: Some(background),
                }
            }
        }
    }
}

impl Decorate for &String {
    fn add_style(self, style: Style) -> CuteString {
        CuteString {
            string: self.clone(),
            design: Design {
                styles: vec![style],
                colors: Colors {
                    foreground: None,
                    background: None,
                }
            }
        }
    }
    fn add_foreground(self, foreground: Color) -> CuteString {
        CuteString {
            string: self.clone(),
            design: Design {
                styles: vec![],
                colors: Colors {
                    foreground: Some(foreground),
                    background: None,
                }
            }
        }
    }
    fn add_background(self, background: Color) -> CuteString {
        CuteString {
            string: self.clone(),
            design: Design {
                styles: vec![],
                colors: Colors {
                    foreground: None,
                    background: Some(background),
                }
            }
        }
    }
}

impl Decorate for &str {
    fn add_style(self, style: Style) -> CuteString {
        CuteString {
            string: self.to_string(),
            design: Design {
                styles: vec![style],
                colors: Colors {
                    foreground: None,
                    background: None,
                }
            }
        }
    }
    fn add_foreground(self, foreground: Color) -> CuteString {
        CuteString {
            string: self.to_string(),
            design: Design {
                styles: vec![],
                colors: Colors {
                    foreground: Some(foreground),
                    background: None,
                }
            }
        }
    }
    fn add_background(self, background: Color) -> CuteString {
        CuteString {
            string: self.to_string(),
            design: Design {
                styles: vec![],
                colors: Colors {
                    foreground: None,
                    background: Some(background),
                }
            }
        }
    }
}

/// Blanket implementation for any item that has the Decorate trait as it can use Cute methods
impl<T: Decorate> Cute for T {
    fn foreground(self, color: Color) -> CuteString {
        self.add_foreground(color)
    }
    fn background(self, color: Color) -> CuteString {
        self.add_background(color)
    }
    fn style(self, style: Style) -> CuteString {
        self.add_style(style)
    }

    fn bold(self) -> CuteString {
        self.add_style(Style::Bold)
    }
    fn dim(self) -> CuteString {
        self.add_style(Style::Dim)
    }
    fn italic(self) -> CuteString {
        self.add_style(Style::Italic)
    }
    fn underline(self) -> CuteString {
        self.add_style(Style::Underline)
    }
    fn inverse(self) -> CuteString {
        self.add_style(Style::Inverse)
    }
    fn strikethrough(self) -> CuteString {
        self.add_style(Style::Strikethrough)
    }

    fn black(self) -> CuteString {
        self.add_foreground(Color::Black)
    }
    fn red(self) -> CuteString {
        self.add_foreground(Color::Red)
    }
    fn green(self) -> CuteString {
        self.add_foreground(Color::Green)
    }
    fn yellow(self) -> CuteString {
        self.add_foreground(Color::Yellow)
    }
    fn blue(self) -> CuteString {
        self.add_foreground(Color::Blue)
    }
    fn magenta(self) -> CuteString {
        self.add_foreground(Color::Magenta)
    }
    fn cyan(self) -> CuteString {
        self.add_foreground(Color::Cyan)
    }
    fn white(self) -> CuteString {
        self.add_foreground(Color::White)
    }

    fn bright_black(self) -> CuteString {
        self.add_foreground(Color::BrightBlack)
    }
    fn bright_red(self) -> CuteString {
        self.add_foreground(Color::BrightRed)
    }
    fn bright_green(self) -> CuteString {
        self.add_foreground(Color::BrightGreen)
    }
    fn bright_yellow(self) -> CuteString {
        self.add_foreground(Color::BrightYellow)
    }
    fn bright_blue(self) -> CuteString {
        self.add_foreground(Color::BrightBlue)
    }
    fn bright_magenta(self) -> CuteString {
        self.add_foreground(Color::BrightMagenta)
    }
    fn bright_cyan(self) -> CuteString {
        self.add_foreground(Color::BrightCyan)
    }
    fn bright_white(self) -> CuteString {
        self.add_foreground(Color::BrightWhite)
    }

    fn extended_color(self, id: u8) -> CuteString {
        self.add_foreground(Color::ExtendedColor(id))
    }
    fn truecolor(self, r: u8, g: u8, b: u8) -> CuteString {
        self.add_foreground(Color::TrueColor { r, g, b })
    }

    fn over_black(self) -> CuteString {
        self.add_background(Color::Black)
    }
    fn over_red(self) -> CuteString {
        self.add_background(Color::Red)
    }
    fn over_green(self) -> CuteString {
        self.add_background(Color::Green)
    }
    fn over_yellow(self) -> CuteString {
        self.add_background(Color::Yellow)
    }
    fn over_blue(self) -> CuteString {
        self.add_background(Color::Blue)
    }
    fn over_magenta(self) -> CuteString {
        self.add_background(Color::Magenta)
    }
    fn over_cyan(self) -> CuteString {
        self.add_background(Color::Cyan)
    }
    fn over_white(self) -> CuteString {
        self.add_background(Color::White)
    }

    fn over_bright_black(self) -> CuteString {
        self.add_background(Color::BrightBlack)
    }
    fn over_bright_red(self) -> CuteString {
        self.add_background(Color::BrightRed)
    }
    fn over_bright_green(self) -> CuteString {
        self.add_background(Color::BrightGreen)
    }
    fn over_bright_yellow(self) -> CuteString {
        self.add_background(Color::BrightYellow)
    }
    fn over_bright_blue(self) -> CuteString {
        self.add_background(Color::BrightBlue)
    }
    fn over_bright_magenta(self) -> CuteString {
        self.add_background(Color::BrightMagenta)
    }
    fn over_bright_cyan(self) -> CuteString {
        self.add_background(Color::BrightCyan)
    }
    fn over_bright_white(self) -> CuteString {
        self.add_background(Color::BrightWhite)
    }

    fn over_extended_color(self, id: u8) -> CuteString {
        self.add_background(Color::ExtendedColor(id))
    }
    fn over_truecolor(self, r: u8, g: u8, b: u8) -> CuteString {
        self.add_background(Color::TrueColor { r, g, b })
    }
}

impl Display for CuteString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}{1}{2}", generate_code(&self.design), self.string, "\x1B[0m") // Add a dedicated reset code after the string
    }
}

fn generate_code(design: &Design) ->  String {
    let mut styles: Vec<String> = design.styles.iter().map(|s| s.code()).collect();
    let mut colors: Vec<String> = vec![];
    if let Some(color) = &design.colors.foreground {
        colors.push(color.foreground())
    }
    if let Some(color) = &design.colors.background {
        colors.push(color.background())
    }
    styles.append(&mut colors);

    "\x1B[".to_string() + &styles.join(";") + "m"
}