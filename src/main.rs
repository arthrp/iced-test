use iced::{ button, Button, Column, Text, Settings, Sandbox, Element, Align };

#[derive(Default)]
struct TestApp {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

fn main() {
    let _ = TestApp::run(Settings::default());
}

impl Sandbox for TestApp {
    type Message = Message;

    fn new() -> Self {
        return Self::default();
    }

    fn view(&mut self) -> Element<Message> {
        return Column::new()
            .padding(10)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment (+)")).on_press(Message::IncrementPressed)
            )
            .push(
                Text::new("Value: ".to_string()+&self.value.to_string()).size(48)
            )
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement (-)")).on_press(Message::DecrementPressed)
            )
            .into();
    }

    fn title(&self) -> String {
        return String::from("Test app in iced");
    }

    fn update(&mut self, message: Message) -> () {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }
}

