use iced::theme;
use iced::widget::{checkbox, column, container, row, svg};
use iced::{color, Element, Length, Sandbox, Settings};

static VECS: [bool; 4] = [false; 4];

//static IMAGE: &[u8] = include_bytes!("../resources/tiger.svg");

pub fn main() -> iced::Result {
    Tiger::run(Settings::default())
}

#[derive(Debug, Default)]
struct Tiger {
    apply_color_filter: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ToggleColorFilter(bool),
}

impl Sandbox for Tiger {
    type Message = Message;

    fn new() -> Self {
        Tiger::default()
    }

    fn title(&self) -> String {
        String::from("SVG - Iced")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ToggleColorFilter(apply_color_filter) => {
                self.apply_color_filter = apply_color_filter;
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        //let handle = svg::Handle::from_memory(IMAGE);
        let handle = svg::Handle::from_path("/usr/share/icons/breeze/mimetypes/16/text-plain.svg");
        let svgs = VECS
            .iter()
            .map(|_| {
                svg(handle.clone())
                    .width(200)
                    .height(200)
                    .style(if self.apply_color_filter {
                        theme::Svg::custom_fn(|_theme| svg::Appearance {
                            color: Some(color!(0x0000ff)),
                        })
                    } else {
                        theme::Svg::Default
                    })
                    .into()
            })
            .collect();
        //let svg0 = svg(handle.clone())
        //    .width(400)
        //    .height(400)
        //    .style(if self.apply_color_filter {
        //        theme::Svg::custom_fn(|_theme| svg::Appearance {
        //            color: Some(color!(0x0000ff)),
        //        })
        //    } else {
        //        theme::Svg::Default
        //    });

        //let svg2 = svg(handle)
        //    .width(400)
        //    .height(400)
        //    .style(if self.apply_color_filter {
        //        theme::Svg::custom_fn(|_theme| svg::Appearance {
        //            color: Some(color!(0x0000ff)),
        //        })
        //    } else {
        //        theme::Svg::Default
        //    });
        let top = container(row(svgs)).width(Length::Fill).center_x();
        let apply_color_filter = checkbox(
            "Apply a color filter",
            self.apply_color_filter,
            Message::ToggleColorFilter,
        );

        container(
            column![
                top,
                //svg0,
                //svg2,
                container(apply_color_filter).width(Length::Fill).center_x()
            ]
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .center_x()
        .center_y()
        .into()
    }
}
