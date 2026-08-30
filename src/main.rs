// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, thread};

use slint::SharedString;



mod downloader;
mod converter;

slint::include_modules!();


fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let ui_clone = ui.clone_strong();

    ui.on_download(move |url: SharedString, mode|{
        let ui_handle = ui_clone.as_weak();
        
            let ui = ui_handle.unwrap();
            let type_tuple = match mode {
                0 => ("Видеофайлы", &["mp4"]),
                1 => ("Аудиофайлы", &["m4a"]),
                _ => panic!("error in  index"),
            };
            let file_path = rfd::FileDialog::new()
                .set_title("Сохранить файл")
                .add_filter(type_tuple.0, type_tuple.1)
                .save_file();

            if let Some(path) = file_path {
                ui.set_is_working(true);
                let ui_handle = ui_handle.clone();
                let url = url.clone();
                let path = path.to_string_lossy().into_owned();
                thread::spawn(move || {
                    let result = downloader::download(&url, &path);
                    
                    let _ = slint::invoke_from_event_loop(move ||{
                        let ui = ui_handle.unwrap();
                        ui.set_is_working(false);
                        match result {
                            Ok(status) if status.success() => ui.set_is_success(true),
                            Ok(status) => {
                                ui.set_is_error(true);
                                ui.set_error_details(format!("FFmpeg exited with {}", status).into());
                            }
                            Err(err) => {
                                ui.set_is_error(true);
                                ui.set_error_details(err.to_string().into());
                            }
                        }
                    });
                });
            }
    });
    let ui_clone = ui.clone_strong();
    ui.on_pick_file(move ||{
        let ui_handle = ui_clone.as_weak();
        let ui = ui_handle.unwrap();
        let file_path = rfd::FileDialog::new()
            .set_title("Выберите файл")
            .pick_file();
        if let Some(path) = file_path {
            ui.set_file_path(path.clone().to_str().unwrap_or_default().into());
            match converter::analyze_input(path.to_str().unwrap_or_default()) {
                converter::MediaType::Video => {
                    ui.invoke_picked_video();
                }
                converter::MediaType::Audio => {
                    ui.invoke_picked_audio();
                }
            }
        }
    });
    let ui_clone = ui.clone_strong();
    ui.on_convert(move |opt,input_file| {
        let ui_handle = ui_clone.as_weak();
        let ui = ui_handle.unwrap();
        let file_path = rfd::FileDialog::new()
            .set_title("Сохранить как")
            .save_file();
        if let Some(path) = file_path {
            // ui.set_is_working2(true);
            // let ui_handle = ui_handle.clone();
            // let dest_file = path.to_string_lossy().into_owned();
            // thread::spawn(move || {
            //     let result = converter::convert(&input_file, opt, &dest_file);

            //     let _ = slint::invoke_from_event_loop(move || {
            //         let ui = ui_handle.unwrap();
            //         match result {
            //             Ok(status) if status.success() => ui.set_is_success2(true),
            //             Ok(status) => {
            //                 ui.set_is_error2(true);
            //                 ui.set_error_details2(format!("FFmpeg exited with {}", status).into());
            //             }
            //             Err(err) => {
            //                 ui.set_is_error2(true);
            //                 ui.set_error_details2(err.to_string().into());
            //             }
            //         }
            //         ui.set_is_working2(false);
            //     });
            // });

        }
    });

    ui.run()?;
    Ok(())
}


