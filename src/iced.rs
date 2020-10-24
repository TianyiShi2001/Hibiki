use crate::components::Library;
use iced::widget::image::Handle;
use iced::{Align, Column, Container, Element, Image, Length, Row, Sandbox, Settings, Svg};

pub fn main() -> iced::Result {
    MainView::run(Settings::default())
    //hibiki::main();
}

struct MainView {
    library: Library,
}

impl Sandbox for MainView {
    type Message = ();

    fn new() -> Self {
        MainView {
            library: Library::init(),
        }
    }

    fn title(&self) -> String {
        String::from("Hibiki")
    }

    fn update(&mut self, _message: ()) {}

    fn view(&mut self) -> Element<()> {
        use id3::{Tag, Version};

        let mut tag = Tag::read_from_path(
            "/home/tianyi/Music/tdsm/NieR Automata Original Soundtrack/0008642661.mp3",
        )
        .unwrap();
        let picture = &tag.pictures().next().unwrap().data;

        let image = Image::new(Handle::from_memory(picture.clone()))
            .width(Length::Fill)
            .height(Length::Fill);

        self.library
            .albums
            .iter()
            .fold(
                Row::new()
                    .padding(10)
                    .spacing(20)
                    .align_items(Align::Center),
                |row, item| {
                    row.push(Image::new(match &item.cover {
                        Some(data) => Handle::from_memory(data.clone()),
                        None => Handle::from_memory(
                            include_bytes!("../assets/folder-music.png").to_vec(),
                        ),
                    }))
                    .height(Length::Units(200))
                },
            )
            .into()

        // Container::new(image)
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .padding(20)
        //     .center_x()
        //     .center_y()
        //     .into()
    }
}
