use std::fs::File;
use std::io::prelude::Write;
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

    let ui_handle = ui.as_weak();
    ui.on_request_save(move || {
        let ui = ui_handle.unwrap();
        if !ui.get_file_name().is_empty() {
            let mut file = File::create(format!("{}.py", ui.get_file_name())).expect("msg");
            file.write_all(ui.get_editor_text().as_bytes().into())
                .expect("msg");
        } else {
            ui.invoke_show_popup();
        }
    });

    ui.run()
}
pub fn execute_command(command: &str, arg: String, expect_msg: &str) {
    let mut execute = Command::new(command);
    execute.arg(arg);
    execute.status().expect(expect_msg);
}
