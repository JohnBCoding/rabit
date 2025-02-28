# 🐇 rabit 🐇

CLI tool for tracking your habits.

## Features
- Daily tracking of habits (🐇s?)
- Easily `Track` with checkboxes or unique values for different types of 🐇 tracking!
- `Observe` your 🐇s in the terminal to see where you bounced!
- Everything stored in one JSON file.

## Usage

```
rabit 0.2.0
A simple habit CLI

Usage: rabit [COMMAND]

Commands:
  track    Track a Rabit
  cull     Cull A Rabit
  observe  View Rabit(s)
  config   Configure CLI Options
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Install

If you currently have cargo installed you can simply run the following to install:

> run cargo install rabit

## Building

🐇 can be built with the rust toolchain

```
git clone https://github.com/JohnBCoding/rabit.git
cd rabit
cargo build --release
```