use std::{path::PathBuf, thread, time::Duration};

use anki::{
    collection::CollectionBuilder,
    error::{AnkiError, DbErrorKind},
    timestamp::TimestampSecs,
};
use clap::{command, Parser, Subcommand};
use color_eyre::eyre::{Context, Result};

mod path;

/// A widget that shows Anki's current due and new card counts
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Choose one-shot or continuous mode
    #[command(subcommand)]
    command: Command,
    /// Print only the card counts, in the form <due> / <new>.
    #[arg(short, long, verbatim_doc_comment)]
    short: bool,
    /// Print output as machine-readable json, with keys "msg",
    /// "due", and "new".
    #[arg(short, long, verbatim_doc_comment)]
    json: bool,
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

#[derive(Subcommand)]
enum Command {
    /// Print output once and then quit. Used for GNOME extensions
    /// and text bars that do the refreshing for you by running the
    /// command again.
    #[command(verbatim_doc_comment)]
    OneShot,
    /// Print output every minute (by default). Used for text bars
    /// that only run the command once and expect output to change.
    /// Settings: --refresh-delay, --retry-delay
    #[command(verbatim_doc_comment)]
    Continuous {
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
    },
}

#[derive(Clone, Copy)]
struct Format {
    short: bool,
    json: bool,
}

fn main() -> Result<()> {
    color_eyre::install().unwrap();

    let args = Args::parse();

    let format = Format {
        short: args.short,
        json: args.json,
    };

    let db_path = path::find_db(args.path, args.user_profile)
        .wrap_err("Couldn't auto-pick Anki database path")?;

    match args.command {
        Command::OneShot => {
            retrieve_and_print(&db_path, Duration::from_secs(0), format);
        }
        Command::Continuous {
            refresh_delay,
            retry_delay,
        } => {
            let refresh_delay = Duration::from_secs(60 * refresh_delay);
            let retry_delay = Duration::from_secs(retry_delay);

            loop {
                if let Success::Yes =
                    retrieve_and_print(&db_path, retry_delay, format)
                {
                    thread::sleep(refresh_delay);
                }
            }
        }
    }

    Ok(())
}

enum Success {
    Yes,
    No,
}

fn retrieve_and_print(
    db_path: &PathBuf,
    retry_delay: Duration,
    format: Format,
) -> Success {
    let Some(mut collection) = log_errors_and_sleep(
        CollectionBuilder::new(db_path).build(),
        retry_delay,
    ) else {
        return Success::No;
    };
    let now = TimestampSecs::now();
    let Some(deck_tree) =
        log_errors_and_sleep(collection.deck_tree(Some(now)), retry_delay)
    else {
        return Success::No;
    };

    let new = deck_tree.new_count;
    let learn = deck_tree.learn_count;
    let review = deck_tree.review_count;
    let total_due = learn + review;

    let output = if format.short {
        format!("{total_due} / {new}")
    } else {
        format!("Anki - due: {total_due}, new: {new}")
    };

    if format.json {
        println!(
            "{{\"msg\": \"{output}\", \"due\": {total_due}, \"new\": {new}}}"
        );
    } else {
        println!("{output}");
    }

    Success::Yes
}

fn log_errors_and_sleep<T>(
    result: anki::error::Result<T>,
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
