# squirrel
local history for repos

![Project is an early alpha](https://img.shields.io/badge/project_readiness-alpha-red.svg)
![build status](https://img.shields.io/travis/jelford/squirrel.svg)

## The problem

When you're experimenting with a piece of code, and all of a sudden, you realize nothing compiles, your tests are shot, 
and if you'd only committed what you had half an hour ago everything would be fine - that's when `squirrel` could have
helped you.

`squirrel` watches your project's working directory, and automatically backs up your source files on each change. 
It scans your `.gitignore` files so that only changes to your source code are picked up. Backups go to a folder
named `.backup`, and a log of all changes is kept in a local `sqlite` database for later inspection.

## Usage

Wake `squirrel` up with:

```
squirrel daemon &
```

Later, when you want to look back in time, use:

```
squirrel show '*glob_that_matches*the_file_i_was_working_on.*'
```
For example:
```
jelford@clara ~/s/code-squirrel> squirrel show '*server*' | head -n 5
Id   File Name                     Timestamp                     Update Type  Snapshot
171  server.rs                     2017-10-21 20:34              Update       hLL2bfCxlbA3eHzUfIhsC-server.rs
158  server.rs                     2017-10-21 20:32              Update       1HKx22oPdE6Rc4L9wsAJu-server.rs
113  server.rs                     2017-10-21 20:14              Update       Bs1XhewR7kFf3myYJZYMv-server.rs
106  server.rs                     2017-10-21 19:33              Update       O1Tcp0JRNzpZExh9bdTfq-server.rs
```

It's pretty manual right now, but you can use that list to get back at all previous versions of your file 
(the snapshots are under `.backup`)

## Advanced

You can see the complete log of all events that have been recorded by looking in the `sqlite` database:

```
sqllitebrowser .backup/event-log.db
```

Being backed by a simple `sqlite` database means that it should be easy to create custom tooling to help
you work backups of your files.
