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

use super::Config;

pub struct Validator<'a> {
    config: &'a Config,
}

impl<'a> Validator<'a> {
    pub fn from(config: &'a Config) -> Validator<'a> {
        Validator { config }
    }

    pub fn validate(self) -> Result<(), String> {
        unimplemented!()
    }
}
