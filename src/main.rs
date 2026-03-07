// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::fs::{read_dir};
use std::rc::Rc;
use slint::{ModelRc, SharedString, ToSharedString, VecModel};

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
            let ui = ui_handle.unwrap();
            match get_files(&ui_handle.unwrap()) {
                Ok(files) => ui.set_files(files),
                Err(err) => {
                    let error_dialog = ErrorDialog::new().unwrap();
                    error_dialog.set_error_text(err.to_shared_string());
                    error_dialog.show().unwrap();
                },
            }
        }
    });

    ui.run()?;

    Ok(())
}

// TODO: Handle error outside of this function (return error if not found, result if found and return files to set it outside of function
fn get_files(ui: &AppWindow) -> Result<ModelRc<FileItem>, Box<dyn Error>> {
    let files_in_dir;
    match read_dir(ui.get_directory()) {
        Ok(dir) => {
            files_in_dir = dir;

            let mut files: Vec<FileItem> = vec![];
            for file in files_in_dir.enumerate() {
                // println!("{:?}", file.1?.file_name());
                files.push(FileItem {
                    name: SharedString::from(file.1?.file_name().into_string().unwrap()),
                    size: 0.0,
                    fileType: FileItemType::FILE,
                });
            }

            let files_vec: Rc<VecModel<FileItem>> = Rc::new(VecModel::from(files));
            let files= ModelRc::from(files_vec);

            Ok(files)
        },
        Err(e) => Err(Box::from(e))
    }
}