use iced::{button, executor, Button, Column, Command, Container, Element, Settings, Text};
use iced::{Application, Sandbox};

fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Application for Counter {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Counter {
                value: 0,
                increment_button: Default::default(),
                decrement_button: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::IncrementPressed => self.value += 1,
            Message::DecrementPressed => self.value -= 1,
        }
        Command::none()
    }
    fn view(&mut self) -> Element<Self::Message> {
        let increment_btn = btn("+", &mut self.increment_button, Message::IncrementPressed);
        let decrement_btn = btn("-", &mut self.decrement_button, Message::DecrementPressed);

        let columns = Column::new()
            .push(increment_btn)
            .push(Text::new(self.value.to_string()).size(50))
            .push(decrement_btn);

        Container::new(columns)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn btn<'a, Msg: Clone>(
    content: &'a str,
    state: &'a mut button::State,
    msg: Msg,
) -> Button<'a, Msg> {
    Button::new::<Element<Msg>>(state, Text::new(content).into()).on_press(msg)
}
