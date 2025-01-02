use std::{env, thread, time::Duration};

use anki::{
    collection::CollectionBuilder,
    error::{AnkiError, DbErrorKind, Result},
    timestamp::TimestampSecs,
};

const REFRESH_RATE_MINUTES: u64 = 1;
const REFRESH_RATE: Duration = Duration::from_secs(REFRESH_RATE_MINUTES * 60);

const RETRY_DB: Duration = Duration::from_secs(10);
const RETRY_ERR: Duration = Duration::from_secs(10);

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
    // TODO auto-find
    let path = env!("HOME").to_owned()
        + "/.var/app/net.ankiweb.Anki/data/Anki2/User 1/collection.anki2";

    loop {
        {
            let Some(mut collection) =
                log_errors_and_sleep(CollectionBuilder::new(&path).build())
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
