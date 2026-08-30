use std::{io, process::{Command, ExitStatus}};

use slint::SharedString;

use crate::ConvertOptions;

#[derive(Debug, serde::Deserialize)]
struct Stream {
    info: String,
    codec: String,
    codec_type: String
}

pub enum MediaType {
    Video,
    Audio
}

pub fn analyze_input(input_file: &str) -> MediaType {
    let mut arguments:Vec<_> = "-v quiet -print_format csv -show_entries stream=codec_type,codec_name".split(" ").collect();
    arguments.append(&mut vec!["-i",input_file]);

    let command = Command::new("ffprobe")
        .args(arguments.clone())
        .output()
        .expect("failed to start ffprobe");

    let output = String::from_utf8(command.stdout).expect("not valid bytes");
    
    let mut rdr = csv::ReaderBuilder::new()
                                    .has_headers(false)
                                    .from_reader(output.as_bytes());
    let mut streams: Vec<Stream> = Vec::new();
    for item in rdr.deserialize(){
        let i: Stream = item.expect("Error when deserializing");
        streams.push(i);
    }
    if streams[0].codec_type == "audio" {
        MediaType::Audio
    }
    else {
        MediaType::Video
    }
    
}

pub fn convert(input_file: &str, opt: ConvertOptions, dest_file: &str) -> io::Result<ExitStatus> {
    let video_bitrate_str = format!("{}{}",opt.bitrate_video as i32,opt.bitrate_video_m);
    let audio_bitrate_str = format!("{}{}",opt.bitrate_audio as i32,opt.bitrate_audio_m);
    match opt.ignored_stream.as_str() {
        "Нет" => {
            Command::new("ffmpeg")
                .args([
                    "-i", input_file,
                    "-c:v", if opt.video_codec.is_empty() { "copy" } else {&opt.video_codec},
                    "-c:a", if opt.audio_codec.is_empty() { "copy" } else {&opt.audio_codec},
                    if opt.bitrate_video != 0. {&video_bitrate_str} else {""},
                    if opt.bitrate_audio != 0. {&audio_bitrate_str} else {""},
                    &opt.additional,
                    dest_file
                ]).spawn().and_then(|mut child| child.wait())
        }
        "Видео" => {
            Command::new("ffmpeg")
                .args([
                    "-i", input_file,
                    "-vn",
                    "-c:a", if opt.audio_codec.is_empty() { "copy" } else {&opt.audio_codec},
                    if opt.bitrate_audio != 0. {&audio_bitrate_str} else {""},
                    &opt.additional,
                    dest_file
                ]).spawn().and_then(|mut child| child.wait())
        }
        "Аудио" => {
            Command::new("ffmpeg")
                .args([
                    "-i", input_file,
                    "-an",
                    "-c:v", if opt.video_codec.is_empty() { "copy" } else {&opt.video_codec},
                    if opt.bitrate_video != 0. {&video_bitrate_str} else {""},
                    &opt.additional,
                    dest_file
                ]).spawn().and_then(|mut child| child.wait())
        }
        _ => panic!("Error in mode of ignoring")
    }
}