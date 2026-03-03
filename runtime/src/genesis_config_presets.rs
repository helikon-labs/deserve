use crate::{
    AccountId, Balance, BalancesConfig, CollatorSelectionConfig, ParachainInfoConfig,
    PolkadotXcmConfig, RuntimeGenesisConfig, SessionConfig, SessionKeys, SudoConfig,
    DSRV as BALANCE_UNIT,
};

use alloc::{vec, vec::Vec};

use polkadot_sdk::{
    sp_application_crypto::Ss58Codec, sp_runtime::AccountId32, staging_xcm as xcm, *,
};

use cumulus_primitives_core::ParaId;
use frame_support::build_struct_json_patch;
use parachains_common::AuraId;
use serde_json::Value;
use sp_genesis_builder::PresetId;
use sp_keyring::Sr25519Keyring;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;
/// Parachain id used for genesis config presets of parachain template.
#[docify::export_content]
pub const PARACHAIN_ID: u32 = 5142;

/// Generate the session keys from individual elements.
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: AuraId) -> SessionKeys {
    SessionKeys { aura: keys }
}

fn get_genesis(
    parachain_id: ParaId,
    root: AccountId,
    balances: Vec<(AccountId, u128)>,
    authority_candidacy_bond: Balance,
    authorities: Vec<(AccountId, AuraId)>,
) -> Value {
    build_struct_json_patch!(RuntimeGenesisConfig {
        balances: BalancesConfig { balances },
        parachain_info: ParachainInfoConfig { parachain_id },
        collator_selection: CollatorSelectionConfig {
            invulnerables: authorities
                .iter()
                .cloned()
                .map(|(acc, _)| acc)
                .collect::<Vec<_>>(),
            candidacy_bond: authority_candidacy_bond,
        },
        session: SessionConfig {
            keys: authorities
                .into_iter()
                .map(|(acc, aura)| {
                    (
                        acc.clone(),                 // account id
                        acc,                         // validator id
                        template_session_keys(aura), // session keys
                    )
                })
                .collect::<Vec<_>>(),
        },
        polkadot_xcm: PolkadotXcmConfig {
            safe_xcm_version: Some(SAFE_XCM_VERSION)
        },
        sudo: SudoConfig { key: Some(root) },
    })
}

fn get_devnet_genesis() -> Value {
    let root = Sr25519Keyring::Alice.to_account_id();
    let balances = Sr25519Keyring::well_known()
        .map(|k| (k.to_account_id(), 1u128 << 60))
        .collect::<Vec<_>>();
    let authority_candidacy_bond = 10_000 * BALANCE_UNIT;
    let authorities = vec![
        (
            Sr25519Keyring::Alice.to_account_id(),
            Sr25519Keyring::Alice.public().into(),
        ),
        (
            Sr25519Keyring::Bob.to_account_id(),
            Sr25519Keyring::Bob.public().into(),
        ),
    ];
    get_genesis(
        PARACHAIN_ID.into(),
        root,
        balances,
        authority_candidacy_bond,
        authorities,
    )
}

fn get_testnet_genesis() -> Value {
    // collator #1
    let collator_1_grandpa =
        AccountId32::from_ss58check("16AYbHs4itBR8YHjkWwNaXa97PSmG197KqQ4VFc9U8BnVLv").unwrap();
    let collator_1_aura =
        AuraId::from_ss58check("15SB1PxkgzYvGZpGQgV41wSvG8yEqwvthrL3JtnMHG9jEcrK").unwrap();
    // collator #2
    let collator_2_grandpa =
        AccountId32::from_ss58check("13FqsCdzLXw471uZNjEmh4EuoMfMt4HsTfVovb8aAFCX6M4S").unwrap();
    let collator_2_aura =
        AuraId::from_ss58check("14Q4H1184DakxsevWh8nxfVAzLT6JRuM76pjK82iryVo4NQz").unwrap();
    // collator #3
    let collator_3_grandpa =
        AccountId32::from_ss58check("12Rq4TYzrcRvF3GemM6dsWu8cvzbdSH4bif6ZkWiHboanfs8").unwrap();
    let collator_3_aura =
        AuraId::from_ss58check("12zs9jsgABc9DFS2bojakSzKkLiCvg2Kwi4YiuXoiprpGQTW").unwrap();

    let root =
        AccountId::from_ss58check("16GGZkN7h6X2s3JCAhfJBj5ErPeLVpKSeoENXGFA8cFg5q34").unwrap();
    let faucet =
        AccountId::from_ss58check("121LpfMRyitLS26AMiZZL7hu9jcRrP8cW12Fc3ehVpCcjCoj").unwrap();
    let developer =
        AccountId::from_ss58check("15fTH34bbKGMUjF1bLmTqxPYgpg481imThwhWcQfCyktyBzL").unwrap();

    let million_unit = 1_000_000 * BALANCE_UNIT;
    let balances = vec![
        (root.clone(), 100 * million_unit),
        (faucet, 1_000 * million_unit),
        (developer, million_unit),
        (collator_1_grandpa.clone(), million_unit),
        (collator_2_grandpa.clone(), million_unit),
        (collator_3_grandpa.clone(), million_unit),
    ];
    let authority_candidacy_bond = 10_000 * BALANCE_UNIT;

    let authorities = vec![
        (collator_1_grandpa, collator_1_aura),
        (collator_2_grandpa, collator_2_aura),
        (collator_3_grandpa, collator_3_aura),
    ];
    get_genesis(
        PARACHAIN_ID.into(),
        root,
        balances,
        authority_candidacy_bond,
        authorities,
    )
}

/// Provides the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &PresetId) -> Option<vec::Vec<u8>> {
    let patch = match id.as_ref() {
        ""
        | "dev"
        | super::DEVNET_PRESET
        | sp_genesis_builder::DEV_RUNTIME_PRESET
        | sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => get_devnet_genesis(),
        "test" | "testnet" | super::TESTNET_PRESET => get_testnet_genesis(),
        _ => return None,
    };
    Some(
        serde_json::to_string(&patch)
            .expect("serialization to json is expected to work. qed.")
            .into_bytes(),
    )
}

/// List of supported presets.
pub fn preset_names() -> Vec<PresetId> {
    vec![
        PresetId::from(sp_genesis_builder::DEV_RUNTIME_PRESET),
        PresetId::from(super::DEVNET_PRESET),
        PresetId::from(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET),
        PresetId::from(super::TESTNET_PRESET),
    ]
}
