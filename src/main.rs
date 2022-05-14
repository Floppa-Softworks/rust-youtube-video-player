use std::io::{Cursor, Error};

use std::process::{Command, Stdio};

use rodio::{OutputStream, Source};

fn get_youtube_video_reader(video_url: &'static str) -> String {
    let ytdl_args = [
        "-j",
        "-q",
        "--no-simulate",
        "-f",
        "webm[abr>0]/bestaudio/best",
        "-R",
        "infinite",
        "--no-playlist",
        "--ignore-config",
        video_url,
        "-o",
        "-",
    ];

    let ytdl = Command::new("yt-dlp")
        .args(ytdl_args)
        .stdin(Stdio::null())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()
        .expect("yt-dlp command failed to start");

    let bytes = String::from_utf8_lossy(&ytdl.stdout).into_owned();
    String::from(bytes.as_str())
}

fn play_audio(bytes: String) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let cursor = Cursor::new(String::from(bytes)); // Adds Read and Seek to the bytes via Cursor
    let source = rodio::Decoder::new(cursor).unwrap(); // Decoder requires it's source to impl both Read and Seek

    stream_handle
        .play_raw(source.convert_samples())
        .expect("bruh what"); // Plays on a different thread

    std::thread::sleep(std::time::Duration::from_secs(5));
}

// command: yt-dlp -j -q --no-simulate -f webm[abr>0]/bestaudio/best -R infinite --no-playlist --ignore-config "https://www.youtube.com/watch?v=P-aFiJ5kjsU" -o -

#[tokio::main]
async fn main() -> Result<(), Error> {
    let bytes = get_youtube_video_reader("https://www.youtube.com/watch?v=P-aFiJ5kjsU");
    play_audio(bytes);
    Ok(())
}
