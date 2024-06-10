use iced::advanced::graphics::text::Paragraph;
use iced::{
    executor, Application, widget::Button, widget::Column, Command, widget::Container, Element, Length, Settings, Subscription, widget::Text, widget::TextInput,
};
use iced::widget::{button, text_input};
use iced::alignment::Horizontal;
use tokio::process::Command as TokioCommand;
use youtube_dl::YoutubeDl;
use regex::Regex;

use std::path::PathBuf;
use native_dialog::FileDialog;

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

pub fn is_valid_youtube_url(url: &str) -> bool {
    let re = Regex::new(r"^(https?://)?(www\.)?(youtube\.com|youtu\.be|music\.youtube\.com)/.+$").unwrap();
    re.is_match(url)
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
                if is_valid_youtube_url(&self.url) {
                    let url = self.url.clone();
                    Command::perform(download_video(url), Message::DownloadComplete)
                } else {
                    self.message = String::from("Invalid YouTube URL");
                    Command::none()
                }
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
            &self.url)
        .padding(10)
        .on_input(Message::UrlChanged)
        .on_submit(Message::Download);

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

    // Use the FileDialog to let the user pick a directory
    let directory: Option<PathBuf> = FileDialog::new()
        .show_open_single_dir()
        .map_err(|e| e.to_string())?; // Convert the error to a String

    // let directory_str = match directory {
    //     Some(path) => path.to_str()
    //         .ok_or_else(|| "Failed to convert path to string".to_string())?,
    //     None => return Err("No directory chosen".to_string()),
    // };

    // If the user picked a directory, convert it to a String and return Ok
    if let Some(path) = directory {
        path.to_str()
            .ok_or_else(|| "Failed to convert path to string".to_string());
            //.map(|s| s.to_owned())

        let args: Vec<&str> = vec![
            "--extract-audio",
            "--audio-format", "m4a",
            "--audio-quality", "0", // '0' is equivalent to '320kbps',
            "--embed-thumbnail",
            "--add-metadata",
            "--download-archive", "archive.txt",
            "--no-overwrites",
            "--ignore-errors",
            "--write-thumbnail",
        ];

        let new_url = url.clone();

        let ydl = YoutubeDl::new(&new_url)

        .format("bestaudio/best")
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .output_template("%(title)s.%(ext)s")
        .extra_arg("--extract-audio")
        .extra_arg("--audio-format m4a")
        .extra_arg("--audio-quality 0") // '0' is equivalent to '320kbps',
        .extra_arg("--embed-thumbnail")
        .extra_arg("--add-metadata")
        .extra_arg("--download-archive archive.txt")
        .extra_arg("--no-overwrites")
        .extra_arg("--ignore-errors")
        .extra_arg("--write-thumbnail")
        // ])
        // for arg in args {
        //     ydl.extra_arg(arg); // Add this line to resolve the error
        // }
        //.extra_arg(args)
        .download_to(path);// download to target directory
        //.build();


        // Return a success message that includes the URL and the directory path
        //Ok(format!("Validations passed. We should proceed with the YouTube download to the directory: '{}', for the URL: '{}'.", path.display(), url))
        Ok("Validations passed we should youtube download.".to_string())
    } else {
        // If the user did not pick a directory, return an Err
        Err("No directory chosen".to_string())
    }

}



fn main() -> iced::Result {

    YouTubeDownloader::run(Settings::default())
}
