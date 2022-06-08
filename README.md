# Doen

Minimal cli based todo tool

## Installation

Installing the project requires cargo to be installed

```sh
git clone git://git.peeko.nl/doen
cd doen
cargo b --release
sudo ln -s $PWD/target/release/doen /usr/bin
```

## Usage

```
doen 

USAGE:
    doen [SUBCOMMAND]

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    add       Add a new todo
    help      Print this message or the help of the given subcommand(s)
    remove    Removes a todo
    show      Displays the active todos ordered by priority (default subcommand)
```
