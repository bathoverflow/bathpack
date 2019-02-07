# bathpack

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
├── bathpack.toml
└── src
    └── Project.java
```

...and the contents of `bathpack.toml` as follows...

```toml
[user]
username = "abc123"

[source]
root = "."
src = { path = "{root}/src", pattern = "*.java" }

[destination]
root = "project-{username}"
archive = true
src = "{root}"
```

> Note: This is a provisional format for `bathpack.toml`, and will likely change.

...`bathpack` will produce the following:

```
.
├── bathpack.toml
├── project-abc123
│   └── Project.java
├── project-abc123.zip
└── src
    └── Project.java
```
