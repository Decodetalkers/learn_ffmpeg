use iced::futures::SinkExt;
use iced::widget::{button, container};
use iced::{
    executor, subscription, Application, Command, Element, Length, Settings, Subscription, Theme,
};

use std::sync::Arc;

use tokio::sync::{mpsc::Receiver, Mutex};
mod player;

#[derive(Debug, Default)]
struct InitFlag {
    url: String,
}

fn main() -> iced::Result {
    FFmpegSimple::run(Settings {
        flags: InitFlag {
            url:
                "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/TearsOfSteel.mp4"
                    .to_string(),
        },
        ..Default::default()
    })
}

#[derive(Debug)]
struct FFmpegSimple {
    player: player::Player,
    rv: Arc<Mutex<Receiver<FFMpegMessages>>>,
    play_status: bool,
}

#[derive(Debug, Clone)]
enum Message {
    RequestStart,
    FFMpeg(FFMpegMessages),
}

#[derive(Debug, Clone)]
enum FFMpegMessages {
    Data(Vec<u8>),
    StatusChanged(bool),
}

impl Application for FFmpegSimple {
    type Message = Message;
    type Flags = InitFlag;
    type Executor = executor::Default;
    type Theme = Theme;
    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let (sd, rv) = tokio::sync::mpsc::channel::<FFMpegMessages>(100);
        let sd2 = sd.clone();
        let url = flags.url;
        let player = player::Player::start(
            url.into(),
            move |newframe| {},
            move |playing| {
                sd2.try_send(FFMpegMessages::StatusChanged(playing)).ok();
            },
        )
        .unwrap();
        (
            FFmpegSimple {
                player,
                rv: Arc::new(Mutex::new(rv)),
                play_status: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("FFMpeg - Iced")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::RequestStart => {
                self.player.toggle_pause_playing();
            }
            Message::FFMpeg(FFMpegMessages::StatusChanged(status)) => {
                self.play_status = status;
            }
            _ => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let icon = if self.play_status { "o" } else { "|>" };
        container(button(icon).on_press(Message::RequestStart))
            .width(Length::Fill)
            .center_x()
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let rv = self.rv.clone();
        struct FFMpeg;
        subscription::channel(
            std::any::TypeId::of::<FFMpeg>(),
            100,
            |mut output| async move {
                let mut rv = rv.lock().await;
                loop {
                    let Some(message) = rv.recv().await else {
                        continue;
                    };
                    let _ = output.send(message).await;
                }
            },
        )
        .map(Message::FFMpeg)
    }
}
