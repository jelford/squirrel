
use std::io;
use std::fs;
use notify::{watcher, RecursiveMode, Watcher};
use std::time::Duration;
use std::sync::mpsc::channel as sync_channel;
use std::path::Path;
use path_filter;

use errors::*;

use super::squirrel;
use super::event;
use super::journal;

fn ensure_dir(path: &Path) -> Result<()> {
    match fs::create_dir(&path) {
        Ok(()) => Ok(()),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::AlreadyExists => Ok(()),
                _ => Err(e.into()),
            }
        }
    }
}

pub(crate) fn run_squirrel(watched_dir: &Path, stash_path: &Path) -> Result<()> {
    let (change_event_tx, change_event_rx) = sync_channel();

    let mut watcher = watcher(change_event_tx, Duration::from_secs(2)).unwrap();
    watcher.watch(&"", RecursiveMode::Recursive).unwrap();
    ensure_dir(&stash_path)?;
    let json_journal = journal::json_journal::new(&stash_path)?;
    let mut squirrel = squirrel::new(&stash_path, json_journal)?;

    let path_filter = path_filter::new(&watched_dir, &stash_path)?;

    loop {
        let event = change_event_rx.recv()?;
        let event = event::FileEvent::from(event);

        let should_fire = {
            let p = event.path();
            p.map(|p| path_filter.allow(p).unwrap_or(true)).unwrap_or(true)
        };
        if should_fire {
            squirrel.dispatch_event(event)?;
        }

    }
}