

use squirrel::event::*;
use errors::*;

pub(crate) mod json_journal;

pub(crate) trait Journal {
    fn journal(&mut self, event: Event) -> Result<()>;
}

