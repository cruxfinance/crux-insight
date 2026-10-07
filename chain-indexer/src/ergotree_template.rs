//! ErgoTree template computation shared by the indexer and the backfill binary.
//!
//! `public.addresses.ergotree_template` stores the hex of the tree's template
//! bytes (constants stripped), or the empty string when no template could be
//! computed. Downstream services classify contracts by this column.

use ergo_lib::ergotree_ir::ergo_tree::ErgoTree;
use ergo_lib::ergotree_ir::serialization::SigmaSerializable;

/// Hex-encoded template of an already parsed tree; `""` if it has none.
pub fn template_hex_for_tree(tree: &ErgoTree) -> String {
    hex::encode(tree.template_bytes().unwrap_or_default())
}

/// Hex-encoded template for serialized tree bytes.
///
/// Returns `""` when the template cannot be computed and `None` only when the
/// bytes do not parse as an ErgoTree at all.
#[allow(dead_code)]
pub fn template_hex(tree_bytes: &[u8]) -> Option<String> {
    ErgoTree::sigma_parse_bytes(tree_bytes)
        .ok()
        .map(|t| template_hex_for_tree(&t))
}
