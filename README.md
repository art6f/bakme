# 🔙 bakme
A small, lightweight cli 'bak'-ing tool

> One day I was working on making my prompts prettier in `.zshrc` and realised that doing manual `cp .zshrc .zshrc.bak` is annoying. Are there commands for it?

👉 **Now it is!**

## What it does
`bakme` is a simple, written in Rust, lightweight CLI tool that create files and folders copies and adds the `.bak` extenstion.

But it also does in a smart way – you can easily create 15 `.bak` files up to `.bak-15` or even more (you shall not, but who can stop anyway).

## Let's have an argument
| Flag | What it does |
|------|-------------|
| `-e`, `--extension` | Custom suffix instead of `.bak` |
| `-t`, `--timed`| Use `_YYYYMMDD_HHMMSS` as a sufix (higher prio than `-e`) |
| `-f`, `--force` | Ignores the copy safety cap and creates as many as you want |
| `--verbose` | Extra output for fun or science |

### It's safe for your older backups
By default program won't make more than 15 copiesof the same file unless you tell it to.

Collisions get number-suffixed automatically (`file.bak-1`, `file.bak-2`, etc.).

Your files are preserwed, not overwritten, but also contest won't be checked


## Some example

```bash
# The basics — create a copy and slap ".bak" on it
bakme config.toml
# → config.toml.bak

# Custom extension
bakme -e old notes.md
# → notes.md.old

# Feeling fancy or precise – timestamp it
bakme -t src/main.rs
# → src/main.rs.bak_20261109_143022

# Back up an entire directory tree
bakme project/

# Already have too many backups? Override the safety limit
bakme -f data/
# → data.bak-16/

# See extra into and clog the terminal
bakme --verbose -f project/
```


> ### **Disclaimer:** Use at your own risk, you are responsible for your files
