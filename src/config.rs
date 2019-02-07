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

use serde::Deserialize;

/// Specifies source & destination locations for files, and user information.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Config {
    /// Details about the user.
    user: User,
}

/// Details about the user.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct User {
    /// The user's University of Bath username.
    username: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that a correct configuration file string succeeds in being parsed and contains correct
    /// values.
    #[test]
    fn deserialize() {
        let toml_str = r#"
            [user]
            username = "user987"
        "#;

        let decoded: Result<Config, _> = toml::from_str(toml_str);

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

        let decoded: Result<Config, _> = toml::from_str(toml_str);

        assert!(decoded.is_err());
    }
    
    /// Test that a configuration file missing the entire `[user]` table does not successfully
    /// parse.
    #[test]
    fn missing_user_table() {
        let toml_str = r#"
        "#;

        let decoded: Result<Config, _> = toml::from_str(toml_str);

        assert!(decoded.is_err());
    }
}
