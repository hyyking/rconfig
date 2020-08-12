# _rconfig_

rust config file manager

## Usage

```
> rconfig --help

manage config files and edit them faster

USAGE:
    rconfig [list|track|untrack|edit|save|load|clear]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --track <track>    track file path

SUBCOMMANDS:
    clear      clear all tracked config archives and files
    edit       edit tracked configs
    help       Prints this message or the help of the given subcommand(s)
    list       list all tracked files
    load       load a tracked config archive
    save       save all tracked configs to an archive
    track      add a config file to track file
    untrack    remove an entry from track file
```

## Installation

1. clone the repository
2. install/run with cargo (resp. cargo install --path . | cargo run)
