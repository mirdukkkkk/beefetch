use std::path::{Path, PathBuf};
use std::{env, fs, io};

use crate::info::{self, Field};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub fields: Vec<Field>,
    pub color: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fields: Field::ALL.to_vec(),
            color: true,
        }
    }
}

/// Loads the config from `explicit`, or from the default XDG location if it exists.
/// Returns the config plus warnings about lines that were skipped.
pub fn load(explicit: Option<&Path>) -> Result<(Config, Vec<String>), String> {
    let (path, required) = match explicit {
        Some(p) => (p.to_owned(), true),
        None => match default_path() {
            Some(p) => (p, false),
            None => return Ok((Config::default(), Vec::new())),
        },
    };

    match fs::read_to_string(&path) {
        Ok(s) => Ok(parse(&s, &path.display().to_string())),
        Err(e) if e.kind() == io::ErrorKind::NotFound && !required => {
            Ok((Config::default(), Vec::new()))
        }
        Err(e) => Err(format!("{}: {e}", path.display())),
    }
}

fn default_path() -> Option<PathBuf> {
    let base = env::var_os("XDG_CONFIG_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
    Some(base.join("beefetch/config"))
}

/// Parses `key = value` lines; `#` starts a comment line.
fn parse(s: &str, origin: &str) -> (Config, Vec<String>) {
    let mut config = Config::default();
    let mut warnings = Vec::new();

    for (n, line) in s.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let result = match line.split_once('=') {
            None => Err("expected `key = value`".to_owned()),
            Some((key, value)) => match (key.trim(), value.trim()) {
                ("fields", v) => info::parse_fields(v).map(|f| config.fields = f),
                ("color", v) => v
                    .parse()
                    .map(|c| config.color = c)
                    .map_err(|_| format!("expected `true` or `false`, got `{v}`")),
                (k, _) => Err(format!("unknown key `{k}`")),
            },
        };

        if let Err(e) = result {
            warnings.push(format!("{origin}:{}: {e}", n + 1));
        }
    }

    (config, warnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_keys_and_comments() {
        let s = "# comment\n\nfields = ram, os\ncolor = false\n";
        let (config, warnings) = parse(s, "cfg");
        assert_eq!(
            config,
            Config {
                fields: vec![Field::Ram, Field::Os],
                color: false
            }
        );
        assert!(warnings.is_empty());
    }

    #[test]
    fn bad_lines_warn_and_keep_defaults() {
        let s = "fields = os, cpu\ncolor = yes\nfoo = 1\njunk\n";
        let (config, warnings) = parse(s, "cfg");
        assert_eq!(config, Config::default());
        assert_eq!(
            warnings.iter().map(|w| &w[..6]).collect::<Vec<_>>(),
            ["cfg:1:", "cfg:2:", "cfg:3:", "cfg:4:"]
        );
    }

    #[test]
    fn missing_explicit_file_is_an_error() {
        assert!(load(Some(Path::new("/nonexistent/beefetch"))).is_err());
    }
}
