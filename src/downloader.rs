use std::{io, process::{Command, ExitStatus}};

pub fn download(url:&str, dest_file: &str) -> ExitStatus{
    Command::new("ffmpeg")
        .args([
            "-user_agent", "Mozilla/5.0",
            "-allowed_extensions", "ALL",
            "-i" , url,
            "-c" , "copy",
            dest_file
        ])
        .status().expect("Failed to launch FFmpeg")
}