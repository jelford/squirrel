
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;

use chrono::DateTime;
use rusqlite::{Connection, Row, Statement, Result as RusqlResult, Error as RusqlError};
use rusqlite::types::{FromSql, ValueRef, FromSqlResult, FromSqlError};

use errors::*;
use squirrel::event::*;

pub(crate) struct SqliteJournal {
    db_connection: Connection,
}

pub(crate) fn new(stash_path: &Path) -> Result<SqliteJournal> {
    let event_log_path = stash_path.join("event-log.db").to_owned();

    let connection = Connection::open(event_log_path)?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS Events (
        event_id INTEGER PRIMARY KEY,
        event_type TEXT,
        timestamp TEXT,
        snapshot TEXT,
        before_path TEXT,
        after_path TEXT
        )",
        &[],
    )?;

    Ok(SqliteJournal { db_connection: connection })
}

impl<'a> super::Journal for SqliteJournal {
    fn journal(&mut self, event: Event) -> Result<()> {
        let event_type = format!("{}", event.event_type);
        let timestamp = format!("{}", event.timestamp.rfc3339());
        let snapshot = event.snapshot.map(|p| format!("{}", p.to_string_lossy()));
        let before_path = event.before_path.map(
            |p| format!("{}", p.to_string_lossy()),
        );
        let after_path = event.after_path.map(|p| format!("{}", p.to_string_lossy()));

        use std::os::raw::c_int;
        let _: Result<c_int> = self.db_connection.execute(
                    &"INSERT INTO Events (event_type, timestamp, snapshot, before_path, after_path) VALUES (?, ?, ?, ?, ?)", 
                    &[&event_type, &timestamp, &snapshot, &before_path, &after_path])
                    .map_err(|e: RusqlError| {
                        let provider_msg = format!("{}", e);
                        ErrorKind::EventJournallingError(format!("Problem writing to database: {}", provider_msg)).into()
                    });


        Ok(())
    }
}



impl FromSql for EventType {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(ref v) => {
                match EventType::from_str(v.to_owned()) {
                    Ok(e) => Ok(e),
                    Err(_) => Err(FromSqlError::InvalidType),
                }
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

fn row_to_event(row: &Row) -> Event {
    let after_path: Option<String> = row.get(5);
    let snapshot: Option<String> = row.get(4);
    let before_path: Option<String> = row.get(3);
    let timestamp: String = row.get(2);
    let timestamp = DateTime::parse_from_rfc3339(&timestamp).unwrap();

    Event {
        event_id: Some(row.get(0)),
        event_type: row.get(1),
        timestamp: EventTime::from_date_time(timestamp),
        after_path: after_path.map(|s| PathBuf::from(s)),
        before_path: snapshot.map(|s| PathBuf::from(s)),
        snapshot: before_path.map(|s| PathBuf::from(s)),
    }
}

pub(crate) struct QmErrMapper<T>
where
    T: Iterator<Item = StdResult<Event, RusqlError>>,
{
    qm: T,
}

impl<'a, T> Iterator for QmErrMapper<T>
where
    T: Iterator<Item = StdResult<Event, RusqlError>>,
{
    type Item = Result<Event>;

    fn next(&mut self) -> Option<Result<Event>> {
        self.qm.next().map(|r| r.map_err(|e| e.into()))
    }
}

impl<'a> super::PagedJournalQuery for Statement<'a> {
    type ResultIterator = QmErrMapper<::std::vec::IntoIter<StdResult<Event, RusqlError>>>;

    fn next_page(&mut self) -> Result<Self::ResultIterator> {
        let mapper: fn(&Row) -> Event = row_to_event;
        let qm: Vec<RusqlResult<Event>> = self.query_map(&[], mapper)?.collect();
        Ok(QmErrMapper { qm: qm.into_iter() })
    }
}

impl<'a> super::JournalReader<'a> for SqliteJournal {
    type BackwardsIterator = Statement<'a>;
    fn backwards(&'a self) -> Result<Self::BackwardsIterator> {
        let stmt = self.db_connection.prepare(
            "SELECT 
                    event_id, 
                    event_type, 
                    timestamp, 
                    snapshot, 
                    before_path, 
                    after_path 
                FROM 
                    Events 
                ORDER BY 
                    timestamp DESC",
        )?;
        Ok(stmt)
    }
}
