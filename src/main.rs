#[macro_use]
extern crate clap;

mod commands;
mod trackfile;

use crate::trackfile::TrackFile;
use clap::{App, Arg};

pub(crate) const TRACK_FILE: &str = concat!(env!("HOME"), "/.config/rconfig/track_file.json");

fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("manage config files and edit them faster")
        .usage(concat!(
            crate_name!(),
            " [list|track|untrack|edit|save|load|clear]"
        ))
        .arg(
            Arg::with_name("track")
                .help("track file path")
                .short("t")
                .long("track")
                .takes_value(true),
        )
        .subcommands(vec![
            commands::list_cli(),
            commands::track_cli(),
            commands::untrack_cli(),
            commands::edit_cli(),
            commands::save_cli(),
            commands::load_cli(),
            commands::clear_cli(),
        ])
}

fn main() -> std::io::Result<()> {
    let matches = build_app().get_matches();

    let path = matches.value_of("track").map(std::path::Path::new);
    let mut file = if path.is_some() {
        TrackFile::open(path.unwrap())
    } else {
        TrackFile::open(TRACK_FILE)
    }?;
    match matches.subcommand() {
        ("list", Some(sub)) => commands::track::list(&mut file, sub),
        ("track", Some(sub)) => commands::track::insert(&mut file, sub),
        ("untrack", Some(sub)) => commands::track::remove(&mut file, sub),
        ("edit", Some(sub)) => commands::config::edit(&mut file, sub),
        ("save", Some(sub)) => commands::config::save(&mut file, sub),
        ("load", Some(sub)) => commands::config::load(&mut file, sub),
        ("clear", Some(sub)) => commands::track::clear(&mut file, sub),
        _ => {
            eprintln!("{}", matches.usage());
            Ok(())
        }
    }
}
