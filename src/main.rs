use std::fs::File;
use std::io::prelude::Write;
use std::process::Command;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension("py").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    let s = "print(\"Hello World!\")";
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        ui.set_editor_text(escaped.to_string().into());
        //print!("{}", escaped);
    }

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
