// #![feature(conservative_impl_trait)]

extern crate chrono;
#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
extern crate glob;
extern crate futures;
extern crate ignore;
#[macro_use]
extern crate log;
extern crate notify;
extern crate rand;
extern crate rusqlite;

use std::path::PathBuf;

mod errors;
use errors::*;
mod squirrel;

mod path_filter;

quick_main!(run);

fn run() -> Result<()> {
    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "James Elford <james.p.elford@gmail.com>")
        (about: "Keep track of source files while you work")
        (@subcommand daemon =>
            (about: "run the daemon to monitor a directory")
        )
        (@subcommand show =>
            (about: "show revisions to files matching GLOB")
            (@arg GLOB: +required "The glob to match against")
        )
    ).get_matches();

    env_logger::init()?;

    let watched_dir = PathBuf::from(".").canonicalize().expect(
        "Unable to determine the path to the current directory",
    );
    let stash_path = watched_dir.join(".backup");

    if let Some(_) = matches.subcommand_matches("daemon") {
        return squirrel::run_squirrel(&watched_dir, &stash_path);
    }

    if let Some(matches) = matches.subcommand_matches("show") {
        let glob = matches.value_of("GLOB").unwrap();
        let glob = glob::Pattern::new(&glob)?;
        return squirrel::list_snapshots(&stash_path, glob);
    }

    Ok(())

}
