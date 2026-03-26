/* file_manager.rs
 *
 * Copyright 2026 Michail Krasnov <mskrasnov07@ya.ru>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

//! File manager widget for `iced`

use anyhow::Result;
use iced::{
    Element, Task,
    widget::{button, column, container, row, space, text},
};
use std::path::PathBuf;
use tokio::fs::DirEntry;

#[derive(Debug, Clone)]
pub struct FileManager {
    current_dir: PathBuf,
    selected_entry: Option<PathBuf>,
    entries: Vec<Entry>,
    loading: bool,
    visible: bool,
    error_text: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub is_hidden: bool,
}

#[derive(Debug, Clone)]
pub enum FileManagerMessage {
    EntrySelected(PathBuf),
    NavigateInto(PathBuf),
    NavigateUp,
    PathEntered(String),
    SelectButtonPressed,
    CancelButtonPressed,
    DirectoryLoaded(OperationResult<Vec<Entry>>),
    CloseErrorBottomBarButtonPressed,
}

#[derive(Debug, Clone)]
pub enum OperationResult<T> {
    Ok(T),
    Err(String),
}

impl FileManager {
    pub async fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf> + Clone,
    {
        let mut fm = Self {
            current_dir: path.clone().into(),
            selected_entry: None,
            entries: Vec::new(),
            loading: true,
            visible: true,
            error_text: None,
        };
        let _ = fm.load_directory(path).await;
        fm
    }

    pub async fn load_directory<P>(&mut self, dir: P) -> Result<()>
    where
        P: Into<PathBuf>,
    {
        let entries = load_directory(dir.into()).await?;
        self.entries = entries;
        Ok(())
    }

    pub fn update(&mut self, message: FileManagerMessage) -> Task<FileManagerMessage> {
        match message {
            FileManagerMessage::EntrySelected(path) => {
                self.selected_entry = Some(path);
                Task::none()
            }
            FileManagerMessage::NavigateInto(path) => {
                if path.is_dir() {
                    self.current_dir = path;
                    self.selected_entry = None;
                    self.loading = true;
                    let current_dir = self.current_dir.clone();
                    Task::perform(
                        async move {
                            let a = load_directory(current_dir).await;
                            OperationResult::from(a)
                        },
                        FileManagerMessage::DirectoryLoaded,
                    )
                } else {
                    Task::none()
                }
            }
            FileManagerMessage::NavigateUp => {
                if let Some(parent) = self.current_dir.parent() {
                    self.current_dir = parent.to_path_buf();
                    self.selected_entry = None;
                    self.loading = true;
                    let current_dir = self.current_dir.clone();
                    Task::perform(
                        async move {
                            let a = load_directory(current_dir).await;
                            OperationResult::from(a)
                        },
                        FileManagerMessage::DirectoryLoaded,
                    )
                } else {
                    Task::none()
                }
            }
            FileManagerMessage::PathEntered(input) => {
                let path = PathBuf::from(&input);
                if path.exists() && path.is_dir() {
                    self.current_dir = path;
                    self.selected_entry = None;
                    self.loading = true;
                    let current_dir = self.current_dir.clone();
                    Task::perform(
                        async move {
                            let a = load_directory(current_dir).await;
                            OperationResult::from(a)
                        },
                        FileManagerMessage::DirectoryLoaded,
                    )
                } else {
                    Task::none()
                }
            }
            FileManagerMessage::SelectButtonPressed | FileManagerMessage::CancelButtonPressed => {
                self.visible = false;
                Task::none()
            }
            FileManagerMessage::DirectoryLoaded(OperationResult::Ok(entries)) => {
                self.entries = entries;
                self.loading = false;
                Task::none()
            }
            FileManagerMessage::DirectoryLoaded(OperationResult::Err(e)) => {
                self.loading = false;
                self.error_text = Some(e);
                Task::none()
            }
            FileManagerMessage::CloseErrorBottomBarButtonPressed => {
                self.error_text = None;
                Task::none()
            }
        }
    }

    pub fn view<'a>(&'a self) -> Element<'a, FileManagerMessage> {
        // let up_button = button("up").on_press(FileManagerMessage::NavigateUp);
        // let path_input = text_input("Path...", self.current_dir.to_str().unwrap_or("/"))
        //     // .on_input(FileManagerMessage::PathEntered)
        //     .on_submit(FileManagerMessage::PathEntered);
        let cancel_button = button("Cancel")
            .on_press(FileManagerMessage::CancelButtonPressed)
            .style(button::secondary);
        let select_button = button("Select")
            .on_press(FileManagerMessage::SelectButtonPressed)
            .style(button::primary);

        let files_list_element = text("not implemented yet");

        container(column![
            // row![up_button, path_input].spacing(5),
            files_list_element,
            row![space::horizontal(), cancel_button, select_button].spacing(5),
        ])
        .into()
    }
}

async fn load_directory(path: PathBuf) -> Result<Vec<Entry>> {
    let mut entries = Vec::new();
    let mut read_dir = tokio::fs::read_dir(path).await?;

    while let Some(entry) = read_dir.next_entry().await? {
        entries.push(Entry::from_dir_entry(entry).await?);
    }
    entries.shrink_to_fit();

    entries.sort_by(|a, b| {
        if a.is_dir == b.is_dir {
            a.name.cmp(&b.name)
        } else {
            b.is_dir.cmp(&a.is_dir)
        }
    });

    Ok(entries)
}

impl Entry {
    pub async fn from_dir_entry(value: DirEntry) -> Result<Self> {
        let file_name = value.file_name().to_string_lossy().to_string();
        let is_hidden = file_name.starts_with('.');

        Ok(Self {
            path: value.path(),
            name: file_name,
            is_dir: value.file_type().await?.is_dir(),
            is_hidden,
        })
    }
}

impl<T> From<Result<T>> for OperationResult<T> {
    fn from(value: Result<T>) -> Self {
        match value {
            Ok(val) => Self::Ok(val),
            Err(why) => Self::Err(why.to_string()),
        }
    }
}
