//
//  validate.rs
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

//! Validation of internal consistency of the user's configuration.
//!
//! This includes:
//! - Keys in `sources` have matching keys in `destination.locations`.
//! - Keys in `destination.locations` have matching keys in `sources`.
//! - Format of `destination.name` is valid.

use strfmt::{FmtError, strfmt};

use super::Config;

use std::collections::HashMap;

/// Validates the contents of a `Config`.
pub struct Validator<'a> {
    config: &'a Config,
}

impl<'a> Validator<'a> {
    pub fn from(config: &'a Config) -> Validator<'a> {
        Validator { config }
    }

    pub fn validate(self) -> Result<(), String> {
        // Check that every source in [sources] has a matching location in [destination.locations].
        for source_key in self.config.sources.keys() {
            if !self.config.destination.locations.contains_key(source_key) {
                return Err(format!(
                    "Key `{}` from [sources] does not exist in [destination.locations]",
                    source_key
                ));
            }
        }

        // Check that every source in [destination.locations] has a matching location in [sources].
        for dest_key in self.config.destination.locations.keys() {
            if !self.config.sources.contains_key(dest_key) {
                return Err(format!(
                    "Key `{}` from [destination.locations] does not exist in [sources]",
                    dest_key
                ));
            }
        }

        // Check that the format of destination.name is correct.
        let mut vars = HashMap::new();
        vars.insert("username".to_string(), self.config.username.clone());

        // Try formatting it, and if we get an error, return a message describing the problem.
        if let Err(fmt_err) = strfmt(&self.config.destination.name, &vars) {
            return Err(match fmt_err {
                FmtError::Invalid(msg) => format!("Value of `destination.name` contains invalid format: {}", msg),
                FmtError::KeyError(msg) => format!("Key in `destination.name` format does not exist: {}", msg),
                FmtError::TypeError(msg) => format!("Type incorrect: {}", msg),
            })
        }

        // No problems found.
        Ok(())
    }
}
