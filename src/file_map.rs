//
//  file_map.rs
//  bathpack
//
//  Created on 2019-02-12 by Søren Mortensen.
//  Copyright (c) 2019 Søren Mortensen, Andrei Trandafir, Stavros Karantonis.
//
//  Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
//  in compliance with the License.  You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software distributed under the
//  License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
//  express or implied.  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::config::{Config, DestLoc, Source};

use failure::{Error, Fail};
use glob::{GlobError, PatternError};
use strfmt::FmtError as StrFmtError;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::PathBuf;

macro_rules! path {
    ($base:expr) => {
        {
            let mut path = ::std::path::PathBuf::new();
            path.push(&$base);
            path
        }
    };
    ($base:expr, $($x:expr),*) => {
        {
            let mut path = $base.clone();
            $(
                path.push(&$x);
            )*
            path
        }
    };
}

#[derive(Debug)]
pub struct FileMap {
    root_dir: PathBuf,
    dest_dir: PathBuf,
    archive: bool,
    map: Vec<(PathBuf, PathBuf)>,
}

#[derive(Debug)]
pub struct FileMapBuilder {
    root_dir: PathBuf,
    config: Config,
}

impl FileMapBuilder {
    pub fn from(config: Config, root_dir: PathBuf) -> Self {
        FileMapBuilder { config, root_dir }
    }

    pub fn build(self) -> Result<FileMap, Error> {
        self.format_destination()?
            .expand_paths()?
            .pair_locations()?
            .flatten_locations()?
            .verify_scope()?
            .verify_existence()
    }

    fn format_destination(self) -> Result<DestFormatted, Error> {
        use strfmt::strfmt;

        let root_dir = self.root_dir;

        let username = self.config.username;
        let dest_pat = self.config.destination.name;

        let mut vars = HashMap::new();
        vars.insert("username".to_string(), username);

        let dest = strfmt(&dest_pat, &vars).map_err(FileMapError::FormatError)?;
        let dest_dir = path!(root_dir, dest);

        let archive = self.config.destination.archive;

        Ok(DestFormatted {
            root_dir,
            dest_dir,
            archive,
            sources: self.config.sources,
            dests: self.config.destination.locations,
        })
    }
}

struct DestFormatted {
    root_dir: PathBuf,
    dest_dir: PathBuf,
    archive: bool,
    sources: BTreeMap<String, Source>,
    dests: BTreeMap<String, DestLoc>,
}

impl DestFormatted {
    fn expand_paths(self) -> Result<PathsExpanded, Error> {
        let root_dir = self.root_dir;
        let dest_dir = self.dest_dir;
        let archive = self.archive;

        let sources = Self::expand_sources(self.sources, &root_dir)?;
        let destinations = Self::expand_dests(self.dests, &path!(root_dir, dest_dir))?;

        Ok(PathsExpanded {
            root_dir,
            dest_dir,
            archive,
            sources,
            dests: destinations,
        })
    }

    fn expand_sources(
        sources: BTreeMap<String, Source>,
        root_dir: &PathBuf,
    ) -> Result<BTreeMap<String, ExpandedSource>, Error> {
        let mut expanded_sources = BTreeMap::new();

        for (key, source) in sources {
            let expanded: ExpandedSource = match source {
                Source::Folder {
                    path: raw_path,
                    pattern,
                } => {
                    // We need paths to both the base of the directory that is this source, and
                    // also one including the file glob pattern we'll match on later. The base
                    // path is needed so we can preserve subdirectories when copying while still
                    // filtering based on the glob pattern.
                    let base_path = path!(root_dir, raw_path);
                    let path = path!(base_path, pattern.as_str());

                    // Convert the pattern path to a String.
                    let path_string = path.to_str().expect("path was invalid Unicode").to_owned();

                    // Glob search using the constructed path/pattern, splitting the results into
                    // successful matches and errors.
                    let (matches, errors): (Vec<_>, Vec<_>) = glob::glob(&path_string)
                        .map_err(|err| FileMapError::Pattern { err })?
                        .partition(Result::is_ok);

                    if !errors.is_empty() {
                        // If we found any errors while accessing individual paths, collect all the
                        // error values...
                        let errors = errors
                            .into_iter()
                            .map(Result::unwrap_err)
                            .collect::<Vec<_>>();
                        // ...and return them.
                        return Err(FileMapError::from(errors).into());
                    } else {
                        // Otherwise, return the matches.
                        let paths = matches.into_iter().map(Result::unwrap).collect();

                        ExpandedSource::FileMatches {
                            base: base_path,
                            items: paths,
                        }
                    }
                }
                Source::File(raw_path) => {
                    let item = path!(root_dir, raw_path).canonicalize()?;
                    let base = item
                        .parent()
                        .expect("couldn't find parent folder of source file")
                        .to_path_buf();

                    ExpandedSource::File { base, item }
                }
            };

            expanded_sources.insert(key, expanded);
        }

        Ok(expanded_sources)
    }

