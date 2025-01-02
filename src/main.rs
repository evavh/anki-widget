use std::{
    env, fs, io,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use anki::{
    collection::CollectionBuilder,
    error::{AnkiError, DbErrorKind, Result},
    timestamp::TimestampSecs,
};

const REFRESH_RATE_MINUTES: u64 = 1;
const REFRESH_RATE: Duration = Duration::from_secs(REFRESH_RATE_MINUTES * 60);

const RETRY_DB: Duration = Duration::from_secs(10);
const RETRY_ERR: Duration = Duration::from_secs(10);

fn paths() -> Vec<PathBuf> {
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

fn log_errors_and_sleep<T>(result: Result<T>) -> Option<T> {
    match result {
        Ok(val) => Some(val),
        Err(AnkiError::DbError { source })
            if source.kind == DbErrorKind::Locked =>
        {
            println!("Anki is active");
            thread::sleep(RETRY_DB);
            None
        }
        Err(other) => {
            println!("Anki error");
            eprintln!("Anki error: {other}");
            thread::sleep(RETRY_ERR);
            None
        }
    }
}

fn main() {
    let paths_per_install: Vec<_> = paths()
        .into_iter()
        .map(|install_path| {
            find_files(&install_path, "collection.anki2").unwrap()
        })
        .filter(|collections| !collections.is_empty())
        .collect();

    let collection_paths = match paths_per_install.len() {
        0 => {
            eprintln!("No anki paths found");
            return;
        }
        1 => &paths_per_install[0],
        _ => {
            eprintln!(
                "Multiple anki paths found: {:#?}",
                paths_per_install.into_iter().flatten().collect::<Vec<_>>()
            );
            eprintln!("Do you have multiple anki installs?");
            return;
        }
    };

    let path = match collection_paths.len() {
        0 => unreachable!("Filtered out empty install paths"),
        1 => &collection_paths[0],
        _ => {
            eprintln!("Multiple anki collections found: {collection_paths:#?}");
            eprintln!("Do you have multiple anki profiles?");
            return;
        }
    };

    loop {
        {
            let Some(mut collection) =
                log_errors_and_sleep(CollectionBuilder::new(path).build())
            else {
                continue;
            };
            let now = TimestampSecs::now();
            let Some(deck_tree) =
                log_errors_and_sleep(collection.deck_tree(Some(now)))
            else {
                continue;
            };

            let new = deck_tree.new_count;
            let learn = deck_tree.learn_count;
            let review = deck_tree.review_count;
            let total_due = learn + review;

            println!("Anki - new: {new}, due: {total_due}");
        }
        thread::sleep(REFRESH_RATE);
    }
}
