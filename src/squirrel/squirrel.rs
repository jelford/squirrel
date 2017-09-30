
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::fs::{self};
use std::time::SystemTime;

use super::event::*;
use super::journal;
use errors;


pub(crate) fn new<'a, Journal>(stash_path: &Path, journal: Journal) -> errors::Result<Squirrel<Journal>> 
    where Journal: journal::Journal {
    Ok(Squirrel {
        stash_path: stash_path.to_owned(),
        next_event_id: 0,
        journal: journal,
    })
}

pub(crate) struct Squirrel<Journal> where Journal: super::journal::Journal  {
    stash_path: PathBuf,
    next_event_id: EventId,
    journal: Journal,
}


impl <Journal> Squirrel<Journal> where Journal: super::journal::Journal {

    fn next_id(&mut self) -> EventId {
        let res = self.next_event_id;
        self.next_event_id += 1;
        res
    }

    fn journal(&mut self, event: Event) -> errors::Result<()> {
        self.journal.journal(event)?;
        Ok(())
    }

    fn save_snapshot(&self, event_id: &EventId, source_file: &Path) -> errors::Result<PathBuf> {
        let mut file_name = OsString::from(format!("{}-", event_id));
        file_name.push(source_file.file_name().unwrap());
        let stashed_file_name = self.stash_path.join(&file_name);
        fs::copy(source_file, &stashed_file_name)?;
        Ok(PathBuf::from(&file_name))
    } 
    
    fn on_write(&mut self, path: &Path) -> errors::Result<()> {
        self.record_write_or_create(&path, EventType::Update)
    }

    fn record_write_or_create(&mut self, path: &Path, event_type: EventType) -> errors::Result<()> {
        if path.is_dir() {
            return Ok(());
        }

        let event_id = self.next_id();
        let snapshot_path = self.save_snapshot(&event_id, &path)?;
        
        self.journal(
            new_event(
                event_id, 
                event_type,
                SystemTime::now(), 
                Some(snapshot_path), 
                None, 
                Some(path.to_owned())))?;

        Ok(())
    }

    fn on_create(&mut self, path: &Path) -> errors::Result<()> {
        self.record_write_or_create(path, EventType::Create)
    }

    fn on_remove(&mut self, path: &Path) -> errors::Result<()> {
        let event_id = self.next_id();

        self.journal(
            new_event(
                event_id,
                EventType::Remove,
                SystemTime::now(),
                None,
                None,
                Some(path.to_owned())
            )
        )?;
        Ok(())
    }

    fn on_rename(&mut self, source: &Path, destination: &Path) -> errors::Result<()> {

        let event_id = self.next_id();
        let snapshot_path = self.save_snapshot(&event_id, &destination)?;

        self.journal(
            new_event(
                event_id,
                EventType::Rename,
                SystemTime::now(),
                Some(snapshot_path),
                Some(destination.to_owned()),
                Some(source.to_owned())
            )
        )?;

        Ok(())
    }

    pub(crate) fn dispatch_event<E>(&mut self, event: E) -> errors::Result<()>
        where FileEvent: From<E> {

        let event = FileEvent::from(event);
        println!("<<< received : {:?}", event);


        match event {
            FileEvent::Write(ref path) => self.on_write(path)?,
            FileEvent::Create(ref path) => self.on_create(path)?,
            FileEvent::Remove(ref path) => self.on_remove(path)?,
            FileEvent::Rename(ref source, ref dest) => self.on_rename(source, dest)?,
            FileEvent::UnknownEvent => (),
        };

        Ok(())
    }
}