    fn expand_dests(
        dests: BTreeMap<String, DestLoc>,
        root_dir: &PathBuf,
    ) -> Result<BTreeMap<String, ExpandedDest>, Error> {
        let mut expanded_dests = BTreeMap::new();

        for (key, dest) in dests {
            let expanded: ExpandedDest = match dest {
                DestLoc::Folder(raw_path) => ExpandedDest(path!(root_dir, raw_path)),
            };

            expanded_dests.insert(key, expanded);
        }

        Ok(expanded_dests)
    }
}

#[derive(Debug)]
struct PathsExpanded {
    root_dir: PathBuf,
    dest_dir: PathBuf,
    archive: bool,
    sources: BTreeMap<String, ExpandedSource>,
    dests: BTreeMap<String, ExpandedDest>,
}

#[derive(Clone, Debug)]
enum ExpandedSource {
    FileMatches { base: PathBuf, items: Vec<PathBuf> },
    File { base: PathBuf, item: PathBuf },
}

#[derive(Debug)]
struct ExpandedDest(PathBuf);

struct MissingSource(String);
struct MissingDest(String);

impl PathsExpanded {
    fn pair_locations(self) -> Result<LocationsPaired, Error> {
        let sources = self.sources;
        let mut dests = self.dests;

        let mut pairs = Vec::<(ExpandedSource, ExpandedDest)>::new();
        let mut missing_sources = Vec::new();
        let mut missing_dests = Vec::new();

        for (key, source) in sources {
            let _ = dests
                .remove(&key)
                .ok_or_else(|| missing_sources.push(MissingSource(key)))
                .map(|dest| pairs.push((source, dest)));
        }

        for (key, _) in dests {
            missing_dests.push(MissingDest(key));
        }

        if !missing_sources.is_empty() || !missing_dests.is_empty() {
            return Err(FileMapError::from((missing_sources, missing_dests)).into());
        }

        Ok(LocationsPaired {
            root_dir: self.root_dir,
            dest_dir: self.dest_dir,
            archive: self.archive,
            pairs,
        })
    }
}

#[derive(Debug)]
struct LocationsPaired {
    root_dir: PathBuf,
    dest_dir: PathBuf,
    archive: bool,
    pairs: Vec<(ExpandedSource, ExpandedDest)>,
}

impl LocationsPaired {
    fn flatten_locations(self) -> Result<LocationsFlattened, Error> {
        let mut flattened_pairs = Vec::new();

        for (source, dest) in self.pairs {
            match source {
                ExpandedSource::FileMatches { base, items } => {
                    for source_path in items {
                        let relative_to_base = source_path
                            .strip_prefix(&base)
                            .expect("prefix could not be stripped from source path");
                        let dest_path = path!(dest.0, relative_to_base);
                        flattened_pairs.push((source_path, dest_path));
                    }
                }
                ExpandedSource::File { base, item } => {
                    let relative_to_base = item
                        .strip_prefix(&base)
                        .expect("prefix could not be stripped from source path");
                    let dest_path = path!(dest.0, relative_to_base);
                    flattened_pairs.push((item, dest_path));
                }
            }
        }

        Ok(LocationsFlattened {
            root_dir: self.root_dir,
            dest_dir: self.dest_dir,
            archive: self.archive,
            pairs: flattened_pairs,
        })
    }
}

#[derive(Debug)]
struct LocationsFlattened {
    root_dir: PathBuf,
    dest_dir: PathBuf,
    archive: bool,
    pairs: Vec<(PathBuf, PathBuf)>,
}

