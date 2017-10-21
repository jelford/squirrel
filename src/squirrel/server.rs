
use std::io;
use std::fs;
use std::time::Duration;
use std::sync::mpsc::channel as sync_channel;
use std::path::{Path, PathBuf};

use notify::{watcher, RecursiveMode, Watcher, DebouncedEvent};

use errors::*;
use path_filter;

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
    ensure_dir(&stash_path)?;
    let json_journal = journal::sqlite_journal::new(&stash_path)?;
    let mut squirrel = squirrel::new(&stash_path, json_journal)?;

    let path_filter = path_filter::new(&watched_dir, &stash_path)?;

    let (change_event_tx, change_event_rx) = sync_channel();

    let mut watcher = watcher(change_event_tx, Duration::from_secs(1)).unwrap();
    watcher
        .watch(&watched_dir, RecursiveMode::Recursive)
        .unwrap();

    for top_level in fs::read_dir(watched_dir)? {
        let top_level = top_level?;

        let watched = top_level.path();
        if watched.is_dir() && !path_filter.allow(&watched)? {
            watcher.unwatch(&watched).expect(&format!(
                "Unable to unwatch {:?}",
                watched
            ));
        }
    }

    loop {
        let e = change_event_rx.recv()?;
        let event = to_squirrel_event(e, &watched_dir)?;
        let should_fire = {
            let p = event.path();
            match p {
                Some(p) => path_filter.allow(p)?,
                _ => true,
            }
        };
        if should_fire {
            squirrel.dispatch_event(event)?;
        }
    }
}

fn relativize<'a>(base_path: &'a Path, abs_path: &'a Path) -> Result<PathBuf> {
    abs_path
        .strip_prefix(base_path)
        .chain_err(|| "o shit")
        .map(|p| p.to_owned())
}

fn to_squirrel_event(notify_event: DebouncedEvent, base_path: &Path) -> Result<event::FileEvent> {

    Ok(match notify_event {
        DebouncedEvent::Write(p) => event::FileEvent::Write(relativize(&base_path, &p)?),
        DebouncedEvent::Create(p) => event::FileEvent::Create(relativize(&base_path, &p)?),
        DebouncedEvent::Rename(p1, p2) => {
            event::FileEvent::Rename(relativize(&base_path, &p1)?, relativize(&base_path, &p2)?)
        }
        DebouncedEvent::Remove(p) => event::FileEvent::Remove(relativize(&base_path, &p)?),
        x => {
            trace!("Received unhandled event from notify layer: {:?}", x);
            event::FileEvent::UnknownEvent
        }
    })
}
