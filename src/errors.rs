
error_chain! {
    errors {
        EventJournallingError(detail: String) {
            description("unable to journal an event")
            display("there was a problem recoring an event in the journal: {}", detail)
        }
    }
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        MpscRecv(::std::sync::mpsc::RecvError);
        StripPathPrefix(::std::path::StripPrefixError);
        GitIgnoreError(::ignore::Error);
    }
}