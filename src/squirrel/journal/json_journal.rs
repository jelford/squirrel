
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path};

use serde_json;

use errors::*;
use squirrel::event::*;

pub(crate) struct JsonJournal<T> where T : Write {
    event_log: T,
}

pub(crate) fn new(stash_path: &Path) -> Result<JsonJournal<File>> {
    let event_log_path = stash_path.join("event-log").to_owned();
    let event_log = OpenOptions::new().read(true).append(true).create(true).open(event_log_path)?;
    Ok(JsonJournal {
        event_log: event_log
    })
}

impl <T> super::Journal for JsonJournal<T> where T: Write {
    fn journal(&mut self, event: Event) -> Result<()> {
        match serde_json::to_writer(&mut self.event_log, &event) {
            Err(_) => {
                return Err(ErrorKind::EventJournallingError("While trying to write out to journal file".to_string()).into())
            }
            _ => {}
        };
        self.event_log.write(b"\n")?;
        self.event_log.flush()?;

        Ok(())

    }
}