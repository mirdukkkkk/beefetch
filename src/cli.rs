use std::path::PathBuf;

use bpaf::Bpaf;

use crate::info::{self, Field};

/// Neofetch clone written in Rust with 🐝
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub struct Args {
    /// Disable ANSI colors (also NO_COLOR=1)
    #[bpaf(short, long, long("nocolor"))]
    pub no_color: bool,

    /// Fields to show, in order: os, kernel, uptime, ram, wm, server
    #[bpaf(short, long, argument::<String>("LIST"), parse(|s| info::parse_fields(&s)), optional)]
    pub fields: Option<Vec<Field>>,

    /// Config file [default: $XDG_CONFIG_HOME/beefetch/config]
    #[bpaf(short, long, env("BEEFETCH_CONFIG"), argument("PATH"))]
    pub config: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bpaf::doc::Section;

    fn parse(argv: &[&str]) -> Result<Args, String> {
        args().run_inner(argv).map_err(|e| e.unwrap_stderr())
    }

    #[test]
    fn no_color_aliases() {
        assert!(!parse(&[]).unwrap().no_color);
        for flag in ["-n", "--no-color", "--nocolor"] {
            assert!(parse(&[flag]).unwrap().no_color, "{flag}");
        }
        assert!(parse(&["--bogus"]).is_err());
    }

    #[test]
    fn fields_are_validated() {
        assert_eq!(
            parse(&["-f", "ram,os"]).unwrap().fields,
            Some(vec![Field::Ram, Field::Os])
        );
        assert!(parse(&["--fields", "cpu"]).unwrap_err().contains("`cpu`"));
    }

    #[test]
    fn manpage_is_up_to_date() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/docs/beefetch.1");
        let manpage = args().render_manpage("beefetch", Section::General, None, None, None);

        if std::env::var_os("UPDATE_MANPAGE").is_some() {
            std::fs::write(path, &manpage).unwrap();
        }
        assert_eq!(
            std::fs::read_to_string(path).unwrap_or_default(),
            manpage,
            "docs/beefetch.1 is stale, run `UPDATE_MANPAGE=1 cargo test`"
        );
    }
}
