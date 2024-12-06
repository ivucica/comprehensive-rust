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
