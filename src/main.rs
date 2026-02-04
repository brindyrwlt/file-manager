// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::fs::read_dir;
use std::rc::Rc;
use slint::{ModelRc, SharedString, VecModel};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    get_files(&ui);

    ui.on_change_current_directory({
        let ui_handle = ui.as_weak();
        move || {
            get_files(&ui_handle.unwrap());
        }
    });

    ui.run()?;

    Ok(())
}

fn get_files(ui: &AppWindow) -> Result<(), Box<dyn Error>> {
    let files_in_dir = read_dir(ui.get_directory()).expect("Can't read into directory");
    // println!("{:?}", files.enumerate());

    let mut files: Vec<FileItem> = vec![];
    for file in files_in_dir.enumerate() {
        // println!("{:?}", file.1?.file_name());
        files.push(FileItem {
            name: SharedString::from(file.1?.file_name().into_string().unwrap()),
            size: 0.0,
            fileType: FileItemType::FILE,
        });
    //     SharedString::from(file.1?.file_name().into_string().unwrap()), 0, FileItemType::FILE
    }

    let files_vec: Rc<VecModel<FileItem>> = Rc::new(VecModel::from(files));
    let files= ModelRc::from(files_vec);

    ui.set_files(files);
    Ok(())
}