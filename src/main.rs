#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //Hides console on release builds

use std::path::{PathBuf};
use std::fmt::Error;
use iced::{Alignment, Length, Task};
use iced::{
    Element, 
    widget::{
        column, row, text, text_editor, button, mouse_area, container, Space
    },
    Fill,
    window, Size,
};

fn main() -> iced::Result {
    // iced::run(Note::update, Note::view)
    let settings = window::Settings{
        size: Size::new(350.0, 300.0),
        decorations: false,
        ..Default::default()
    };
    iced::application(Note::new, Note::update, Note::view).window(settings).title("Rust Notes").run()
}


#[derive(Clone)]
enum Message {
    RequestDrag,
    TitleBarDragged(Option<window::Id>),
    RequestMinimize,
    MinimizeWindow(Option<window::Id>),
    RequestClose,
    CloseWindow(Option<window::Id>),
    NewSession,
    SaveFile,
    FileSaved(Result<PathBuf, Error>),
    Edit(text_editor::Action),
}

#[derive(Default)]
struct Note {
    title: String,
    content: text_editor::Content,
}

impl Note {

    fn new() -> Self {
        Self{
            title: "Rust Notes".to_string(),
            content: text_editor::Content::new(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::RequestDrag => {
                window::latest().map(Message::TitleBarDragged)
            }

            Message::TitleBarDragged(id) => {
                match id {
                    Some(id) => {
                        return window::drag(id);
                    }
                    None => {
                        println!("Drag failed. (No window id found)");
                    }
                }
                Task::none()
            }

            Message::RequestClose => {
                window::latest().map(Message::CloseWindow)
            }

            Message::CloseWindow(id) => {
                match id {
                    Some(id) => {
                        return window::close(id)
                    }
                    None => {
                        println!("No window");
                    }
                }
                Task::none()
            }

            Message::RequestMinimize => {
                window::latest().map(Message::MinimizeWindow)
                // println!("{:?}", win);
                // Task::none()
            }

            Message::MinimizeWindow(id) => {
                // window::minimize(id.unwrap(), false)
                match id {
                    Some(id) => {
                        return window::minimize(id, true);
                        // println!("{id}");
                    }
                    None => {
                        println!("No window found!");
                    }
                }
                Task::none()
            }

            Message::Edit(action) => self.content.perform(action).into(),

            Message::FileSaved(result) => {
                todo!();
                //Task::none()
            }

            Message::NewSession => {
                todo!();
                //Task::none()
            }

            Message::SaveFile => {
                todo!();
                //Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let title_text = text(&self.title).size(15);

        let window_controls = row![
            button("-").on_press(Message::RequestMinimize),
            button("x").on_press(Message::RequestClose),
        ].spacing(10);

        let title_bar_content = row![
            title_text,
            Space::width(Space::new(), Length::Fill),
            window_controls,
        ]
        .width(Length::Fill)
        .align_y(Alignment::Center);

        let title_bar_custom = mouse_area(
            container(title_bar_content)
                .padding(5)
                .style(container::dark)
        )
        .on_press(Message::RequestDrag);


        let editor = text_editor(&self.content)
            .id("editor")
            .height(Fill)
            .wrapping(text::Wrapping::Word)
            .on_action(Message::Edit);

        
        column![
            title_bar_custom,
            editor,
        ].into()
    }
}