//! Enable loading the magic database files at runtime rather than embedding the GPLed database

use std::fs::File;
use std::io::Read;

use fnv::FnvHashMap;
use once_cell::sync::OnceCell;
use petgraph::prelude::DiGraph;

use super::MagicRule;
use crate::fdo_magic::ruleset;
use crate::Mime;

static RUNTIME_RULES: OnceCell<Vec<Vec<u8>>> = OnceCell::new();
static ALIAS_STRING: OnceCell<String> = OnceCell::new();
static SUBCLASS_STRING: OnceCell<String> = OnceCell::new();

/// Load the magic database from the predefined locations in the XDG standard
fn load_xdg_shared_magic() -> Result<Vec<Vec<u8>>, String> {
    const SEARCH_PATHS: &[&str; 3] = &[
        "/usr/share/mime/magic",
        "/usr/local/share/mime/magic",
        "$HOME/.local/share/mime/magic",
    ];

    let files: Vec<Vec<u8>> = SEARCH_PATHS
        .iter()
        .filter_map(|p| File::open(p).ok())
        .map(|mut f| {
            let mut buf = vec![];
            f.read_to_end(&mut buf)
                .map_err(|e| format!("Failed to read magic file bytes: {:#?}", e))?;
            Ok(buf)
        })
        .collect::<Result<_, String>>()?;

    if files.is_empty() {
        Err("No MIME magic files found in the XDG default paths".to_string())
    } else {
        Ok(files)
    }
}

/// Load a number of files at `paths` and concatenate them together with a newline
fn load_concat_strings(paths: &[&str]) -> String {
    let strings: Vec<String> = paths
        .iter()
        .filter_map(|p| File::open(p).ok())
        .map(|mut f| {
            let mut s = String::new();
            f.read_to_string(&mut s)
                .expect("Failed to read aliases from file");
            s
        })
        .collect();

    strings.join("\n")
}

/// Load the magic aliases from the XDG standard locations and concatenate them together
fn load_aliases() -> String {
    const SEARCH_PATHS: &[&str; 3] = &[
        "/usr/share/mime/aliases",
        "/usr/local/share/mime/aliases",
        "$HOME/.local/share/mime/aliases",
    ];

    load_concat_strings(SEARCH_PATHS)
}

/// Load the subclass definitions from the XDG standard locations and concatenate them together
fn load_subclasses() -> String {
    const SEARCH_PATHS: &[&str; 3] = &[
        "/usr/share/mime/subclasses",
        "/usr/local/share/mime/subclasses",
        "$HOME/.local/share/mime/subclasses",
    ];

    load_concat_strings(SEARCH_PATHS)
}

pub(crate) fn aliases() -> &'static str {
    ALIAS_STRING.get_or_init(load_aliases)
}

pub(crate) fn subclasses() -> &'static str {
    SUBCLASS_STRING.get_or_init(load_subclasses)
}

pub(crate) fn rules() -> Result<FnvHashMap<Mime, DiGraph<MagicRule<'static>, u32>>, String> {
    let files = RUNTIME_RULES.get_or_try_init(load_xdg_shared_magic)?;
    ruleset::from_multiple(files)
}
