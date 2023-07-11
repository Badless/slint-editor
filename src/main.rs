use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_request_execute(move || {
        let _ui = ui_handle.unwrap();
        let path = std::env::current_dir().unwrap();
        let res = rfd::FileDialog::new()
            .set_directory(&path)
            .pick_file().unwrap();
        execute_command(
            "python3",
            format!("{}", res.to_owned().as_path().to_string_lossy()),
            "Failed to execute!"
        );
    });

    let ui_handle = ui.as_weak();
    ui.on_request_save(move || {
        let ui = ui_handle.unwrap();
        let path = std::env::current_dir().unwrap();
        let res = rfd::FileDialog::new()
            .set_directory(&path)
            .save_file().unwrap();
        if ui.get_tab_index() == 0 {
            let mut file = File::create(res.to_owned().as_path()).unwrap();
            file.write_all(ui.get_editor_text_tab1().as_bytes().into()).expect("msg");
            if ui.get_tab1_name() == "Tab 1" {
                ui.set_tab1_name(res.to_owned().file_name().unwrap().to_str().unwrap().to_string().into());
            }
        } else if ui.get_tab_index() == 1 {
            let mut file = File::create(res.to_owned().as_path()).unwrap();
            file.write_all(ui.get_editor_text_tab2().as_bytes().into()).expect("msg");
            if ui.get_tab1_name() == "Tab 2" {
                ui.set_tab1_name(res.to_owned().file_name().unwrap().to_str().unwrap().to_string().into());
            }
        } else if ui.get_tab_index() == 2 {
            let mut file = File::create(res.to_owned().as_path()).unwrap();
            file.write_all(ui.get_editor_text_tab3().as_bytes().into()).expect("msg");
            if ui.get_tab1_name() == "Tab 3" {
                ui.set_tab1_name(res.to_owned().file_name().unwrap().to_str().unwrap().to_string().into());
            }
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_request_open(move || {
        let ui = ui_handle.unwrap();
        let path = std::env::current_dir().unwrap();
        let res = rfd::FileDialog::new()
            .set_directory(&path)
            .pick_file().unwrap();
        let mut file = File::open(res.to_owned().as_path()).expect("msg");
        let mut opened_file = String::new();
        file.read_to_string(&mut opened_file).expect("msg");
        if ui.get_tab_index() == 0 {
            ui.set_editor_text_tab1(opened_file.to_string().into());
            ui.set_tab1_name(res.to_owned().file_name().unwrap().to_str().unwrap().to_string().into());
        } else if ui.get_tab_index() == 1 {
            ui.set_editor_text_tab2(opened_file.to_string().into());
            ui.set_tab2_name(res.to_owned().file_name().unwrap().to_str().unwrap().to_string().into());
        } else if ui.get_tab_index() == 2 {
            ui.set_editor_text_tab3(opened_file.to_string().into());
            ui.set_tab3_name(res.to_owned().file_name().unwrap().to_str().unwrap().to_string().into());
        }
    });

    ui.run()
}
pub fn execute_command(command: &str, arg: String, expect_msg: &str) {
    let mut execute = Command::new(command);
    execute.arg(arg);
    execute.status().expect(expect_msg);
}
