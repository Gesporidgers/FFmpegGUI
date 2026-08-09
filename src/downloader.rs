use std::{io, process::Command};

pub fn download(url: &str, dest_file: &str) -> io::Result<std::process::ExitStatus> {
    Command::new("ffmpeg")
        .args([
            "-user_agent",
            "Mozilla/5.0",
            "-allowed_extensions",
            "ALL",
            "-i",
            url,
            "-c",
            "copy",
            dest_file,
        ])
        .spawn()
        .and_then(|mut child| child.wait())
}
