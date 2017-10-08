
use std::result::Result as StdResult;
use std::path::{PathBuf, Path};
use std::fmt::{Display, Error as FmtError, Formatter, Result as FmtResult};

use chrono::prelude::{DateTime, Utc, TimeZone};

use errors::*;

#[derive(Debug)]
pub(crate) enum FileEvent {
    Write(PathBuf),
    Create(PathBuf),
    Rename(PathBuf, PathBuf),
    Remove(PathBuf),
    UnknownEvent,
}

impl FileEvent {
    pub(crate) fn path(&self) -> Option<&Path> {
        match self {
            &FileEvent::Write(ref p) |
            &FileEvent::Create(ref p) |
            &FileEvent::Rename(ref p, _) |
            &FileEvent::Remove(ref p) => Some(p),
            &FileEvent::UnknownEvent => None,
        }
    }
}

pub(crate) type EventId = i64;

#[derive(Debug)]
pub(crate) struct EventTime(DateTime<Utc>);

impl EventTime {
    pub(crate) fn rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }

    pub(crate) fn from_date_time<Tz>(from: DateTime<Tz>) -> EventTime
    where
        Tz: TimeZone,
    {
        let utc_datetime = from.with_timezone(&Utc);
        EventTime(utc_datetime)
    }
}

impl Display for EventTime {
    fn fmt(&self, mut f: &mut Formatter) -> FmtResult {
        self.0.to_rfc3339().fmt(&mut f)
    }
}

pub(crate) fn get_timestamp_now() -> EventTime {
    EventTime(Utc::now())
}

#[derive(Debug)]
pub(crate) enum EventType {
    Create,
    Remove,
    Update,
    Rename,
}

impl EventType {
    pub(crate) fn from_str(s: &str) -> Result<EventType> {
        if s == "Create" {
            Ok(EventType::Create)
        } else if s == "Remove" {
            Ok(EventType::Remove)
        } else if s == "Update" {
            Ok(EventType::Update)
        } else if s == "Rename" {
            Ok(EventType::Rename)
        } else {
            Err(format!("unable to convert '{}' to EventType", s).into())
        }
    }
}

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter) -> StdResult<(), FmtError> {
        match self {
            &EventType::Create => f.write_str(&"Create"),
            &EventType::Remove => f.write_str(&"Remove"),
            &EventType::Update => f.write_str(&"Update"),
            &EventType::Rename => f.write_str(&"Rename"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Event {
    pub event_id: Option<EventId>,
    pub event_type: EventType,
    pub timestamp: EventTime,
    pub snapshot: Option<PathBuf>,
    pub before_path: Option<PathBuf>,
    pub after_path: Option<PathBuf>,
}

pub(crate) fn new_event(
    event_type: EventType,
    timestamp: EventTime,
    snapshot: Option<PathBuf>,
    after_path: Option<PathBuf>,
    before_path: Option<PathBuf>,
) -> Event {
    Event {
        event_id: None,
        event_type: event_type,
        timestamp: timestamp,
        snapshot: snapshot,
        before_path: before_path,
        after_path: after_path,
    }
}
