use std::{collections::hash_map::Entry, fs, io, path::Path};

use crate::trackfile::TrackFile;

pub fn list(file: &mut TrackFile, _matches: &clap::ArgMatches<'_>) -> io::Result<()> {
    for (k, v) in file.map().iter() {
        println!("entry: '{}' ~> path: '{}'", k, v.file());
    }
    Ok(())
}

pub fn insert(file: &mut TrackFile, matches: &clap::ArgMatches<'_>) -> io::Result<()> {
    let map = file.map_mut();
    let path = matches.value_of("file").unwrap();
    let abbr = matches.value_of("abbr").unwrap_or(path);
    match map.entry(abbr.into()) {
        Entry::Occupied(occ) => {
            eprintln!(
                "entry '{}' for path '{}' is already present",
                occ.key(),
                occ.get().file()
            );
        }
        Entry::Vacant(vac) => {
            println!("entry '{}' for path '{}' inserted", vac.key(), &path);
            let entry = crate::trackfile::Entry::new(path.into(), vac.key());
            vac.insert(entry);
        }
    }
    Ok(())
}

pub fn remove(file: &mut TrackFile, matches: &clap::ArgMatches<'_>) -> io::Result<()> {
    let map = file.map_mut();
    let entry = matches.value_of("entry").unwrap();
    match map.entry(entry.into()) {
        Entry::Occupied(occ) => {
            println!(
                "entry '{}' for path '{}' was removed",
                occ.key(),
                occ.get().file()
            );
            occ.remove_entry();
        }
        Entry::Vacant(vac) => {
            eprintln!("entry '{}' not present, nothing to remove", vac.key());
        }
    }
    Ok(())
}

pub fn clear(_ws: &mut TrackFile, _matches: &clap::ArgMatches<'_>) -> io::Result<()> {
    let a = fs::read_dir(".")?.filter_map(Result::ok).find(|entry| {
        matches!(
            entry.path().extension().and_then(std::ffi::OsStr::to_str),
            Some("tar")
        )
    });
    let archive = a.as_ref().map(fs::DirEntry::path);
    let dir = archive
        .as_deref()
        .and_then(Path::file_stem)
        .map(Path::new)
        .and_then(|p| if p.exists() { Some(p) } else { None });

    println!("following files will be removed: ");
    if let Some(ref archive_path) = archive {
        println!("archive: '{}'", archive_path.display());
    }
    if let Some(ref dir_path) = dir {
        println!("directory: '{}'", dir_path.display());
    }

    let result = super::query("these files will be deleted, do you want to continue? [y/n]")?;
    if matches!(result.chars().next(), Some('y') | Some('Y') | None) {
        if let Some(ref archive_path) = archive {
            fs::remove_file(archive_path)?;
        }
        if let Some(ref dir_path) = dir {
            fs::remove_dir_all(dir_path)?;
        }
    } else {
        println!("clearing was skipped");
    }
    Ok(())
}
