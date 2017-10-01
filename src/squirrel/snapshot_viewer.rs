use std::path::Path;
use glob::{Pattern};

use super::event::Event;
use super::journal::{JournalReader, sqlite_journal, PagedJournalQuery};

use errors::*;

pub fn list_snapshots(stash_path: &Path, glob: Pattern) -> Result<()> {
    let viewer = SnapshotViewer {
        journal: sqlite_journal::new(&stash_path)?,
        glob: glob,
    };

    viewer.show_relevant_snapshots()?;
    Ok(())
}

struct SnapshotViewer<J> {
    journal: J,
    glob: Pattern,
}

fn match_name(glob: &Pattern, event: &Event) -> Option<String> {
    let a_path = match (&event.after_path, &event.before_path) {
        (&Some(ref p), _) if glob.matches_path(&p) => Some(p),
        (&None, &Some(ref p)) if glob.matches_path(&p) => Some(p),
        _ => None,
    };

    a_path.map(|p| {
        p.file_name().map(|p| String::from(p.to_string_lossy())).unwrap_or(String::from("<unknown>"))
    })
}

impl <'a, J> SnapshotViewer<J> where J : JournalReader<'a> {
    fn show_relevant_snapshots(&'a self) -> Result<()> {
        let g = &self.glob;
        println!("Id\tFile Name\tTimestamp\tUpdate Type");

        let mut back = self.journal.backwards()?;
        for event in back.next_page()? {
            let event = event?;
            if let Some(matched_name) = match_name(&g, &event) {
                let timestamp = event.timestamp;
                println!("{}\t{}\t{}\t{}\t", 
                    event.event_id.unwrap(), 
                    matched_name, 
                    &timestamp, 
                    event.event_type);
            }
        }
        Ok(())
    }
}