//
//  file_map/mod.rs
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

use crate::config::Config;

use failure::{Error, Fail};
use glob::{GlobError, PatternError};

use std::collections::BTreeMap;
use std::path::PathBuf;

pub struct FileMap {
    src_path: PathBuf,
    dest_path: PathBuf,
    archive: bool,
    map: BTreeMap<PathBuf, PathBuf>,
}

#[derive(Debug)]
pub struct FileMapBuilder {
    config: Config,
}

impl FileMapBuilder {
    pub fn from(config: Config) -> Self {
        FileMapBuilder { config }
    }

    pub fn build(self) -> Result<FileMap, Error> {
        println!("{:#?}", self);

        self.expand_locations()?
            .pair_locations()?
            .verify_existence()
    }

    fn expand_locations(self) -> Result<ExpandedLocations, Error> {
        let sources = unimplemented!();
        let destinations = unimplemented!();

        Ok(ExpandedLocations {
            config: self.config,
            sources,
            destinations,
        })
    }
}

struct ExpandedLocations {
    config: Config,
    sources: BTreeMap<String, PathBuf>,
    destinations: BTreeMap<String, PathBuf>,
}

impl ExpandedLocations {
    fn pair_locations(self) -> Result<PairedLocations, Error> {
        let pairs = unimplemented!();

        Ok(PairedLocations {
            config: self.config,
            pairs,
        })
    }
}

struct PairedLocations {
    config: Config,
    pairs: BTreeMap<PathBuf, PathBuf>,
}

impl PairedLocations {
    fn verify_existence(self) -> Result<FileMap, Error> {
        unimplemented!()
    }
}

#[derive(Debug, Fail)]
pub enum FileMapError {
    #[fail(display = "invalid pattern format: {}", err)]
    Pattern { err: PatternError },
    #[fail(display = "error while matching glob pattern: {}", err)]
    Glob { err: GlobError },
    #[fail(display = "no matches for glob pattern: {}", pattern)]
    NoMatches { pattern: String },
    #[fail(
        display = "source location `{}` specified in [destination.locations] is missing",
        key
    )]
    MissingSource { key: String },
    #[fail(
        display = "destination location `{}` specified in [sources] is missing",
        key
    )]
    MissingDest { key: String },
}
