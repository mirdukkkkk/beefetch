mod info;
mod ui;

use std::env;
use std::io::{self, IsTerminal, Write};

fn main() {
    let mut help = false;
    let mut color =
        env::var_os("NO_COLOR").is_none_or(|v| v.is_empty()) && io::stdout().is_terminal();

    for arg in env::args_os().skip(1) {
        match arg.as_encoded_bytes() {
            b"-h" | b"--help" => help = true,
            b"-n" | b"--no-color" => color = false,
            _ => {}
        }
    }

    let out = if help {
        ui::help(color)
    } else {
        ui::render(&info::Info::collect(), color)
    };

    let _ = io::stdout().lock().write_all(out.as_bytes());
}
