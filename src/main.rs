mod cli;
mod config;
mod info;
mod ui;

use std::env;
use std::io::{self, IsTerminal, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = cli::args().run();

    let config = match config::load(args.config.as_deref()) {
        Ok((config, warnings)) => {
            for w in warnings {
                eprintln!("beefetch: {w}");
            }
            config
        }
        Err(e) => {
            eprintln!("beefetch: {e}");
            return ExitCode::FAILURE;
        }
    };

    let color = config.color
        && !args.no_color
        && env::var_os("NO_COLOR").is_none_or(|v| v.is_empty())
        && io::stdout().is_terminal();

    let fields: Vec<_> = args
        .fields
        .unwrap_or(config.fields)
        .into_iter()
        .map(|f| (f.label(), f.collect()))
        .collect();

    let out = ui::render(&info::user(), &info::host(), &fields, color);
    let _ = io::stdout().lock().write_all(out.as_bytes());
    ExitCode::SUCCESS
}
