use crate::info::Info;

const UNDERLINE_BLUE: &str = "\x1b[4;1;34m";
const RED: &str = "\x1b[1;31m";
const YELLOW: &str = "\x1b[1;33m";
const BLUE: &str = "\x1b[1;34m";
const WHITE: &str = "\x1b[1;37m";
const RESET: &str = "\x1b[0m";

/// Visible width of the bee column, including the gap before the info column.
const BEE_WIDTH: usize = 16;

/// Each line is a list of (color, text) segments; an empty color means plain text.
const BEE: [&[(&str, &str)]; 7] = [
    &[],
    &[("", "     "), (WHITE, "__")],
    &[("", "    "), (WHITE, "// \\")],
    &[("", "    "), (WHITE, "\\\\_/"), ("", " "), (YELLOW, "/")],
    &[("", "  "), (YELLOW, "--(||)("), (WHITE, "'"), (YELLOW, ")")],
    &[("", "     "), (YELLOW, "'''")],
    &[],
];

fn paint(out: &mut String, color: bool, code: &str, text: &str) {
    if color && !code.is_empty() {
        out.push_str(code);
        out.push_str(text);
        out.push_str(RESET);
    } else {
        out.push_str(text);
    }
}

pub fn render(info: &Info, color: bool) -> String {
    let fields = [
        ("OS:", &info.os),
        ("Kernel:", &info.kernel),
        ("Uptime:", &info.uptime),
        ("RAM:", &info.ram),
        ("WM:", &info.wm),
        ("Server:", &info.server),
    ];
    let mut out = String::with_capacity(512);

    for row in 0..BEE.len().max(fields.len() + 1) {
        let mut width = 0;
        for &(code, text) in BEE.get(row).copied().unwrap_or_default() {
            paint(&mut out, color, code, text);
            width += text.len();
        }

        // Row 0 is user@host, rows 1..=fields.len() are fields; skip trailing padding past that.
        if row <= fields.len() {
            out.extend(std::iter::repeat_n(' ', BEE_WIDTH.saturating_sub(width)));
        }

        if row == 0 {
            paint(&mut out, color, UNDERLINE_BLUE, &info.user);
            out.push('@');
            paint(&mut out, color, UNDERLINE_BLUE, &info.host);
        } else if let Some((label, value)) = fields.get(row - 1) {
            paint(&mut out, color, RED, label);
            out.push(' ');
            out.push_str(value);
        }
        out.push('\n');
    }

    out
}

pub fn help(color: bool) -> String {
    let mut out = String::with_capacity(256);

    paint(&mut out, color, YELLOW, "beefetch");
    out.push_str(" - neofetch clone written in Rust with 🐝\n\n");
    paint(&mut out, color, BLUE, "Usage:");
    out.push_str("\n  beefetch [options]\n\n");
    paint(&mut out, color, BLUE, "Options:");
    out.push_str(concat!(
        "\n  -h, --help       Show this help message",
        "\n  -n, --no-color   Disable ANSI colors (also NO_COLOR=1)\n",
    ));

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn info() -> Info {
        Info {
            user: "bee".into(),
            host: "hive".into(),
            os: "NixOS".into(),
            kernel: "7.3.0".into(),
            uptime: "1h 2m".into(),
            ram: "1 MiB / 2 MiB".into(),
            wm: "KDE".into(),
            server: "wayland".into(),
        }
    }

    #[test]
    fn plain_output_is_aligned() {
        let expected = concat!(
            "                bee@hive\n",
            "     __         OS: NixOS\n",
            "    // \\        Kernel: 7.3.0\n",
            "    \\\\_/ /      Uptime: 1h 2m\n",
            "  --(||)(')     RAM: 1 MiB / 2 MiB\n",
            "     '''        WM: KDE\n",
            "                Server: wayland\n",
        );
        assert_eq!(render(&info(), false), expected);
    }

    #[test]
    fn colored_output_strips_to_plain() {
        let colored = render(&info(), true);
        let mut plain = String::new();
        let mut in_escape = false;
        for c in colored.chars() {
            match (in_escape, c) {
                (false, '\x1b') => in_escape = true,
                (true, 'm') => in_escape = false,
                (false, _) => plain.push(c),
                _ => {}
            }
        }
        assert_eq!(plain, render(&info(), false));
    }
}
