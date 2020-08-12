use clap::{App, Arg, SubCommand};

pub mod config;
pub mod track;

pub fn query(query: &str) -> std::io::Result<String> {
    use std::io::Write;
    let mut buff = String::new();
    print!("{}", query);
    std::io::stdout().flush()?;
    let i = std::io::stdin();
    i.read_line(&mut buff)?;
    Ok(buff)
}

pub fn track_cli() -> App<'static, 'static> {
    SubCommand::with_name("track")
        .about("add a config file to track file")
        .arg(Arg::with_name("file").help("file to track").required(true))
        .arg(
            Arg::with_name("abbr")
                .help("abbreviation for the file [default is file path]")
                .short("a")
                .long("abbr")
                .takes_value(true),
        )
}

pub fn list_cli() -> App<'static, 'static> {
    SubCommand::with_name("list").about("list all tracked files")
}

pub fn untrack_cli() -> App<'static, 'static> {
    SubCommand::with_name("untrack")
        .about("remove an entry from track file")
        .arg(
            Arg::with_name("entry")
                .help("entry to untrack")
                .required(true),
        )
}

pub fn edit_cli() -> App<'static, 'static> {
    SubCommand::with_name("edit")
        .about("edit tracked configs")
        .arg(
            Arg::with_name("entry")
                .help("entry to edit")
                .required(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("editor")
                .help("file editor")
                .short("e")
                .long("editor")
                .default_value(env!("EDITOR"))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("flags")
                .help("editor flags")
                .short("f")
                .long("flags")
                .takes_value(true)
                .require_equals(true),
        )
}

pub fn save_cli() -> App<'static, 'static> {
    SubCommand::with_name("save")
        .about("save all tracked configs to an archive")
        .arg(Arg::with_name("path").help("archive path").required(true))
        .arg(
            Arg::with_name("override")
                .help("overide previous archive")
                .short("o")
                .long("override"),
        )
        .arg(
            Arg::with_name("symlinks")
                .help("follow symlinks when building archive")
                .long("follow-symlinks"),
        )
}

pub fn load_cli() -> App<'static, 'static> {
    SubCommand::with_name("load")
        .about("load a tracked config archive")
        .arg(Arg::with_name("path").help("archive path").required(true))
        .arg(
            Arg::with_name("unchecked")
                .help("install to tracked path without checking")
                .short("u")
                .long("unchecked"),
        )
}

pub fn clear_cli() -> App<'static, 'static> {
    SubCommand::with_name("clear").about("clear all tracked config archive and files")
}
