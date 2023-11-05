use ffmpeg_next::format::Pixel;
use iced::futures::SinkExt;
use iced::widget::image::Handle;
use iced::widget::{button, column, container, Image};
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
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
enum Message {
    RequestStart,
    FFMpeg(FFMpegMessages),
}

#[derive(derive_more::Deref, derive_more::DerefMut)]
struct Rescaler(ffmpeg_next::software::scaling::Context);
unsafe impl std::marker::Send for Rescaler {}

fn rgba_rescaler_for_frame(frame: &ffmpeg_next::util::frame::Video) -> Rescaler {
    Rescaler(
        ffmpeg_next::software::scaling::Context::get(
            frame.format(),
            frame.width(),
            frame.height(),
            Pixel::RGB24,
            frame.width(),
            frame.height(),
            ffmpeg_next::software::scaling::Flags::BILINEAR,
        )
        .unwrap(),
    )
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
        let mut to_rgba_rescaler: Option<Rescaler> = None;
        let player = player::Player::start(
            url.into(),
            move |new_frame| {
                let rebuild_rescaler =
                    to_rgba_rescaler.as_ref().map_or(true, |existing_rescaler| {
                        existing_rescaler.input().format != new_frame.format()
                    });

                if rebuild_rescaler {
                    to_rgba_rescaler = Some(rgba_rescaler_for_frame(new_frame));
                }
                let rescaler = to_rgba_rescaler.as_mut().unwrap();

                let mut rgb_frame = ffmpeg_next::util::frame::Video::empty();
                rescaler.run(new_frame, &mut rgb_frame).unwrap();
                sd.try_send(FFMpegMessages::Data(rgb_frame.data(0).to_vec()))
                    .ok();
            },
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
                data: Vec::new(),
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
            Message::FFMpeg(FFMpegMessages::Data(data)) => {
                self.data = data;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let icon = if self.play_status { "o" } else { "|>" };
        let imagehd = Handle::from_memory(self.data.clone());
        let image: Element<Message> = Image::new(imagehd).width(Length::Fill).into();
        container(column![
            image,
            container(button(icon).on_press(Message::RequestStart))
                .width(Length::Fill)
                .center_x()
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
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
