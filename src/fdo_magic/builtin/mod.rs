//! Read magic file bundled in crate

use super::MagicRule;
use crate::Mime;
use fnv::FnvHashMap;
use lazy_static::lazy_static;
use petgraph::prelude::*;

/// Preload alias list
lazy_static! {
    static ref ALIASES: FnvHashMap<Mime, Mime> = init::get_aliaslist();
}

/// Load magic file before anything else.
lazy_static! {
    static ref ALLRULES: FnvHashMap<Mime, DiGraph<MagicRule<'static>, u32>> = rules();
}

pub mod check;
pub mod init;

#[cfg(not(feature = "with-gpl-data"))]
mod runtime;

fn rules() -> FnvHashMap<Mime, DiGraph<MagicRule<'static>, u32>> {
    #[cfg(feature = "with-gpl-data")]
    return static_rules();
    #[cfg(not(feature = "with-gpl-data"))]
    return runtime_rules();
}

#[cfg(feature = "with-gpl-data")]
fn static_rules() -> FnvHashMap<Mime, DiGraph<MagicRule<'static>, u32>> {
    super::ruleset::from_u8(tree_magic_db::magic()).unwrap_or_default()
}

#[cfg(not(feature = "with-gpl-data"))]
fn runtime_rules() -> FnvHashMap<Mime, DiGraph<MagicRule<'static>, u32>> {
    runtime::rules().unwrap_or_default()
}
