use std::{collections::HashMap, env, fs, process::Command};

use anyhow::{Result, anyhow};
use semver::Version;
use toml_edit::Document;

use crate::shell;

// file names
const CARGO_TOML_FILE_NAME: &str = "Cargo.toml";
const CARGO_LOCK_FILE_NAME: &str = "Cargo.lock";

// TOML keys
const DEPENDENCIES_KEY: &str = "dependencies";
const FEATURES_KEY: &str = "features";
const NAME_KEY: &str = "name";
const PACKAGE_KEY: &str = "package";
const VERSION_KEY: &str = "version";

// Cargo CLI
const CARGO_PROGRAM: &str = "cargo";
const ADD_COMMAND: &str = "add";
const RM_COMMAND: &str = "rm";
const UPDATE_COMMAND: &str = "update";
const FEATURES_OPTION: &str = "--features";

#[derive(Default)]
struct Dependency {
    version: Option<Version>,
    features: Option<Vec<String>>,
}

macro_rules! get_toml_item_as {
    ($table:expr, $key:expr, $as_fn:ident) => {
        if let Some(toml_item) = $table.get($key) {
            toml_item.$as_fn()
        } else {
            None
        }
    };
}

macro_rules! get_toml_str {
    ($table:expr, $key:expr) => {
        get_toml_item_as!($table, $key, as_str)
    };
}

macro_rules! get_toml_array {
    ($table:expr, $key:expr) => {
        get_toml_item_as!($table, $key, as_array)
    };
}

macro_rules! get_toml_table {
    ($table:expr, $key:expr) => {
        get_toml_item_as!($table, $key, as_table)
    };
}

macro_rules! get_toml_array_of_tables {
    ($table:expr, $key:expr) => {
        get_toml_item_as!($table, $key, as_array_of_tables)
    };
}

pub fn update() -> Result<()> {
    if fs::exists(CARGO_TOML_FILE_NAME)? {
        shell::print_status("Running", "cargo update");
        if Command::new(CARGO_PROGRAM).arg(UPDATE_COMMAND).status()?.success() {
            shell::print_status("Reading", format!("`{}`", CARGO_TOML_FILE_NAME));
            let cargo_toml = fs::read_to_string(CARGO_TOML_FILE_NAME)?.parse::<Document<String>>()?;
            let toml_packages = get_dependencies(cargo_toml);
            
            if fs::exists(CARGO_LOCK_FILE_NAME)? {
                shell::print_status("Reading", format!("`{}`", CARGO_LOCK_FILE_NAME));
                let cargo_lock = fs::read_to_string(CARGO_LOCK_FILE_NAME)?.parse::<Document<String>>()?;
                if let Some(packages) = get_toml_array_of_tables!(cargo_lock, PACKAGE_KEY) {
                    for package in packages.iter() {
                        if let Some(name) = get_toml_str!(package, NAME_KEY)
                            && toml_packages.contains_key(name)
                            && let Some(version) = get_toml_str!(package, VERSION_KEY)
                            && let Ok(version) = Version::parse(version)
                            && let Some(dependecy) = toml_packages.get(name)
                            && let Some(dependency_version) = &dependecy.version
                            && dependency_version.cmp_precedence(&version).is_lt()
                        {
                            shell::print_status("Updating", format!("{} v{} -> v{}", name, dependency_version, version));
                            Command::new(CARGO_PROGRAM).arg(RM_COMMAND).arg(name).status()?;
                            let mut add_command = Command::new(CARGO_PROGRAM);
                            add_command.arg(ADD_COMMAND).arg(name);
                            if let Some(features) = &dependecy.features {
                                add_command.arg(FEATURES_OPTION).arg(features.join(","));
                            }
                            add_command.status()?;
                        }
                    }
                }
            }
            Ok(())
        } else {
            Err(anyhow!("cargo update failed"))
        }
    } else {
        Err(anyhow!("could not find `{}` in `{}` directory", CARGO_TOML_FILE_NAME, env::current_dir()?.display()))
    }
}

fn get_dependencies(cargo_toml: Document<String>) -> HashMap<String, Dependency> {
    let mut toml_packages = HashMap::new();
    if let Some(dependencies) = get_toml_table!(cargo_toml, DEPENDENCIES_KEY) {
        for (key, value) in dependencies.get_values() {
            if let Some(key) = key.first() {
                let mut dependency = Dependency::default();
                if let Some(value) = value.as_inline_table() {
                    if let Some(version) = get_toml_str!(value, VERSION_KEY)
                        && let Ok(version) = Version::parse(version)
                    {
                        dependency.version = Some(version);
                    }
                    if let Some(features) = get_toml_array!(value, FEATURES_KEY) {
                        let mut dependency_features = Vec::new();
                        for feature in features {
                            if let Some(feature) = feature.as_str() {
                                dependency_features.push(feature.to_owned());
                            }
                        }
                        dependency.features = Some(dependency_features);
                    }
                } else {
                    if let Some(value) = value.as_str()
                        && let Ok(version) = Version::parse(value)
                    {
                        dependency.version = Some(version);
                    }
                }
                toml_packages.insert(key.get().to_owned(), dependency);
            }
        }
    }
    toml_packages
}
