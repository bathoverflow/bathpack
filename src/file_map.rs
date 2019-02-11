//
//  file_map.rs
//  bathpack
//
//  Created on 2019-02-11 by Søren Mortensen.
//  Copyright (c) 2018 Søren Mortensen, Andrei Trandafir, Stavros Karantonis.
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

use crate::config::{Config, Source};

use std::path::PathBuf;

/// Map from locations specified in a [`Config`][config] to locations on disk.
///
/// [config]: ../config/struct.Config.html
pub struct FileMap<'a> {
    /// The user's configuration.
    config: &'a Config,
    /// The root directory of the project.
    root_dir: PathBuf,
    /// Paths to every file specified by the config, with all glob patterns expanded.
    paths: Vec<PathBuf>,
}

impl<'a> FileMap<'a> {
    /// Create a `FileMap` with a reference to the [`Config`][config] that it should be built from.
    ///
    /// [config]: ../config/struct.Config.html
    pub fn new(config: &'a Config, root_dir: PathBuf) -> Self {
        FileMap {
            config,
            root_dir,
            paths: Vec::new(),
        }
    }

    pub fn build(&mut self) {
        for source in self.config.sources() {
            self.paths.append(&mut self.expand_source(source));
        }
    }

    fn expand_source(&self, source: &Source) -> Vec<PathBuf> {
        match source {
            Source::File(ref name) => {
                // Clone the root directory path.
                let mut path = self.root_dir.clone();
                // Push the filename to that path.
                path.push(name);

                // Return the path inside a Vec.
                vec![path]
            },
            Source::Folder {
                ref path,
                ref pattern,
            } => {
                // Clone the root directory path.
                let mut pattern_path = self.root_dir.clone();
                // Push the folder's path.
                pattern_path.push(path);
                // Push the pattern that matches files inside that folder.
                pattern_path.push(pattern);

                // Turn the path into a string, so it can be interpreted as a pattern by the glob crate.
                // TODO: Replace unwrap() calls with safe error handling
                let pattern = pattern_path.to_str().unwrap();

                // Get all matches of that pattern, and return them.
                // TODO: Replace unwrap() calls with safe error handling
                glob::glob(pattern).unwrap().map(|each| each.unwrap()).collect()
            }
        }
    }

    pub fn into_paths(self) -> std::vec::IntoIter<PathBuf> {
        self.paths.into_iter()
    }
}
