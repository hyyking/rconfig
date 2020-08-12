use std::{convert, fs, io, path::Path, process::Command};

use crate::trackfile::{Entry, TrackFile};

pub fn edit(file: &mut TrackFile, matches: &clap::ArgMatches<'_>) -> io::Result<()> {
    let map = file.map();
    let editor = matches.value_of("editor").unwrap();
    let mut cmd = Command::new(editor);
    if let Some(value) = matches.value_of("flags") {
        cmd.arg(value);
    }
    let entries: Vec<_> = matches
        .values_of("entry")
        .unwrap()
        .map(|entry| {
            let file = map.get(entry).map(Entry::file);
            if file.is_none() {
                eprintln!("entry '{}' was skipped because it does not exist", entry);
            }
            file
        })
        .filter_map(convert::identity)
        .collect();
    cmd.args(&entries);

    let mut child = cmd.spawn()?;
    let exit_code = child.wait()?;

    if !(exit_code.success() && matches!(exit_code.code(), None | Some(0))) {
        eprintln!("editor process ended with an error");
        eprintln!("error code: {}", exit_code.code().unwrap());
    }

    Ok(())
}

pub fn save(file: &mut TrackFile, matches: &clap::ArgMatches<'_>) -> io::Result<()> {
    let ovrr = matches.is_present("override");
    let archive_path = matches
        .value_of("path")
        .map(Path::new)
        .unwrap()
        .with_extension("tar");
    let dir = archive_path.file_stem().map(Path::new).unwrap();

    // create file for archive
    let archive_file = if archive_path.exists() && ovrr {
        fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&archive_path)?
    } else if archive_path.exists() {
        eprintln!(
            "file '{}' already exists and could be overwritten with flag --override",
            archive_path.display()
        );
        return Ok(());
    } else {
        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&archive_path)?
    };

    let mut builder = tar::Builder::new(archive_file);
    builder.follow_symlinks(matches.is_present("symlinks"));
    // add track_file
    println!("adding track file '{}' to archive", file.path().display());
    builder.append_path_with_name(file.path(), dir.join(file.path().file_name().unwrap()))?;
    for entry in file.map().values() {
        println!("adding config file '{}' to archive", entry.file());
        builder.append_path_with_name(entry.file(), dir.join(format!("{}.entry", entry.hash())))?;
    }
    builder.finish()?;

    Ok(())
}

pub fn load(file: &mut TrackFile, matches: &clap::ArgMatches<'_>) -> io::Result<()> {
    let ovrr = matches.is_present("unchecked");
    let archive_path = matches.value_of("path").map(Path::new).unwrap();

    // unpack archive directory
    let mut ar = tar::Archive::new(fs::File::open(archive_path)?);
    ar.unpack(".")?;

    // load archive track file
    let dir = Path::new(archive_path.file_stem().unwrap());
    let file = {
        let track_file = dir.join(file.path().file_name().unwrap());
        TrackFile::open(&track_file)?
    };
    'outer: for (k, entry) in file.map() {
        let hash = entry.hash();
        let mut output_path = String::from(entry.file());
        // query each path
        if !ovrr {
            loop {
                let result = super::query(&format!(
                    "entry '{}' is linked to '{}' take action [install/skip/change]: ",
                    k, output_path
                ))?;
                match result.to_lowercase().as_str() {
                    "change" | "c" => {
                        output_path = super::query("enter new path: ")?;
                        break;
                    }
                    "skip" | "s" => continue 'outer,
                    "install" | "i" | "" => break,
                    _ => continue,
                }
            }
        }
        // copy file to new path
        fs::copy(format!("{}/{}.entry", dir.display(), hash), output_path)?;
    }
    Ok(())
}
