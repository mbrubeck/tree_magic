use crate::MIME;
use fnv::FnvHashMap;

#[cfg(not(feature = "with-gpl-data"))]
use super::runtime;

fn aliases() -> &'static str {
    #[cfg(feature = "with-gpl-data")]
    return tree_magic_db::aliases();
    #[cfg(not(feature = "with-gpl-data"))]
    return runtime::aliases();
}

fn subclasses() -> &'static str {
    #[cfg(feature = "with-gpl-data")]
    return tree_magic_db::subclasses();
    #[cfg(not(feature = "with-gpl-data"))]
    return runtime::subclasses();
}

pub fn get_aliaslist() -> FnvHashMap<MIME, MIME> {
    aliases()
        .lines()
        .filter_map(|line| {
            if !line.is_empty() {
                let mut parts = line.split_whitespace();
                let a = parts.next().unwrap();
                let b = parts.next().unwrap();
                Some((a, b))
            } else {
                None
            }
        })
        .collect()
}

/// Get list of supported MIME types
pub fn get_supported() -> Vec<MIME> {
    super::ALLRULES.keys().cloned().collect()
}

/// Get list of parent -> child subclass links
pub fn get_subclasses() -> Vec<(MIME, MIME)> {
    subclasses()
        .lines()
        .filter_map(|line| {
            if !line.is_empty() {
                let mut parts = line.split_whitespace();

                let child = parts.next().unwrap();
                let child = super::ALIASES.get(child).copied().unwrap_or(child);

                let parent = parts.next().unwrap();
                let parent = super::ALIASES.get(parent).copied().unwrap_or(parent);

                Some((parent, child))
            } else {
                None
            }
        })
        .collect()
}
