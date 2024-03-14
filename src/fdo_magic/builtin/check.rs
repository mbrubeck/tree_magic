use super::ALLRULES;
use crate::{fdo_magic::check::from_u8_walker, read_bytes, Mime};
use fnv::FnvHashMap;
use petgraph::prelude::*;
use std::path::Path;

pub(crate) struct FdoMagic;

impl crate::Checker for FdoMagic {
    fn from_u8(&self, file: &[u8], mimetype: &str) -> bool {
        from_u8(file, mimetype)
    }

    fn from_filepath(&self, filepath: &Path, mimetype: &str) -> bool {
        from_filepath(filepath, mimetype)
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
pub fn from_u8(file: &[u8], mimetype: &str) -> bool {
    // Get magic ruleset
    let Some(graph) = ALLRULES.get(mimetype) else {
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
pub fn from_filepath(filepath: &Path, mimetype: &str) -> bool {
    // Get magic ruleset
    let Some(magic_rules) = ALLRULES.get(mimetype) else {
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

    from_u8(&bytes, mimetype)
}
