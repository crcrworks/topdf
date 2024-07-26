use crate::{App, Scene};

use crate::file::Status;
use crossterm::event::KeyEvent;
use ratatui::crossterm::event::{KeyCode, KeyEventKind};

impl App {
    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.back(),
            KeyCode::Enter => self.confirm(),
            _ => {}
        }

        match self.scene {
            Scene::File => match key.code {
                KeyCode::Char('j') | KeyCode::Down => self.select_next(),
                KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
                KeyCode::Char('g') | KeyCode::Home => self.select_first(),
                KeyCode::Char('G') | KeyCode::End => self.select_last(),
                KeyCode::Char('l') | KeyCode::Right | KeyCode::Char(' ') => self.toggle_status(),
                _ => {}
            },
            Scene::PageFormat => match key.code {
                KeyCode::Char('j') | KeyCode::Down => self.select_next(),
                KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
                KeyCode::Char('g') | KeyCode::Home => self.select_first(),
                KeyCode::Char('G') | KeyCode::End => self.select_last(),
                _ => {}
            },
            Scene::Name => match key.code {
                KeyCode::Char(c) => self.input.enter_char(c),
                KeyCode::Backspace => self.input.delete_char(),
                KeyCode::Left => self.input.move_cursor_left(),
                KeyCode::Right => self.input.move_cursor_right(),
                _ => {}
            },
        }
    }

    fn back(&mut self) {
        match self.scene {
            Scene::File => self.should_exit = true,
            Scene::PageFormat => self.scene = Scene::File,
            Scene::Name => self.scene = Scene::PageFormat,
        }
    }

    fn select_next(&mut self) {
        match self.scene {
            Scene::File => self.file_list.state.select_next(),
            Scene::PageFormat => self.size_option_list.state.select_next(),
            _ => {}
        }
    }
    fn select_previous(&mut self) {
        match self.scene {
            Scene::File => self.file_list.state.select_previous(),
            Scene::PageFormat => self.size_option_list.state.select_previous(),
            _ => {}
        }
    }
    fn select_first(&mut self) {
        self.file_list.state.select_first();
    }

    fn select_last(&mut self) {
        self.file_list.state.select_last();
    }

    fn toggle_status(&mut self) {
        if let Some(i) = self.file_list.state.selected() {
            self.file_list.items[i].status = match self.file_list.items[i].status {
                Status::Unchecked => Status::Checked,
                Status::Checked => Status::Unchecked,
            }
        }
    }

    fn confirm(&mut self) {
        match self.scene {
            Scene::File => self.scene = Scene::PageFormat,
            Scene::PageFormat => self.scene = Scene::Name,
            Scene::Name => {
                if self.input.value.is_empty() {
                    return;
                }
                let index: usize = self.size_option_list.state.selected().unwrap_or_default();
                let selected_option = &self.size_option_list.options[index];
                let result = self.file_list.convert_to_pdf(
                    selected_option.width,
                    selected_option.height,
                    &self.input.value,
                );
                match result {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Failed to convert PDF - {}", e);
                    }
                }
                self.should_exit = true;
                self.exit_message = format!(
                    "PDF Saved! path: {}.pdf",
                    self.file_list
                        .root_dir
                        .join(&self.input.value)
                        .to_string_lossy()
                );
            }
        }
    }
}
