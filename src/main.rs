use iced::futures::SinkExt;
use iced::widget::{button, container};
use iced::{
    executor, subscription, Application, Command, Element, Length, Settings, Subscription, Theme,
};

use std::sync::mpsc;

mod player;

#[allow(unused)]
#[derive(Debug, Default)]
struct InitFlag {
    url: String,
}

fn main() -> iced::Result {
    Tiger::run(Settings {
        flags: InitFlag {
            url:
                "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/TearsOfSteel.mp4"
                    .to_string(),
        },
        ..Default::default()
    })
}

#[derive(Debug)]
struct Tiger {
    player: player::Player,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RequestStart,
    FFMpeg(FFMpegEvent),
}

#[derive(Debug, Clone)]
enum FFMpegMessages {
    Data(Vec<u8>),
}

#[derive(Debug, Clone, Copy)]
enum FFMpegEvent {
    Frame,
}

impl Application for Tiger {
    type Message = Message;
    type Flags = InitFlag;
    type Executor = executor::Default;
    type Theme = Theme;
    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let (listener, sender) = mpsc::channel::<FFMpegMessages>();
        let url = flags.url;
        let player = player::Player::start(
            url.into(),
            move |newframe| {
                println!("eee");
            },
            move |playing| {
                println!("is Playing: {playing}");
            },
        )
        .unwrap();
        (Tiger { player }, Command::none())
    }

    fn title(&self) -> String {
        String::from("SVG - Iced")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        if let Message::RequestStart = message {
            self.player.toggle_pause_playing();
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        container(button("|>").on_press(Message::RequestStart))
            .width(Length::Fill)
            .center_x()
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        struct FFMpeg;
        subscription::channel(
            std::any::TypeId::of::<FFMpeg>(),
            100,
            |mut output| async move {
                loop {
                    let _ = output.send(FFMpegEvent::Frame).await;
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
            },
        )
        .map(Message::FFMpeg)
    }
}
