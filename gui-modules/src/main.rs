mod widget;
mod label;
mod button;
mod window;

use crate::label::Label;
use crate::widget::Widget;
use crate::button::Button;
use crate::window::Window;

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new("Click me!")));
    window.draw();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::window::Window;
    use crate::label::Label;
    use crate::button::Button;

    #[test]
    fn test_main_window() {
        let mut window = Window::new("Test Window");
        window.add_widget(Box::new(Label::new("Test Label")));
        window.add_widget(Box::new(Button::new("Test Button")));

        let mut buffer = String::new();
        window.draw_into(&mut buffer);

        // Verify that the window renders correctly
        assert!(buffer.contains("Test Window"));
        assert!(buffer.contains("Test Label"));
        assert!(buffer.contains("Test Button"));
    }
}
