use iced::widget::{button, container};
use iced::{executor, Application, Command, Element, Settings, Theme};

fn main() -> iced::Result {
    Tiger::run(Settings::default())
}

#[derive(Debug, Default)]
struct Tiger;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    //ToggleColorFilter(bool),
}

impl Application for Tiger {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;
    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Tiger::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("SVG - Iced")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        //let handle = svg::Handle::from_memory(IMAGE);
        container(button("|>")).center_x().into()
    }
}
