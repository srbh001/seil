use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::crates::custom_themes;

use crate::lexer::{Lexer, Processor, Token};
use crate::welcome::welcome_screen;
use iced::widget::text::Text;
use iced::widget::{button, column, container, row, text, text_editor, Button};
use iced::widget::{horizontal_space, Column};
use iced::window::Icon;
use iced::Settings;
use iced::{executor, Application, Command, Element, Length, Sandbox, Theme};
use iced::{window, Font, Size};

// use crate::highlighter::highlighter::{self, Highlighter};

pub fn tesh_editor() {
    //let vec_u8_icon = include_bytes!("../assets/EDITOR.png").to_vec();
    //let icon = iced::window::icon::from_rgba(vec_u8_icon, 128, 128).unwrap();
    let settings = Settings {
        window: window::Settings {
            min_size: Some(Size {
                width: 800.0,
                height: 600.0,
            }),
            transparent: true,
            decorations: true,
            //icon: Some(icon),
            ..window::Settings::default()
        },
        ..Settings::default()
    };

    Editor::run(settings);
}

struct Editor {
    path: Option<PathBuf>,
    lexer: Lexer, // TODO: Change this to parser
    content: text_editor::Content,
    error: Option<Error>,
    state: State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    OpenFile,
    CloseFile,
    NewFile,
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

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Message>) {
        let lexer = Lexer::new("");
        let lexer_input = lexer.input.iter().collect::<String>();

        println!("Lexer input: {:?}", lexer_input.as_str());

        (
            Self {
                /*path: Some(default_path()),*/
                path: None,
                lexer: lexer,
                content: text_editor::Content::with_text(lexer_input.as_str()),
                error: None,
                state: State::Welcome,
            },
            //Command::perform(load_file(self.path), Message::FileOpened),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("SEIL")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
                //let input = self.content.text().clone();
                Command::none()
            }
            Message::FileOpened(Ok((path, result))) => {
                self.lexer = Lexer::new(result.as_str());
                let lexer_input = self.lexer.input.iter().collect::<String>();
                self.content = text_editor::Content::with_text(lexer_input.as_str());
                self.path = Some(path);
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
                self.state = State::Welcome;
                Command::none()
            }
            Message::NewFile => {
                self.path = None;
                self.content = text_editor::Content::default();
                self.state = State::Editing;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        //let text_to_edit = self.lexer.input.iter().collect::<String>();
        //let text = text_editor::Content::from(text_to_edit);

        let controls = row![button("Open").on_press(Message::OpenFile)];
        let controls = match self.path {
            Some(_) => controls
                .push(button("Close").on_press(Message::CloseFile))
                .spacing(10)
                .padding(10),
            None => controls,
        };

        //let close_file = button("Close").on_press(Message::New);
        let path = match self.path.as_deref().and_then(Path::to_str) {
            Some(path) => text(path).size(14),
            _ => text("No file opened").size(14),
        };
        let position = {
            let (line, column) = self.content.cursor_position();
            text(format!("{}: {}", line + 1, column + 1))
        };

        let status_bar = row![path, horizontal_space(), position];

        let input = text_editor(&self.content)
            .on_action(Message::Edit)
            .height(Length::Fill);
        // .highlight::<Highlighter>(highlighter::Settings {}, |highlight, _theme| {
        //     highlight.format()
        //});

        // let welcome = text("Welcome to SEIL").size(50);
        match self.state {
            State::Editing => container(column![controls, input, status_bar])
                .padding(10)
                .into(),
            State::Welcome => welcome_screen(),
        }
    }
    fn theme(&self) -> Theme {
        let palett = custom_themes::black().to_owned();
        Theme::Dracula
    }
}

async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;
    load_file(handle.path().to_owned()).await
}

async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(Error::IO)?;
    Ok((path, contents))
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
    IO(io::ErrorKind),
}

fn default_path() -> PathBuf {
    PathBuf::from(format!("{}/src/test/test.asm", env!("CARGO_MANIFEST_DIR")))
}
const FONT: Font = Font::DEFAULT;

fn highlight_token(token: Token) -> String {
    match token {
        Token::Opcode(op) => op.to_owned(),
        Token::Label(label) => label.to_owned(),
        Token::Number(num) => num.to_string(),
        Token::Register(reg) => format!("R{}", reg),
        Token::Comment(comment) => comment.to_owned(),
        Token::Comma => ",".to_owned(),
        Token::NewLine => "\n".to_owned(),
        _ => String::new(), // Default for other tokens
    }
}

fn highlight_code(input: &str, processor: Processor) -> Column<Message> {
    let mut lexer = Lexer::new(input);
    let mut column = Column::new().spacing(5);

    while let token = lexer.next_token(processor) {
        if let Token::EOF = token {
            break;
        }
        let highlighted_text = highlight_token(token);
        column = column.push(Text::new(highlighted_text).font(FONT).size(20));
    }

    column
}
