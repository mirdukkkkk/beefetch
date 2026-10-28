use bpaf::Bpaf;

/// Neofetch clone written in Rust with 🐝
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub struct Args {
    /// Disable ANSI colors (also NO_COLOR=1)
    #[bpaf(short, long, long("nocolor"))]
    pub no_color: bool,
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
