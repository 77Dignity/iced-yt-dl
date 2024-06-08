use iced::{
    widget::button, executor, alignment::Alignment, Application, widget::Button, widget::Column, Command, Element,
    widget::ProgressBar, Settings, Subscription, widget::Text, widget::TextInput, widget::text_input,
};
use std::sync::{Arc, Mutex};
use tokio::task;
use youtube_dl::YoutubeDl;
use nfd2::Response;

#[derive(Default)]
struct YouTubeDLGui {
    url: String,
    directory: String,
    progress: f32,
    url_input: text_input::State,
    dir_button: button::State,
    download_button: button::State,
    state: Arc<Mutex<AppState>>,
}

#[derive(Default)]
struct AppState {
    url: String,
    directory: String,
    progress: f32,
}

#[derive(Debug, Clone)]
enum Message {
    UrlChanged(String),
    DirectoryChosen(Option<String>),
    Download,
    ProgressUpdated(f32),
}

impl Application for YouTubeDLGui {
    type Theme = iced::theme::Theme;
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            YouTubeDLGui {
                state: Arc::new(Mutex::new(AppState::default())),
                ..Self::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("YouTube DL GUI")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::UrlChanged(url) => {
                self.url = url;
                Command::none()
            }
            Message::DirectoryChosen(directory) => {
                if let Some(dir) = directory {
                    self.directory = dir;
                }
                Command::none()
            }
            Message::Download => {
                let state = self.state.clone();
                let url = self.url.clone();
                let directory = self.directory.clone();

                task::spawn(async move {
                    if !url.is_empty() && !directory.is_empty() {
                        let output_template = format!("{}/%(title)s.%(ext)s", directory);
                        let _ = YoutubeDl::new(&url)
                            .output_template(&output_template)
                            .download_to(directory)
                            .unwrap();

                        for i in 0..=100 {
                            state.lock().unwrap().progress = i as f32 / 100.0;
                            futures::future::pending::<()>().await;
                        }
                    }
                });

                Command::none()
            }
            Message::ProgressUpdated(progress) => {
                self.progress = progress;
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(100)).map(|_| {
            let progress = self.state.lock().unwrap().progress;
            Message::ProgressUpdated(progress)
        })
    }

    fn view(&mut self) -> Element<Message> {
        let url_input = TextInput::new(
            &mut self.url_input,
            "Enter YouTube URL",
            &self.url,
            Message::UrlChanged,
        )
        .padding(10)
        .size(20);

        let dir_button = Button::new(&mut self.dir_button, Text::new("Choose Download Directory"))
            .padding(10)
            .on_press(Message::DirectoryChosen(choose_directory()));

        let download_button = Button::new(&mut self.download_button, Text::new("Download"))
            .padding(10)
            .on_press(Message::Download);

        let progress_bar = ProgressBar::new(0.0..=1.0, self.progress);

        let content = Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(url_input)
            .push(dir_button)
            .push(progress_bar)
            .push(download_button);

        content.into()
    }
}

fn choose_directory() -> Option<String> {
    let result = nfd2::open_pick_folder(None).unwrap();
    match result {
        Response::Okay(path) => Some(path),
        _ => None,
    }
}

fn main() {
    YouTubeDLGui::run(Settings::default());
}
