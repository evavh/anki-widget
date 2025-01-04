use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::{env, fs};

use color_eyre::eyre::{eyre, Context, OptionExt, Result};
use color_eyre::Section;

pub(crate) fn find_db(
    override_path: Option<PathBuf>,
    user_profile: Option<String>,
) -> Result<PathBuf> {
    let possible_paths: Vec<PathBuf> = override_path
        .map::<Result<_>, _>(|p| Ok(vec![p]))
        .unwrap_or_else(|| {
            Ok(default_paths()
                .wrap_err("Failed to enumerate possible paths")?)
        })?;

    let paths_per_install: Vec<_> = possible_paths
        .iter()
        .filter_map(|possible_path| {
            find_files(&possible_path, "collection.anki2").ok()
        })
        .filter(|collection_paths| !collection_paths.is_empty())
        .collect();

    let mut collection_paths = match &paths_per_install.len() {
        0 => {
            return Err(eyre!("No Anki data paths found")
                .note(format!(
                    "Checked the following paths: {possible_paths:#?}"
                ))
                .suggestion("Run with --path <PATH> to use a different path"));
        }
        1 => paths_per_install[0].clone(),
        _ => {
            return Err(eyre!("Multiple Anki data paths found")
                .note(format!(
                "Found the following paths: {:#?}",
                trim_to_install_path(paths_per_install)
                .wrap_err("While trimming to install path")?
            ))
                .note("Do you have multiple Anki installs?")
                .suggestion("Run with --path <PATH> to select one"));
        }
    };

    if let Some(user_profile) = user_profile {
        collection_paths = filter_profile(collection_paths, user_profile);
    }

    let path = match collection_paths.len() {
        0 => unreachable!("Filtered out empty data paths"),
        1 => collection_paths[0].clone(),
        _ => {
            return Err(eyre!("Multiple Anki collections found")
                .note(format!(
                "Found the following collection files: {collection_paths:#?}"
            ))
                .note("Do you have multiple Anki user profiles?")
                .suggestion(format!(
                    "Profiles found: {:?}",
                    get_profile_names(collection_paths)
                ))
                .suggestion(
                    "Run with --user-profile <PROFILE> to select one",
                ));
        }
    };

    Ok(path)
}

fn filter_profile(
    collection_paths: Vec<PathBuf>,
    profile: String,
) -> Vec<PathBuf> {
    collection_paths
        .into_iter()
        .filter(|path| path.components().any(|c| *c.as_os_str() == *profile))
        .collect()
}

fn trim_to_install_path(paths: Vec<Vec<PathBuf>>) -> Result<Vec<PathBuf>> {
    paths
        .into_iter()
        .flatten()
        .map(|full_path| {
            let mut new_path = full_path;
            while new_path
                .file_name()
                .ok_or_eyre("No Anki2 in install path")?
                != "Anki2"
            {
                new_path = new_path
                    .parent()
                    .ok_or_eyre("No Anki2 in install path")?
                    .to_path_buf();
            }
            Ok(new_path)
        })
        .collect()
}

fn get_profile_names(paths: Vec<PathBuf>) -> Vec<OsString> {
    paths
        .iter()
        .filter_map(|p| {
            Some(
                p.components()
                    .skip_while(|c| c.as_os_str() != "Anki2")
                    .nth(1)?
                    .as_os_str()
                    .to_owned(),
            )
        })
        .collect()
}

fn default_paths() -> Result<Vec<PathBuf>> {
    let home = env::var("HOME").wrap_err("Failed to read $HOME")?;
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
    Ok(paths)
}

fn find_files(root: &Path, filename: &str) -> Result<Vec<PathBuf>> {
    let mut results = Vec::new();

    if root.is_dir() {
        for entry in fs::read_dir(root)? {
            let path = entry?.path();

            if path.is_dir() {
                let found = find_files(&path, filename)?;
                results.extend(found);
            } else {
                if path.file_name().ok_or_eyre(eyre!(
                    "Unexpected empty filename for path {path:?}"
                ))? == filename
                {
                    results.push(path);
                }
            }
        }
    }
    Ok(results)
}
