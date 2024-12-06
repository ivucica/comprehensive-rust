use crate::Widget;

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    pub fn new(title: &str) -> Window {
        Window { title: title.to_owned(), widgets: Vec::new() }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    pub fn inner_width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        // Add 4 paddings for borders
        self.inner_width() + 4
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let mut inner = String::new();
        for widget in &self.widgets {
            widget.draw_into(&mut inner);
        }

        let inner_width = self.inner_width();

        // TODO: Change draw_into to return Result<(), std::fmt::Error>. Then use the
        // ?-operator here instead of .unwrap().
        writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap();
        writeln!(buffer, "| {:^inner_width$} |", &self.title).unwrap();
        writeln!(buffer, "+={:=<inner_width$}=+", "").unwrap();
        for line in inner.lines() {
            writeln!(buffer, "| {:inner_width$} |", line).unwrap();
        }
        writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::label::Label;
    use crate::button::Button;
    use crate::widget::Widget;

    #[test]
    fn test_window_inner_width() {
        let mut window = Window::new("My Window");
        window.add_widget(Box::new(Label::new("Short")));
        window.add_widget(Box::new(Label::new("A much longer label")));
        assert_eq!(window.inner_width(), "A much longer label".len());
    }

    #[test]
    fn test_window_width() {
        let window = Window::new("My Window");
        let expected_width = window.inner_width() + 4;
        assert_eq!(window.width(), expected_width);
    }

    #[test]
    fn test_draw_into() {
        let mut window = Window::new("My Window");
        window.add_widget(Box::new(Label::new("Content")));
        window.add_widget(Box::new(Button::new("Click me")));

        let mut buffer = String::new();
        window.draw_into(&mut buffer);

        // Check that the buffer contains the window title and widget outputs
        assert!(buffer.contains("My Window"));
        assert!(buffer.contains("Content"));
        assert!(buffer.contains("Click me"));
    }
}
