#[derive(Debug, Clone, Copy)]
pub enum Style {
    Normal,
    Bold,
    Italic,
    Underline,
    Inverse,
    Strikethrough,
    Dim,
}

impl Style {
    pub fn code(&self) -> String {
        match &self {
            Style::Normal => "0".to_string(),
            Style::Bold => "1".to_string(),
            Style::Dim => "2".to_string(),
            Style::Italic => "3".to_string(),
            Style::Underline => "4".to_string(),
            Style::Inverse => "7".to_string(),
            Style::Strikethrough => "9".to_string(),
        }
    }
}