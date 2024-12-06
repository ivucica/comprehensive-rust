use crate::label::Label;
use crate::widget::Widget;

pub struct Button {
    label: Label,
}

impl Button {
    pub fn new(label: &str) -> Button {
        Button { label: Label::new(label) }
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + 8 // add a bit of padding
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width = self.width();
        let mut label = String::new();
        self.label.draw_into(&mut label);

        writeln!(buffer, "+{:-<width$}+", "").unwrap();
        for line in label.lines() {
            writeln!(buffer, "|{:^width$}|", &line).unwrap();
        }
        writeln!(buffer, "+{:-<width$}+", "").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::Widget;

    #[test]
    fn test_button_width() {
        let button = Button::new("Click me");
        let expected_width = "Click me".len() + 8;
        assert_eq!(button.width(), expected_width);
    }

    #[test]
    fn test_draw_into() {
        let button = Button::new("Click me");
        let mut buffer = String::new();
        button.draw_into(&mut buffer);
        // Verify that the buffer contains the expected output
        let expected_output = format!(
            "+{:-<width$}+\n|{:^width$}|\n+{:-<width$}+",
            "",
            "Click me",
            "",
            width = button.width()
        );
        assert_eq!(buffer.trim(), expected_output.trim());
    }
}
