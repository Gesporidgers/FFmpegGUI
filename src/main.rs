// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

mod downloader;

slint::include_modules!();


fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_download({
        let ui_handle = ui.as_weak();
        move |url_in| {
            let ui = ui_handle.unwrap();
            let type_tuple = match ui.get_index() {
                0 => ("Видеофайлы", &["mp4"]),
                1 => ("Аудиофайлы", &["m4a"]),
                _ => panic!("error in index")              
            };
            let file_path = rfd::FileDialog::new()
                        .set_title("Сохранить файл")
                        .add_filter(type_tuple.0, type_tuple.1)
                        .save_file();
            match file_path {
                Some(path) => {
                    ui.set_is_working(true);
                    downloader::download(&url_in, &path.to_string_lossy());
                }
                None => {}
            }
        }
    });

    
    ui.run()?;
    Ok(())
}


