use std::env;
use std::fs;
use std::io;
use std::path::Path;

use std::path::PathBuf;

pub(crate) fn find_db() -> Option<PathBuf> {
    let paths_per_install: Vec<_> = possible_paths()
        .into_iter()
        .map(|data_path| find_files(&data_path, "collection.anki2").unwrap())
        .filter(|collections| !collections.is_empty())
        .collect();

    let collection_paths = match paths_per_install.len() {
        0 => {
            eprintln!("No Anki data paths found");
            return None;
        }
        1 => &paths_per_install[0],
        _ => {
            eprintln!(
                "Multiple Anki data paths with collection files found: {:#?}",
                trim_to_install_path(paths_per_install)
            );
            eprintln!("Do you have multiple Anki installs?");
            return None;
        }
    };

    let path = match collection_paths.len() {
        0 => unreachable!("Filtered out empty data paths"),
        1 => collection_paths[0].clone(),
        _ => {
            eprintln!("Multiple Anki collections found: {collection_paths:#?}");
            eprintln!("Do you have multiple Anki profiles?");
            return None;
        }
    };

    Some(path)
}

fn trim_to_install_path(paths: Vec<Vec<PathBuf>>) -> Vec<PathBuf> {
    paths
        .into_iter()
        .flatten()
        .map(|full_path| {
            let mut new_path = full_path;
            while new_path
                .file_name()
                .expect("Should encounter Anki2 before parents run out")
                != "Anki2"
            {
                new_path = new_path
                    .parent()
                    .expect("Should encounter Anki2 before parents run out")
                    .to_path_buf();
            }
            new_path
        })
        .collect()
}

fn possible_paths() -> Vec<PathBuf> {
    let home = env::var("HOME").unwrap();
    let home = PathBuf::from(home);

    let mut paths = vec![
        home.clone()
            .join(".var")
            .join("app")
            .join("net.ankiweb.Anki")
            .join("data")
            .join("Anki2"),
        home.join(".local").join("share").join("Anki2"),
    ];

    if let Some(data_dir) = env::var("XDG_DATA_HOME").ok() {
        let data_dir = PathBuf::from(data_dir);
        paths.push(data_dir.join("Anki2"));
    }
    paths
}

fn find_files(root: &Path, filename: &str) -> io::Result<Vec<PathBuf>> {
    let mut results = Vec::new();

    if root.is_dir() {
        for entry in fs::read_dir(root)? {
            let path = entry?.path();

            if path.is_dir() {
                let found = find_files(&path, filename)?;
                results.extend(found);
            } else {
                if path.file_name().expect("Should'nt terminate in ..")
                    == filename
                {
                    results.push(path);
                }
            }
        }
    }
    Ok(results)
}