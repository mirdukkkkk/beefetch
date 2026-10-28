use std::str::FromStr;
use std::{env, fs};

const NA: &str = "N/A";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Os,
    Kernel,
    Uptime,
    Ram,
    Wm,
    Server,
}

impl Field {
    pub const ALL: [Field; 6] = [
        Field::Os,
        Field::Kernel,
        Field::Uptime,
        Field::Ram,
        Field::Wm,
        Field::Server,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Field::Os => "os",
            Field::Kernel => "kernel",
            Field::Uptime => "uptime",
            Field::Ram => "ram",
            Field::Wm => "wm",
            Field::Server => "server",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Field::Os => "OS:",
            Field::Kernel => "Kernel:",
            Field::Uptime => "Uptime:",
            Field::Ram => "RAM:",
            Field::Wm => "WM:",
            Field::Server => "Server:",
        }
    }

    pub fn collect(self) -> String {
        let value = match self {
            Field::Os => {
                return fs::read_to_string("/etc/os-release")
                    .or_else(|_| fs::read_to_string("/usr/lib/os-release"))
                    .ok()
                    .and_then(|s| pretty_name(&s))
                    .unwrap_or_else(|| "Linux".into());
            }
            Field::Kernel => read_trimmed("/proc/sys/kernel/osrelease"),
            Field::Uptime => fs::read_to_string("/proc/uptime")
                .ok()
                .and_then(|s| uptime(&s)),
            Field::Ram => fs::read_to_string("/proc/meminfo")
                .ok()
                .and_then(|s| ram(&s)),
            Field::Wm => env_var("XDG_CURRENT_DESKTOP")
                .or_else(|| env_var("DESKTOP_SESSION"))
                .or_else(|| {
                    if env_var("WAYLAND_DISPLAY").is_some() {
                        Some("Wayland".into())
                    } else if env_var("DISPLAY").is_some() {
                        Some("X11".into())
                    } else {
                        None
                    }
                }),
            Field::Server => {
                return env_var("XDG_SESSION_TYPE").unwrap_or_else(|| "tty".into());
            }
        };
        value.unwrap_or_else(|| NA.into())
    }
}

impl FromStr for Field {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Field::ALL
            .into_iter()
            .find(|f| f.name().eq_ignore_ascii_case(s))
            .ok_or_else(|| {
                let names: Vec<_> = Field::ALL.iter().map(|f| f.name()).collect();
                format!("unknown field `{s}`, expected one of: {}", names.join(", "))
            })
    }
}

/// Parses a comma-separated field list; empty items are ignored.
pub fn parse_fields(list: &str) -> Result<Vec<Field>, String> {
    list.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .collect()
}

pub fn user() -> String {
    env_var("USER")
        .or_else(user_from_passwd)
        .unwrap_or_else(|| NA.into())
}

pub fn host() -> String {
    read_trimmed("/proc/sys/kernel/hostname")
        .or_else(|| read_trimmed("/etc/hostname"))
        .unwrap_or_else(|| NA.into())
}

fn env_var(key: &str) -> Option<String> {
    env::var(key).ok().filter(|v| !v.is_empty())
}

fn read_trimmed(path: &str) -> Option<String> {
    let s = fs::read_to_string(path).ok()?;
    let s = s.trim();
    (!s.is_empty()).then(|| s.to_owned())
}

fn user_from_passwd() -> Option<String> {
    let status = fs::read_to_string("/proc/self/status").ok()?;
    let uid = status
        .lines()
        .find_map(|l| l.strip_prefix("Uid:"))?
        .split_whitespace()
        .next()?;

    fs::read_to_string("/etc/passwd")
        .ok()?
        .lines()
        .find_map(|l| {
            let mut fields = l.split(':');
            let name = fields.next()?;
            (fields.nth(1)? == uid).then(|| name.to_owned())
        })
}

fn pretty_name(os_release: &str) -> Option<String> {
    let value = os_release
        .lines()
        .find_map(|l| l.strip_prefix("PRETTY_NAME="))?
        .trim_matches(['"', '\'']);
    (!value.is_empty()).then(|| value.to_owned())
}

fn uptime(proc_uptime: &str) -> Option<String> {
    let secs: u64 = proc_uptime.split(['.', ' ']).next()?.parse().ok()?;
    let (d, h, m) = (secs / 86400, secs % 86400 / 3600, secs % 3600 / 60);

    Some(if d > 0 {
        format!("{d}d {h}h {m}m")
    } else {
        format!("{h}h {m}m")
    })
}

fn ram(meminfo: &str) -> Option<String> {
    let (mut total, mut available) = (None, None);

    for line in meminfo.lines() {
        if let Some(v) = line.strip_prefix("MemTotal:") {
            total = kib(v);
        } else if let Some(v) = line.strip_prefix("MemAvailable:") {
            available = kib(v);
        }
        if total.is_some() && available.is_some() {
            break;
        }
    }

    let (total, available) = (total?, available?);
    Some(format!(
        "{} MiB / {} MiB",
        total.saturating_sub(available) / 1024,
        total / 1024
    ))
}

fn kib(value: &str) -> Option<u64> {
    value.split_whitespace().next()?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretty_name_keeps_equals_signs() {
        let s = "NAME=Foo\nPRETTY_NAME=\"Foo a=b\"\nID=foo\n";
        assert_eq!(pretty_name(s).as_deref(), Some("Foo a=b"));
        assert_eq!(pretty_name("NAME=Foo\n"), None);
        assert_eq!(pretty_name("PRETTY_NAME=\"\"\n"), None);
    }

    #[test]
    fn uptime_formats_days() {
        assert_eq!(uptime("59.99 100.00\n").as_deref(), Some("0h 0m"));
        assert_eq!(uptime("3725.10 1.00\n").as_deref(), Some("1h 2m"));
        assert_eq!(uptime("90061.00 1.00\n").as_deref(), Some("1d 1h 1m"));
        assert_eq!(uptime(""), None);
    }

    #[test]
    fn ram_uses_mem_available() {
        let s = "MemTotal:       32782336 kB\nMemFree:  1000 kB\nMemAvailable:   19060736 kB\n";
        assert_eq!(ram(s).as_deref(), Some("13400 MiB / 32014 MiB"));
        assert_eq!(ram("MemTotal: 1024 kB\n"), None);
    }

    #[test]
    fn field_list_parsing() {
        assert_eq!(
            parse_fields(" RAM, os ,,kernel"),
            Ok(vec![Field::Ram, Field::Os, Field::Kernel])
        );
        assert_eq!(parse_fields(""), Ok(vec![]));
        assert!(parse_fields("os,cpu").unwrap_err().contains("`cpu`"));
    }
}