impl LocationsFlattened {
    fn verify_scope(self) -> Result<ScopeVerified, Error> {
        let outside: Vec<String> = self
            .pairs
            .iter()
            .map(|(_, d)| d)
            .filter(|&p| !p.starts_with(&self.dest_dir))
            .map(|p| p.to_str().unwrap().to_owned())
            .collect();

        if !outside.is_empty() {
            Err(FileMapError::Scope(outside).into())
        } else {
            Ok(ScopeVerified {
                root_dir: self.root_dir,
                dest_dir: self.dest_dir,
                archive: self.archive,
                pairs: self.pairs,
            })
        }
    }
}

struct ScopeVerified {
    root_dir: PathBuf,
    dest_dir: PathBuf,
    archive: bool,
    pairs: Vec<(PathBuf, PathBuf)>,
}

impl ScopeVerified {
    fn verify_existence(self) -> Result<FileMap, Error> {
        let nonexistent: Vec<String> = self
            .pairs
            .iter()
            .map(|(s, _)| s)
            .filter(|p| !p.exists())
            .map(|p| p.to_str().unwrap().to_owned())
            .collect();

        if !nonexistent.is_empty() {
            Err(FileMapError::NonexistentFiles { files: nonexistent }.into())
        } else {
            Ok(FileMap {
                root_dir: self.root_dir,
                dest_dir: self.dest_dir,
                archive: self.archive,
                map: self.pairs,
            })
        }
    }
}

#[derive(Debug, Fail)]
pub enum FileMapError {
    /// The destination name could not be formatted, due to the given reason.
    FormatError(StrFmtError),
    /// The files at the paths given are outside the scope of the destination directory.
    Scope(Vec<String>),
    //    #[fail(display = "invalid pattern format: {}", err)]
    Pattern {
        err: PatternError,
    },
    //    #[fail(display = "errors while matching glob patterns: {:#?}", errs)]
    Glob {
        errs: Vec<GlobError>,
    },
    //    #[fail(display = "no matches for glob pattern: {}", pattern)]
    NoMatches {
        pattern: String,
    },
    //    #[fail(
    //        display = "sources `{:?}` specified in [destination.locations] do not exist",
    //        keys
    //    )]
    MissingSources {
        keys: Vec<String>,
    },
    //    #[fail(
    //        display = "destinations `{:?}` specified in [sources] do not exist",
    //        keys
    //    )]
    MissingDests {
        keys: Vec<String>,
    },
    //    #[fail(
    //        display = "sources `{:?}` and destinations `{:?}` do not exist",
    //        srcs, dests
    //    )]
    MissingFiles {
        srcs: Vec<String>,
        dests: Vec<String>,
    },
    //    #[fail(display = "files {:?} do not exist", files)]
    NonexistentFiles {
        files: Vec<String>,
    },
}

impl Display for FileMapError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            FileMapError::FormatError(ref fmt_err) => match *fmt_err {
                StrFmtError::Invalid(ref msg) => write!(f, "Destination format invalid: {}", msg),
                StrFmtError::KeyError(ref msg) => {
                    write!(f, "Destination format key error: {}", msg)
                }
                StrFmtError::TypeError(ref msg) => {
                    write!(f, "Destination format type error: {}", msg)
                }
            },
            FileMapError::Scope(ref paths) => write!(
                f,
                "Some destination paths were outside the destination directory:\n{}",
                paths.join("\n")
            ),
            _ => write!(f, ""),
        }
    }
}

impl From<Vec<GlobError>> for FileMapError {
    fn from(errs: Vec<GlobError>) -> Self {
        FileMapError::Glob { errs }
    }
}

impl From<Vec<MissingSource>> for FileMapError {
    fn from(keys: Vec<MissingSource>) -> Self {
        FileMapError::MissingSources {
            keys: keys.into_iter().map(|e| e.0).collect(),
        }
    }
}

impl From<Vec<MissingDest>> for FileMapError {
    fn from(keys: Vec<MissingDest>) -> Self {
        FileMapError::MissingDests {
            keys: keys.into_iter().map(|e| e.0).collect(),
        }
    }
}

impl From<(Vec<MissingSource>, Vec<MissingDest>)> for FileMapError {
    fn from((srcs, dests): (Vec<MissingSource>, Vec<MissingDest>)) -> Self {
        FileMapError::MissingFiles {
            srcs: srcs.into_iter().map(|e| e.0).collect(),
            dests: dests.into_iter().map(|e| e.0).collect(),
        }
    }
}
