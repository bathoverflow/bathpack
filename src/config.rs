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

use serde::{Deserialize, Serialize};

use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Specifies source & destination locations for files, and user information.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// Details about the user.
    pub user: User,
}

impl Config {
    /// Attempt to parse a `Config` from a string containing some TOML data.
    pub fn parse<T>(toml_str: T) -> Result<Config>
    where
        T: Into<String>,
    {
        toml::from_str(&toml_str.into()).map_err(|e| e.into())
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

/// Details about the user.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// The user's University of Bath username.
    pub username: String,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TomlError(toml::de::Error),
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
            [user]
            username = "user987"
        "#;

        let decoded: Result<Config> = Config::parse(toml_str);
        assert!(decoded.is_ok());

        let config = decoded.unwrap();
        assert_eq!(config.user.username, "user987".to_string());
    }

    /// Test that a configuration file with no value for `user.username` does not successfully
    /// parse.
    #[test]
    fn missing_username() {
        let toml_str = r#"
            [user]
        "#;

        let decoded: Result<Config> = Config::parse(toml_str);
        assert!(decoded.is_err());
    }

    /// Test that a configuration file missing the entire `[user]` table does not successfully
    /// parse.
    #[test]
    fn missing_user_table() {
        let toml_str = r#"
        "#;

        let decoded: Result<Config> = Config::parse(toml_str);
        assert!(decoded.is_err());
    }

    /// Test that a correct configuration file string succeeds in being parsed and contains correct
    /// values.
    #[test]
    fn parse_file() {
        let toml_str = r#"
            [user]
            username = "user987"
        "#;

        let decoded: Result<Config> = Config::parse(toml_str);
        assert!(decoded.is_ok());

        let config = decoded.unwrap();
        assert_eq!(config.user.username, "user987".to_string());
    }
}
