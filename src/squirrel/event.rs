
use std::path::{PathBuf, Path};
use std::time::SystemTime;
use notify::DebouncedEvent;

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
            &FileEvent::Write(ref p) | &FileEvent::Create(ref p) | &FileEvent::Rename(ref p, _) | &FileEvent::Remove(ref p) => Some(p),
            &FileEvent::UnknownEvent => None
        }
    }
}

impl From<DebouncedEvent> for FileEvent {
    fn from(notify_event: DebouncedEvent) -> Self {
        match notify_event {
            DebouncedEvent::Write(p) => FileEvent::Write(p),
            DebouncedEvent::Create(p) => FileEvent::Create(p),
            DebouncedEvent::Rename(p1, p2) => FileEvent::Rename(p1, p2),
            DebouncedEvent::Remove(p) => FileEvent::Remove(p),
            _ => FileEvent::UnknownEvent,
        }
    }
}

pub(crate) type EventId = u64;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum EventType {
    Create,
    Remove,
    Update,
    Rename,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Event {
    event_id: EventId,
    event_type: EventType,
    timestamp: SystemTime,
    snapshot: Option<PathBuf>,
    before_path: Option<PathBuf>,
    after_path: Option<PathBuf>
}

pub(crate) fn new_event(event_id: EventId,
    event_type: EventType,
    timestamp: SystemTime,
    snapshot: Option<PathBuf>,
    after_path: Option<PathBuf>,
    before_path: Option<PathBuf>) -> Event {
        Event {
            event_id: event_id ,
            event_type: event_type ,
            timestamp: timestamp ,
            snapshot: snapshot ,
            before_path: before_path,
            after_path: after_path 
        }
}