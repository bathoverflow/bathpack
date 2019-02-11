//
//  config.rs
//  bathpack
//
//  Created on 2019-02-07 by Søren Mortensen.
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

//! Parsing and structure of `bathpack.toml` configuration file.

use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

/// Read and return the user's configuration file from the default location, printing an error and exiting on failure.
pub fn read_config() -> Config {
    let mut config_file = match std::env::current_dir() {
        Ok(mut path) => {
            path.push("bathpack.toml");
            path
        },
        Err(e) => {
            eprintln!("Could not access current directory: {}", e);
            exit(1);
        },
    };

    match Config::parse_file(config_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not read bathpack.toml: {}", e);
            exit(1);
        }
    }
}

/// Specifies source & destination locations for files, and user information.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// The user's University of Bath username.
    username: String,
    /// Key-value pairs, where the key is the name of the source, and the value is the location (file or folder).
    sources: BTreeMap<String, Source>,
    /// The destination for all files, including a list of locations.
    destination: Destination,
}

impl Config {
    /// Attempt to parse a `Config` from a string containing some TOML data.
    pub fn parse<T>(toml_str: T) -> Result<Config>
    where
        T: AsRef<str>,
    {
        toml::from_str(toml_str.as_ref()).map_err(|e| e.into())
    }

    /// Attempt to parse a `Config` from a file containing TOML data at the location `path`.
    pub fn parse_file<P>(path: P) -> Result<Config>
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Config::parse(contents)
    }
}

/// A source location - either a folder or a file.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Source {
    /// A folder, interpreted as all files in that folder matching the given glob pattern. The folder location is
    /// represented as a relative path to the folder in a string.
    Folder { path: String, pattern: String },
    /// A file, stored as a relative path in a string.
    File(String),
}

/// The final destination of a Bathpack run, including the name and a list of destination locations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Destination {
    /// The name of the final folder/archive.
    name: String,
    /// Whether to archive the folder.
    archive: bool,
    /// Key-value pairs, where each key is the name of a source in a [`Config`][config], and each value is the location
    /// to move that source to.
    ///
    /// [config]: ./struct.Config.html
    locations: BTreeMap<String, DestLoc>,
}

/// A destination location.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DestLoc {
    /// A folder, stored as a relative path in a string.
    Folder(String),
}

/// Convenience alias for functions that return [`Error`][error]s.
///
/// [error]: ./enum.Error.html
pub type Result<T> = std::result::Result<T, Error>;

/// Errors to do with [`Config`][config] reading and parsing.
///
/// [config]: ./struct.Config.html
#[derive(Debug)]
pub enum Error {
    /// Wraps a [`toml::de::Error`][tomlerr].
    ///
    /// [tomlerr]: ../../toml/de/struct.Error.html
    TomlError(toml::de::Error),
    /// Wraps a [`std::io::Error`][ioerr].
    ///
    /// [ioerr]: https://doc.rust-lang.org/std/io/struct.Error.html
    IoError(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::TomlError(ref toml_err) => write!(f, "{}", toml_err),
            Error::IoError(ref io_err) => write!(f, "{}", io_err),
        }
    }
}

impl std::error::Error for Error {}

impl From<toml::de::Error> for Error {
    fn from(toml_error: toml::de::Error) -> Self {
        Error::TomlError(toml_error)
    }
}

impl From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        Error::IoError(io_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that a correct configuration file string succeeds in being parsed and contains correct
    /// values.
    #[test]
    fn parse_str() {
        let toml_str = r#"
            username = "user987"

            [sources]
            test-folder = { path = "test_path", pattern = "test_pattern" }
            test-file = "test_file_name"
            
            [destination]
            name = "test-{username}"
            archive = true

            [destination.locations]
            test-folder = "."
            test-file = "test-new-folder/subfolder"
        "#;

        let decoded: Result<Config> = Config::parse(toml_str);
        assert!(decoded.is_ok());

        let config = decoded.unwrap();
        assert_eq!(config.username, "user987".to_string());
    }

    /// Test that a configuration file with no value for `username` does not successfully
    /// parse.
    #[test]
    fn missing_username() {
        let toml_str = r#"
            [sources]
            test-folder = { path = "test_path", pattern = "test_pattern" }
            test-file = "test_file_name"
            
            [destination]
            name = "test-{username}"
            archive = true

            [destination.locations]
            test-folder = "."
            test-file = "test-new-folder/subfolder"
        "#;

        let decoded: Result<Config> = Config::parse(toml_str);
        assert!(decoded.is_err());
    }

    /// Test that a configuration file with no `sources` table does not successfully parse.
    #[test]
    fn missing_sources() {
        let toml_str = r#"
            username = "user987"

            [destination]
            name = "test-{username}"
            archive = true

            [destination.locations]
            test-folder = "."
            test-file = "test-new-folder/subfolder"
        "#;

        let decoded: Result<Config> = Config::parse(toml_str);
        assert!(decoded.is_err());
    }

    /// Test that a configuration file with an empty `sources` table successfully parses.
    #[test]
    fn empty_sources() {
        let toml_str = r#"
            username = "user987"
            
            [sources]
            
            [destination]
            name = "test-{username}"
            archive = true
            
            [destination.locations]
            test-folder = "."
            test-file = "test-new-folder/subfolder"
        "#;

        let decoded: Result<Config> = Config::parse(toml_str);
        assert!(decoded.is_ok());

        let config = decoded.unwrap();
        assert!(config.sources.is_empty());
    }

    /// Test that a configuration file with an empty `destination` table does not successfully
    /// parse.
    #[test]
    fn empty_destination() {
        let toml_str = r#"
            username = "user987"
            
            [sources]
            test-folder = { path = "test_path", pattern = "test_pattern" }
            test-file = "test_file_name"
            
            [destination]
        "#;

        let decoded: Result<Config> = Config::parse(toml_str);
        assert!(decoded.is_err());
    }

    /// Test that a configuration file with an empty `destination` table, apart from
    /// `destination.locations`, does not successfully parse.
    #[test]
    fn empty_destination_with_locations() {
        let toml_str = r#"
            username = "user987"
            
            [sources]
            test-folder = { path = "test_path", pattern = "test_pattern" }
            test-file = "test_file_name"
            
            [destination]

            [destination.locations]
            test-folder = "."
            test-file = "test-new-folder/subfolder"
        "#;

        let decoded: Result<Config> = Config::parse(toml_str);
        assert!(decoded.is_err());
    }

    /// Test that a configuration file with no `destination.locations` table does not successfully
    /// parse.
    #[test]
    fn missing_destination_locations() {
        let toml_str = r#"
            username = "user987"
            
            [sources]
            test-folder = { path = "test_path", pattern = "test_pattern" }
            test-file = "test_file_name"
            
            [destination]
            name = "test-{username}"
            archive = true
        "#;

        let decoded: Result<Config> = Config::parse(toml_str);
        assert!(decoded.is_err());
    }

    /// Test that a configuration file with an empty `destination.locations` table successfully
    /// parses.
    #[test]
    fn empty_destination_locations() {
        let toml_str = r#"
            username = "user987"
            
            [sources]
            test-folder = { path = "test_path", pattern = "test_pattern" }
            test-file = "test_file_name"
            
            [destination]
            name = "test-{username}"
            archive = true
            
            [destination.locations]
        "#;

        let decoded: Result<Config> = Config::parse(toml_str);
        assert!(decoded.is_ok());

        let config = decoded.unwrap();
        assert!(config.destination.locations.is_empty());
    }
}
