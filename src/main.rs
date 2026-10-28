mod cli;
mod info;
mod ui;

use std::env;
use std::io::{self, IsTerminal, Write};

fn main() {
    let args = cli::args().run();

    let color = !args.no_color
        && env::var_os("NO_COLOR").is_none_or(|v| v.is_empty())
        && io::stdout().is_terminal();

    let out = ui::render(&info::Info::collect(), color);
    let _ = io::stdout().lock().write_all(out.as_bytes());
}
