use super::ALL_RULES;
use crate::{fdo_magic::check::from_u8_walker, read_bytes, Mime};
use fnv::FnvHashMap;
use petgraph::prelude::*;
use std::path::Path;

pub(crate) struct FdoMagic;

impl crate::Checker for FdoMagic {
    fn match_bytes(&self, file: &[u8], mimetype: &str) -> bool {
        match_bytes(file, mimetype)
    }

    fn match_filepath(&self, filepath: &Path, mimetype: &str) -> bool {
        match_filepath(filepath, mimetype)
    }

    fn get_supported(&self) -> Vec<Mime> {
        super::init::get_supported()
    }

    fn get_subclasses(&self) -> Vec<(Mime, Mime)> {
        super::init::get_subclasses()
    }

    fn get_aliaslist(&self) -> FnvHashMap<Mime, Mime> {
        super::init::get_aliaslist()
    }
}

/// Test against all rules
#[allow(unused_variables)]
pub fn match_bytes(file: &[u8], mimetype: &str) -> bool {
    // Get magic ruleset
    let Some(graph) = ALL_RULES.get(mimetype) else {
        return false;
    };

    // Check all rulesets
    graph
        .externals(Incoming)
        .any(|node| from_u8_walker(file, graph, node, true))
}

/// This only exists for the case of a direct match_filepath call
/// and even then we could probably get rid of this...
#[allow(unused_variables)]
pub fn match_filepath(filepath: &Path, mimetype: &str) -> bool {
    // Get magic ruleset
    let Some(magic_rules) = ALL_RULES.get(mimetype) else {
        return false;
    };

    // Get # of bytes to read
    let scanlen = magic_rules
        .node_weights()
        .map(|rule| rule.scan_len())
        .max()
        .unwrap_or(0);

    let Ok(bytes) = read_bytes(filepath, scanlen) else {
        return false;
    };

    match_bytes(&bytes, mimetype)
}
