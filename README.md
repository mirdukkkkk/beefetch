# 🐝fetch

![Static Badge](https://img.shields.io/badge/%F0%9F%90%9D-bee-yellow?style=for-the-badge) ![GitHub License](https://img.shields.io/github/license/mirdukkkkk/beefetch?style=for-the-badge) ![GitHub Tag](https://img.shields.io/github/v/tag/mirdukkkkk/beefetch?style=for-the-badge&label=version) ![AUR Version](https://img.shields.io/aur/version/beefetch?style=for-the-badge)

> A 🐝 is _buzzing_

## Screenshot (🐝)

![🐝](docs/beefetch.png)

## Build

```bash
cargo build --release
# binary: target/release/beefetch
```

### Nix

```bash
nix run github:mirdukkkkk/beefetch
nix develop   # dev shell with cargo, clippy, rustfmt, rust-analyzer, hyperfine
```

## Usage

```
beefetch [-n] [-f LIST] [-c PATH]

-n, --no-color     Disable ANSI colors (also NO_COLOR=1)
-f, --fields LIST  Fields to show, in order: os, kernel, uptime, ram, wm, server
-c, --config PATH  Config file (also BEEFETCH_CONFIG)
```

## Config

`$XDG_CONFIG_HOME/beefetch/config` (usually `~/.config/beefetch/config`):

```
# fields to show, in order
fields = os, kernel, uptime, ram, wm, server
color = true
```

Command-line flags override the config.

## Go v2 vs Rust v3

| | Go v2 | Rust v3 |
|---|---|---|
| Binary size | 2.1 MB | 316 KB |
| Startup (`hyperfine -N`) | 1.5 ms | 0.66 ms |

## 🐝's

[@mirdukkkkk](https://github.com/mirdukkkkk), [@EtherCD](https://github.com/EtherCD)
