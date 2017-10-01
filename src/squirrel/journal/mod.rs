

use squirrel::event::*;
use errors::*;

pub(crate) mod sqlite_journal;

pub(crate) trait Journal {
    fn journal(&mut self, event: Event) -> Result<()>;
}

pub(crate) trait PagedJournalQuery {
    type ResultIterator: Iterator<Item=Result<Event>>;

    fn next_page(&mut self) -> Result<Self::ResultIterator>;
}

pub (crate) trait JournalReader<'a> {
    type BackwardsIterator : PagedJournalQuery;

    fn backwards(&'a self) -> Result<Self::BackwardsIterator>;
}
