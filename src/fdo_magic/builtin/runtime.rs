//! Enable loading the magic database files at runtime rather than embedding the GPLed database

use std::fs::{read, read_to_string};
use std::path::PathBuf;

use fnv::FnvHashMap;
use once_cell::sync::OnceCell;
use petgraph::prelude::DiGraph;

use super::MagicRule;
use crate::fdo_magic::ruleset;
use crate::Mime;

fn search_paths(filename: &str) -> Vec<PathBuf> {
    let mut search_paths = vec![
        PathBuf::from("/usr/share/mime").join(filename),
        PathBuf::from("/usr/local/share/mime").join(filename),
    ];
    if let Some(home) = home::home_dir() {
        search_paths.push(home.join(".local/share/mime").join(filename));
    }
    search_paths
}

/// Load the magic database from the predefined locations in the XDG standard
fn load_xdg_shared_magic() -> Vec<Vec<u8>> {
    search_paths("magic")
        .iter()
        .map(read)
        .filter_map(Result::ok)
        .collect()
}

/// Load a number of files at `paths` and concatenate them together with a newline
fn load_concat_strings(filename: &str) -> String {
    search_paths(filename)
        .iter()
        .map(read_to_string)
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn aliases() -> &'static str {
    static ALIAS_STRING: OnceCell<String> = OnceCell::new();
    ALIAS_STRING.get_or_init(|| load_concat_strings("aliases"))
}

pub fn subclasses() -> &'static str {
    static SUBCLASS_STRING: OnceCell<String> = OnceCell::new();
    SUBCLASS_STRING.get_or_init(|| load_concat_strings("subclasses"))
}

pub fn rules() -> Result<FnvHashMap<Mime, DiGraph<MagicRule<'static>, u32>>, String> {
    static RUNTIME_RULES: OnceCell<Vec<Vec<u8>>> = OnceCell::new();
    let files = RUNTIME_RULES.get_or_init(load_xdg_shared_magic);
    ruleset::from_multiple(files)
}
