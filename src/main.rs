use std::fs::File;
//use std::io::prelude::Write;
use std::io::Read;
use std::process::Command;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_request_execute(move || {
        let ui = ui_handle.unwrap();
        if !ui.get_file_name().is_empty() {
            execute_command(
                "python3",
                format!("{}.py", ui.get_file_name()),
                "Failed to execute!",
            );
        } else {
            ui.invoke_show_popup();
        }
    });

    /*let ui_handle = ui.as_weak();
    ui.on_request_save(move || {
        let ui = ui_handle.unwrap();
        if !ui.get_file_name().is_empty() {
            let mut file = File::create(format!("{}.py", ui.get_file_name())).expect("msg");
            file.write_all(ui.get_editor_text().as_bytes().into())
                .expect("msg");
        } else {
            ui.invoke_show_popup();                 TODO: RECODE IT USING RFD!
        }
    });*/

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
        if ui.get_tab1_name() == "Tab 1" {
            ui.set_editor_text_tab1(opened_file.to_string().into());
            ui.set_tab1_name(res.to_owned().file_name().unwrap().to_str().unwrap().to_string().into());
        } else if ui.get_tab2_name() == "Tab 2" {
            ui.set_editor_text_tab2(opened_file.to_string().into());
            ui.set_tab2_name(res.to_owned().file_name().unwrap().to_str().unwrap().to_string().into());
        } else if ui.get_tab3_name() == "Tab 3" {
            ui.set_editor_text_tab3(opened_file.to_string().into());
            ui.set_tab3_name(res.to_owned().file_name().unwrap().to_str().unwrap().to_string().into());
        }
    });
    // TODO: MAKE TABS WORK BETTER!

    ui.run()
}
pub fn execute_command(command: &str, arg: String, expect_msg: &str) {
    let mut execute = Command::new(command);
    execute.arg(arg);
    execute.status().expect(expect_msg);
}
