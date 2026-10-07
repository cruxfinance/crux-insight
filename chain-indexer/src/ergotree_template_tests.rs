//! Drift tests for the shared ErgoTree template helper.

use crate::ergotree_template::*;
use ergo_lib::ergotree_ir::ergo_tree::ErgoTree;
use ergo_lib::ergotree_ir::serialization::SigmaSerializable;
use serde::Deserialize;

#[derive(Deserialize)]
struct Entry {
    ergotree: String,
    template_028: String,
    address_028: String,
}

/// Real mainnet trees with the templates / addresses computed by ergo-lib
/// 0.28.0. Under 0.29 every non-empty 0.28 template must be reproduced
/// byte-for-byte; the ones 0.28 could not handle (ErgoTree v3) must now
/// have a template.
/// Debug builds recurse deeply on large trees and overflow the default 2 MB
/// test-thread stack, so the body runs on a bigger one.
#[test]
fn templates_match_028_fixture() {
    std::thread::Builder::new()
        .stack_size(32 << 20)
        .spawn(templates_match_028_fixture_impl)
        .unwrap()
        .join()
        .unwrap();
}

fn templates_match_028_fixture_impl() {
    use ergo_lib::ergotree_ir::chain::address::{Address, NetworkAddress, NetworkPrefix};
    let entries: Vec<Entry> =
        serde_json::from_str(include_str!("../testdata/template_fixture.json")).unwrap();
    let (mut matched, mut previously_empty, mut now_nonempty) = (0, 0, 0);
    for e in &entries {
        let bytes = hex::decode(&e.ergotree).unwrap();
        let tree = ErgoTree::sigma_parse_bytes(&bytes).unwrap();
        let tpl = template_hex_for_tree(&tree);
        let addr = NetworkAddress::new(
            NetworkPrefix::Mainnet,
            &Address::recreate_from_ergo_tree(&tree).unwrap(),
        )
        .to_base58();
        assert_eq!(addr, e.address_028, "address drift for {}", e.ergotree);
        if e.template_028.is_empty() {
            previously_empty += 1;
            let version = bytes[0] & 0x07;
            if !tpl.is_empty() {
                now_nonempty += 1;
            }
            eprintln!(
                "0.28 empty: header=0x{:02x} version={version} -> 0.29 template len {}",
                bytes[0],
                tpl.len() / 2
            );
            assert!(!tpl.is_empty(), "v3 tree still has no template: {}", e.ergotree);
        } else {
            assert_eq!(tpl, e.template_028, "template drift for {}", e.ergotree);
            matched += 1;
        }
    }
    eprintln!(
        "sampled={} matched={matched} previously_empty={previously_empty} now_nonempty={now_nonempty}",
        entries.len()
    );
    assert!(entries.len() >= 400);
}

/// Spectrum N2T pool template, copied from
/// `crux_boxes::spectrum::template::N2T_POOL_TEMPLATE_HEX`.
const N2T_POOL_TEMPLATE_HEX: &str =
    "d819d601b2a5730000d602e4c6a70404d603db63087201d604db6308a7d605b27203730100d606b27204730200d607b27203730300d608b27204730400d6099973058c720602d60a999973068c7205027209d60bc17201d60cc1a7d60d99720b720cd60e91720d7307d60f8c720802d6107e720f06d6117e720d06d612998c720702720fd6137e720c06d6147308d6157e721206d6167e720a06d6177e720906d6189c72117217d6199c72157217d1ededededededed93c27201c2a793e4c672010404720293b27203730900b27204730a00938c7205018c720601938c7207018c72080193b17203730b9593720a730c95720e929c9c721072117e7202069c7ef07212069a9c72137e7214067e9c720d7e72020506929c9c721372157e7202069c7ef0720d069a9c72107e7214067e9c72127e7202050695ed720e917212730d907216a19d721872139d72197210ed9272189c721672139272199c7216721091720b730e";

#[test]
fn spectrum_n2t_pool_template() {
    let n2t = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/testdata/n2t_pool.hex"
    ))
    .unwrap();
    assert_eq!(
        template_hex(&hex::decode(n2t.trim()).unwrap()).unwrap(),
        N2T_POOL_TEMPLATE_HEX
    );
}

