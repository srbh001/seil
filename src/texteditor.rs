use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::crates::custom_themes;
use crate::lexer::Lexer;
use crate::parser::{Parser, ParserError};
use crate::welcome::welcome_screen;

use iced::widget::horizontal_space;
use iced::widget::{button, column, container, row, text, text_editor};
use iced::{executor, Application, Command, Element, Font, Length, Theme};
use iced::{window, Settings, Size};
use iced::widget::text::LineHeight;

pub fn tesh_editor() {
    let settings = Settings {
        window: window::Settings {
            min_size: Some(Size { width: 800.0, height: 600.0 }),
            transparent: true,
            decorations: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    };
    let _ = Editor::run(settings);
}

struct Editor {
    path: Option<PathBuf>,
    lexer: Lexer,
    content: text_editor::Content,
    error: Option<Error>,
    parse_errors: Vec<ParserError>,
    state: State,
    line_count: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    OpenFile,
    CloseFile,
    NewFile,
    Copy,
    PasteRequested,
    Paste(Option<String>),
}

pub enum State {
    Welcome,
    Editing,
}

impl Application for Editor {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = executor::Default;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                path: None,
                lexer: Lexer::new(""),
                content: text_editor::Content::default(),
                error: None,
                parse_errors: Vec::new(),
                state: State::Welcome,
                line_count: 1,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        match &self.path {
            Some(p) => format!(
                "SEIL — {}",
                p.file_name().and_then(|n| n.to_str()).unwrap_or("untitled")
            ),
            None => String::from("SEIL"),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
                self.line_count = self.content.line_count();
                let source = self.content.text();
                let mut parser = Parser::new(&source);
                self.parse_errors = match parser.parse() {
                    Ok(_) => Vec::new(),
                    Err(e) => vec![e],
                };
                Command::none()
            }
            Message::FileOpened(Ok((path, result))) => {
                self.lexer = Lexer::new(result.as_str());
                self.content = text_editor::Content::with_text(result.as_str());
                self.line_count = self.content.line_count();
                self.path = Some(path);
                let mut parser = Parser::new(result.as_str());
                self.parse_errors = match parser.parse() {
                    Ok(_) => Vec::new(),
                    Err(e) => vec![e],
                };
                Command::none()
            }
            Message::FileOpened(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::OpenFile => {
                self.state = State::Editing;
                Command::perform(pick_file(), Message::FileOpened)
            }
            Message::CloseFile => {
                self.path = None;
                self.content = text_editor::Content::default();
                self.parse_errors = Vec::new();
                self.state = State::Welcome;
                Command::none()
            }
            Message::NewFile => {
                self.path = None;
                self.content = text_editor::Content::default();
                self.parse_errors = Vec::new();
                self.line_count = 1;
                self.state = State::Editing;
                Command::none()
            }
            Message::Copy => {
                if let Some(selection) = self.content.selection() {
                    return iced::clipboard::write(selection);
                }
                Command::none()
            }
            Message::PasteRequested => iced::clipboard::read(Message::Paste),
            Message::Paste(Some(text)) => {
                self.content.perform(text_editor::Action::Edit(
                    text_editor::Edit::Paste(Arc::new(text)),
                ));
                self.line_count = self.content.line_count();
                Command::none()
            }
            Message::Paste(None) => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // --- Toolbar ---
        let mut btn_row = row![
            button("  New  ").on_press(Message::NewFile),
            button("  Open ").on_press(Message::OpenFile),
            button(" Paste ").on_press(Message::PasteRequested),
        ]
        .spacing(6);

        if self.path.is_some() {
            btn_row = btn_row
                .push(button("  Copy ").on_press(Message::Copy))
                .push(button(" Close ").on_press(Message::CloseFile));
        }

        let toolbar = container(btn_row).padding([8, 10]).width(Length::Fill);

        // --- Line numbers ---
        let line_nums: String = (1..=self.line_count.max(1))
            .map(|i| format!("{:>4}", i))
            .collect::<Vec<_>>()
            .join("\n");

        let line_number_panel = container(
            text(line_nums)
                .font(Font::MONOSPACE)
                .size(14)
                .line_height(LineHeight::default()),
        )
        .padding([10, 8]);

        // --- Editor ---
        let editor = text_editor(&self.content)
            .on_action(Message::Edit)
            .height(Length::Fill);

        let editor_area = row![line_number_panel, editor].padding([0, 6]).spacing(0);

        // --- Error bar ---
        let error_msg = if let Some(e) = self.parse_errors.first() {
            format!("[{}:{}] {}", e.line_number, e.column_number, e.message)
        } else {
            String::new()
        };
        let error_bar = container(text(error_msg).size(13))
            .padding([3, 10])
            .width(Length::Fill);

        // --- Status bar ---
        let path_label = match self.path.as_deref().and_then(Path::to_str) {
            Some(p) => text(p).size(13),
            None => text("untitled").size(13),
        };
        let (line, col) = self.content.cursor_position();
        let cursor = text(format!("Ln {}, Col {}", line + 1, col + 1)).size(13);
        let status_bar = container(
            row![path_label, horizontal_space(), cursor]
        )
        .padding([4, 10])
        .width(Length::Fill);

        match self.state {
            State::Editing => container(
                column![toolbar, editor_area, error_bar, status_bar]
            )
            .into(),
            State::Welcome => welcome_screen(),
        }
    }

    fn theme(&self) -> Theme {
        let _palette = Theme::custom("CUSTOM".to_string(), custom_themes::black().to_owned());
        Theme::Dracula
    }
}

async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose an assembly file")
        .add_filter("Assembly", &["asm", "s", "txt"])
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;
    load_file(handle.path().to_owned()).await
}

async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|e| e.kind())
        .map_err(Error::IO)?;
    Ok((path, contents))
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
    IO(io::ErrorKind),
}
