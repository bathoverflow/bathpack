//
//  main.rs
//  bathpack
//
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

//! Bathpack is a tool for automating the packaging of coursework files for submission at the
//! University of Bath, specifically for the BSc/MComp Computer Science degree.
//!
//! Bathpack works by reading a configuration file in TOML format, called `bathpack.toml` by
//! default, describing the locations of source files and destination locations, as well as
//! details about the final folder/archive.
//!
//! Optionally, information about the destination can be specified separately, such as in another
//! TOML file alongside `bathpack.toml` or inside/alongside Bathpack. This way, configurations
//! for specific coursework submissions can be distributed to multiple users.

#![allow(dead_code)]

extern crate failure;
extern crate glob;
extern crate serde;
extern crate strfmt;
extern crate toml;

mod config;
mod file_map;

use failure::Error;

use config::read_config;
use file_map::FileMapBuilder;

use std::process::exit;

/// Reads in a configuration file.
fn main() {
    let result = run();

    if let Err(err) = result {
        eprintln!("{}", err);
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;
    let config = read_config(&current_dir)?;

    let file_map = FileMapBuilder::from(config, current_dir).build()?;
    println!("{:#?}", file_map);

    Ok(())
}
