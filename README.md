# bathpack

[![Build Status]][builds.sr.ht]

[Build Status]: https://builds.sr.ht/~nerosnm/bathpack.svg
[builds.sr.ht]: https://builds.sr.ht/~nerosnm/bathpack

`bathpack` is a tool for packaging files for coursework submission at the University of Bath.

## Project Overview

The core functionality of `bathpack` entails:

- Reading a configuration file (`bathpack.toml`) that specifies source locations for files and
  folders in a project, as well as details about the user.
- Reading information (either directly from `bathpack.toml` or from another file specified within
  `bathpack.toml`) about the destinations for those files and folders.
- Copying the specified files and folders to their destinations, in a strictly non-destructive way.
- Packaging the copied files into an archive, and naming that archive according to the user's
  details.

The end result of all of this functionality is that, given a project folder containing the
following...

```
.
├── README.md
├── bathpack.toml
└── src
    └── Project.java
```

...and the contents of `bathpack.toml` as follows...

```toml
username = "abc123"

[sources]
src = { path = "src", pattern = "*.java" }
readme = "README.md"

[destination]
name = "project-{username}"
archive = true

[destination.locations]
src = "."
readme = "."
```

...`bathpack` will produce the following:

```
.
├── README.md
├── bathpack.toml
├── project-abc123
│   ├── Project.java
│   └── README.md
├── project-abc123.zip
└── src
    └── Project.java
```
