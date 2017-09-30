
#[macro_use]
extern crate error_chain;
extern crate notify;
extern crate futures;
extern crate ignore;
extern crate ring;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate tempdir;

use std::path::PathBuf;

mod errors; 
use errors::*;
mod squirrel;

mod path_filter;

quick_main!(run);

fn run() -> Result<()> {

    let watched_dir = PathBuf::from(".").canonicalize().expect("Unable to determine the path to the current directory");
    let stash_path = watched_dir.join(".backup");

    squirrel::run_squirrel(&watched_dir, &stash_path)
}
