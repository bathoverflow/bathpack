//
//  main.rs
//  bathpack
//
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

extern crate serde;
extern crate toml;

mod config;

use config::Config;

fn main() {
    let mut config_path = std::env::current_dir().expect("Unable to access the current directory");
    config_path.push("bathpack.toml");

    let config = Config::parse_file(config_path).expect("Could not read bathpack.toml");
}
