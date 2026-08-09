use std::path::{Path, PathBuf};
use std::fmt::Error;
use iced::{
    Element, wgpu::naga::compact::KeepUnused::No, widget::{
        TextEditor, TextInput, column, text, text_editor, text_input
    },
    Fill,
    window, Size
};

fn main() -> iced::Result {
    println!("Hello, world!");
    // iced::run(Note::update, Note::view)
    let settings = window::Settings{
        size: Size::new(600.0, 600.0),
        decorations: false,
        ..Default::default()
    };
    iced::application(Note::new, Note::update, Note::view).window(settings).run()
}


#[derive(Clone)]
enum Message {
    NewSession,
    SaveFile,
    FileSaved(Result<PathBuf, Error>),
    Edit(text_editor::Action),
}

#[derive(Default)]
struct Note {
    title: String,
    content: text_editor::Content
}

impl Note {
    fn new() -> Self {
        Self{
            title: "".to_string(),
            content: text_editor::Content::new(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => self.content.perform(action),
            Message::FileSaved(result) => println!("Saved! {:?}", result),
            Message::NewSession => {},
            Message::SaveFile => {},
        }
    }

    fn view(&self) -> Element<Message> {
        // "Hellooo".into()
        text_editor(&self.content)
            .id("editor")
            .height(Fill)
            .wrapping(text::Wrapping::Word)
            .on_action(Message::Edit)
            .into()
    }
}