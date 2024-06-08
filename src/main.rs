use iced::advanced::graphics::text::Paragraph;
use iced::{
    executor, Application, widget::Button, widget::Column, Command, widget::Container, Element, Length, Settings, Subscription, widget::Text, widget::TextInput,
};
use iced::widget::{button, text_input};
use iced::alignment::Horizontal;
use tokio::process::Command as TokioCommand;

#[derive(Default)]
struct YouTubeDownloader {
    url: String,
    url_input: text_input::State<Paragraph>,
    download_button: button::State,
    message: String,
}

#[derive(Debug, Clone)]
enum Message {
    UrlChanged(String),
    Download,
    DownloadComplete(Result<String, String>),
}

impl Application for YouTubeDownloader {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (YouTubeDownloader, Command<Message>) {
        (YouTubeDownloader::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("YouTube Downloader")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::UrlChanged(url) => {
                self.url = url;
                Command::none()
            }
            Message::Download => {
                let url = self.url.clone();
                Command::perform(download_video(url), Message::DownloadComplete)
            }
            Message::DownloadComplete(result) => {
                self.message = match result {
                    Ok(msg) => msg,
                    Err(err) => err,
                };
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let url_input = TextInput::new(
            "Enter YouTube URL...",
            "",
            )
        .padding(10);

        let download_button = Button::new("Download")
            .padding(10)
            .on_press(Message::Download); 

        let content = Column::new()
            .align_items(iced::Alignment::Center)
            .spacing(20)
            .push(url_input)
            .push(download_button)
            .push(Text::new(&self.message).horizontal_alignment(Horizontal::Center));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

async fn download_video(url: String) -> Result<String, String> {
    match TokioCommand::new("youtube-dl")
        .arg(url)
        .output()
        .await
    {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from("Download complete!"))
            } else {
                Err(format!(
                    "Failed to download video: {}",
                    String::from_utf8_lossy(&output.stderr)
                ))
            }
        }
        Err(e) => Err(format!("Failed to execute youtube-dl: {}", e)),
    }
}

fn main() -> iced::Result {
    YouTubeDownloader::run(Settings::default())
}
