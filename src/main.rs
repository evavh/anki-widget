use std::{path::PathBuf, thread, time::Duration};

use anki::{
    collection::CollectionBuilder,
    error::{AnkiError, DbErrorKind, Result},
    timestamp::TimestampSecs,
};
use clap::{command, Parser};

mod path;

/// A widget that shows Anki's current due and new card counts.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Minutes between checking the database for new card
    /// counts.
    #[arg(
        short,
        long,
        default_value_t = 1,
        value_name = "MINUTES",
        verbatim_doc_comment
    )]
    refresh_delay: u64,
    /// Seconds between retries when the database is in use,
    /// or some other error occurs.
    #[arg(
        short = 't',
        long,
        default_value_t = 10,
        value_name = "SECONDS",
        verbatim_doc_comment
    )]
    retry_delay: u64,
    /// The full path to your Anki2 folder, by default the
    /// widget will search for this. Use this if you have
    /// a custom path, or multiple paths were found.
    #[arg(short, long, verbatim_doc_comment)]
    path: Option<PathBuf>,
    /// The user profile to use. Use this if multiple
    /// profiles were found.
    #[arg(short, long, value_name = "PROFILE", verbatim_doc_comment)]
    user_profile: Option<String>,
}

fn main() {
    let args = Args::parse();
    let refresh_delay = Duration::from_secs(60 * args.refresh_delay);
    let retry_delay = Duration::from_secs(args.retry_delay);

    let Some(db_path) = path::find_db(args.path, args.user_profile) else {
        return;
    };

    loop {
        {
            let Some(mut collection) = log_errors_and_sleep(
                CollectionBuilder::new(&db_path).build(),
                retry_delay,
            ) else {
                continue;
            };
            let now = TimestampSecs::now();
            let Some(deck_tree) = log_errors_and_sleep(
                collection.deck_tree(Some(now)),
                retry_delay,
            ) else {
                continue;
            };

            let new = deck_tree.new_count;
            let learn = deck_tree.learn_count;
            let review = deck_tree.review_count;
            let total_due = learn + review;

            println!("Anki - new: {new}, due: {total_due}");
        }
        thread::sleep(refresh_delay);
    }
}

fn log_errors_and_sleep<T>(
    result: Result<T>,
    retry_delay: Duration,
) -> Option<T> {
    match result {
        Ok(val) => Some(val),
        Err(AnkiError::DbError { source })
            if source.kind == DbErrorKind::Locked =>
        {
            println!("Anki is active");
            thread::sleep(retry_delay);
            None
        }
        Err(other) => {
            println!("Anki error");
            eprintln!("Anki error: {other}");
            thread::sleep(retry_delay);
            None
        }
    }
}
