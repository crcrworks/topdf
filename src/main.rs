mod elements;
mod file;
mod input;
mod render;
mod style;
mod terminal;
mod unit;

use elements::{
    input_name::Input,
    select_size::{SizeOption, SizeOptionList},
};
use file::FileList;
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event},
    terminal::Terminal,
    widgets::ListState,
};
use std::{error::Error, io, path::PathBuf};
use unit::MM;

pub const DPI: f32 = 350.0;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::default();

    let current_dir = std::env::current_dir()?;
    app.file_list.load_files(&current_dir)?;

    let terminal = terminal::init_terminal()?;
    let app_result = app.run(terminal);
    terminal::restore_terminal()?;
    if let Ok(message) = app_result {
        if !message.is_empty() {
            println!("{}", message);
        }
    }
    Ok(())
}

pub struct App {
    pub scene: Scene,
    pub should_exit: bool,
    pub file_list: FileList,
    pub size_option_list: SizeOptionList,
    pub input: Input,
    pub exit_message: String,
}

impl App {
    fn run(&mut self, mut terminal: Terminal<impl Backend>) -> io::Result<String> {
        while !self.should_exit {
            terminal.draw(|f| {
                render::ui(f, self);
            })?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            }
        }
        Ok(self.exit_message.clone())
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Scene {
    File,
    PageFormat,
    Name,
}

impl FromIterator<SizeOption> for SizeOptionList {
    fn from_iter<I: IntoIterator<Item = SizeOption>>(iter: I) -> Self {
        let options = iter.into_iter().map(SizeOption::new).collect();
        let state = ListState::default();
        Self { options, state }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            scene: Scene::File,
            exit_message: String::from(""),
            should_exit: false,
            input: Input::default(),
            file_list: FileList {
                root_dir: PathBuf::new(),
                items: vec![],
                state: ListState::default(),
            },
            size_option_list: SizeOptionList::from_iter([
                SizeOption {
                    name: String::from("A0"),
                    width: MM::new(841., DPI),
                    height: MM::new(1189., DPI),
                },
                SizeOption {
                    name: String::from("A1"),
                    width: MM::new(594., DPI),
                    height: MM::new(841., DPI),
                },
                SizeOption {
                    name: String::from("A2"),
                    width: MM::new(420., DPI),
                    height: MM::new(594., DPI),
                },
                SizeOption {
                    name: String::from("A3"),
                    width: MM::new(297., DPI),
                    height: MM::new(420., DPI),
                },
                SizeOption {
                    name: String::from("A4"),
                    width: MM::new(210., DPI),
                    height: MM::new(297., DPI),
                },
                SizeOption {
                    name: String::from("A5"),
                    width: MM::new(148., DPI),
                    height: MM::new(210., DPI),
                },
                SizeOption {
                    name: String::from("A6"),
                    width: MM::new(105., DPI),
                    height: MM::new(148., DPI),
                },
                SizeOption {
                    name: String::from("A7"),
                    width: MM::new(74., DPI),
                    height: MM::new(105., DPI),
                },
                SizeOption {
                    name: String::from("B0"),
                    width: MM::new(1030., DPI),
                    height: MM::new(1456., DPI),
                },
                SizeOption {
                    name: String::from("B1"),
                    width: MM::new(728., DPI),
                    height: MM::new(1030., DPI),
                },
                SizeOption {
                    name: String::from("B2"),
                    width: MM::new(515., DPI),
                    height: MM::new(728., DPI),
                },
                SizeOption {
                    name: String::from("B3"),
                    width: MM::new(364., DPI),
                    height: MM::new(515., DPI),
                },
                SizeOption {
                    name: String::from("B4"),
                    width: MM::new(257., DPI),
                    height: MM::new(364., DPI),
                },
                SizeOption {
                    name: String::from("B5"),
                    width: MM::new(182., DPI),
                    height: MM::new(257., DPI),
                },
                SizeOption {
                    name: String::from("B6"),
                    width: MM::new(128., DPI),
                    height: MM::new(182., DPI),
                },
                SizeOption {
                    name: String::from("B7"),
                    width: MM::new(91., DPI),
                    height: MM::new(128., DPI),
                },
            ]),
        }
    }
}
