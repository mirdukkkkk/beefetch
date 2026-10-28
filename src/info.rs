use std::{env, fs};

const NA: &str = "N/A";

pub struct Info {
    pub user: String,
    pub host: String,
    pub os: String,
    pub kernel: String,
    pub uptime: String,
    pub ram: String,
    pub wm: String,
    pub server: String,
}

impl Info {
    pub fn collect() -> Self {
        Self {
            user: env_var("USER")
                .or_else(user_from_passwd)
                .unwrap_or_else(|| NA.into()),
            host: read_trimmed("/proc/sys/kernel/hostname")
                .or_else(|| read_trimmed("/etc/hostname"))
                .unwrap_or_else(|| NA.into()),
            os: fs::read_to_string("/etc/os-release")
                .or_else(|_| fs::read_to_string("/usr/lib/os-release"))
                .ok()
                .and_then(|s| pretty_name(&s))
                .unwrap_or_else(|| "Linux".into()),
            kernel: read_trimmed("/proc/sys/kernel/osrelease").unwrap_or_else(|| NA.into()),
            uptime: fs::read_to_string("/proc/uptime")
                .ok()
                .and_then(|s| uptime(&s))
                .unwrap_or_else(|| NA.into()),
            ram: fs::read_to_string("/proc/meminfo")
                .ok()
                .and_then(|s| ram(&s))
                .unwrap_or_else(|| NA.into()),
            wm: env_var("XDG_CURRENT_DESKTOP")
                .or_else(|| env_var("DESKTOP_SESSION"))
                .unwrap_or_else(|| {
                    if env_var("WAYLAND_DISPLAY").is_some() {
                        "Wayland".into()
                    } else if env_var("DISPLAY").is_some() {
                        "X11".into()
                    } else {
                        NA.into()
                    }
                }),
            server: env_var("XDG_SESSION_TYPE").unwrap_or_else(|| "tty".into()),
        }
    }
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
}
