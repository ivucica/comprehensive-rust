use crate::widget::Widget;
pub struct Label {
    label: String,
}

impl Label {
    pub fn new(label: &str) -> Label {
        Label { label: label.to_owned() }
    }
}


impl Widget for Label {
    fn width(&self) -> usize {
        self.label.lines().map(|line| line.chars().count()).max().unwrap_or(0)
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        writeln!(buffer, "{}", &self.label).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_width() {
        let label = Label::new("Test Label");
        assert_eq!(label.width(), 10);
    }

    #[test]
    fn test_draw_into() {
        let label = Label::new("Test Label");
        let mut buffer = String::new();
        label.draw_into(&mut buffer);
        assert_eq!(buffer.trim(), "Test Label");
    }
}
