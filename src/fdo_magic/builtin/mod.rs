//! Read magic file bundled in crate

use super::MagicRule;
use crate::Mime;
use fnv::FnvHashMap;
use once_cell::sync::Lazy;
use petgraph::prelude::*;

/// Preload alias list
static ALIASES: Lazy<FnvHashMap<Mime, Mime>> = Lazy::new(init::get_aliaslist);

/// Load magic file before anything else.
static ALL_RULES: Lazy<FnvHashMap<Mime, DiGraph<MagicRule<'static>, u32>>> = Lazy::new(rules);

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
