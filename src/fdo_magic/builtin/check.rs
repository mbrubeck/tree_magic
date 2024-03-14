use crate::{fdo_magic, read_bytes, Mime};
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
    let graph = match super::ALLRULES.get(mimetype) {
        Some(item) => item,
        None => return false, // No rule for this mime
    };

    // Check all rulesets
    for x in graph.externals(Incoming) {
        if fdo_magic::check::from_u8_walker(file, mimetype, graph, x, true) {
            return true;
        }
    }

    false
}

/// This only exists for the case of a direct match_filepath call
/// and even then we could probably get rid of this...
#[allow(unused_variables)]
pub fn from_filepath(filepath: &Path, mimetype: &str) -> bool {
    // Get magic ruleset
    let magic_rules = match super::ALLRULES.get(mimetype) {
        Some(item) => item,
        None => return false, // No rule for this mime
    };

    // Get # of bytes to read
    let mut scanlen = 0;
    for x in magic_rules.raw_nodes() {
        let y = &x.weight;
        let tmplen = y.start_off as usize + y.val.len() + y.region_len as usize;

        if tmplen > scanlen {
            scanlen = tmplen;
        }
    }

    let b = match read_bytes(filepath, scanlen) {
        Ok(x) => x,
        Err(_) => return false,
    };

    from_u8(b.as_slice(), mimetype)
}
