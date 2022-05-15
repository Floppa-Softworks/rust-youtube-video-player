mod cli;

use clap::Parser;
use cli::{Args, Subcommands};
use std::io::{BufReader, Error};

use std::process::{Command, Stdio};
use std::time::Duration;

use rodio::OutputStream;

fn download_video(video_url: String) -> () {
    let ytdl_args = [
        "--extract-audio",
        "--audio-format",
        "mp3",
        "-o",
        "music.mp3",
        &video_url,
    ];

    let mut command = Command::new("yt-dlp.exe")
        .args(ytdl_args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .expect("yt-dlp command failed to start");

    command
        .wait()
        .expect("Couldn't wait for command to finish.");

    std::thread::sleep(Duration::from_secs_f32(0.5))
}

fn play_audio() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = std::fs::File::open("music.mp3").unwrap();

    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}

// command: yt-dlp -j -q --no-simulate -f webm[abr>0]/bestaudio/best -R infinite --no-playlist --ignore-config "https://www.youtube.com/watch?v=P-aFiJ5kjsU" -o -

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    match &args.subcommands {
        Subcommands::Play { url } => {
            let video_url = url.to_owned().unwrap();
            download_video(video_url);
            play_audio();
        }
    }

    //download_video("https://www.youtube.com/watch?v=P-aFiJ5kjsU");
    //play_audio();
    Ok(())
}
