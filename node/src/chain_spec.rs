use polkadot_sdk::*;

use deserve_runtime as runtime;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};

pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;
pub const ROCOCO_LOCAL: &str = "rococo-local";
pub const PASEO: &str = "paseo";
pub const TOKEN_SYMBOL: &str = "DSRV";
pub const TOKEN_DECIMALS: u32 = 9;
pub const SS58_FORMAT: u32 = 0;

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    #[serde(alias = "relayChain", alias = "RelayChain")]
    pub relay_chain: String,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

fn get_chain_properties() -> sc_chain_spec::Properties {
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), TOKEN_SYMBOL.into());
    properties.insert("tokenDecimals".into(), TOKEN_DECIMALS.into());
    properties.insert("ss58Format".into(), SS58_FORMAT.into());
    properties
}

pub fn devnet_chain_spec() -> ChainSpec {
    ChainSpec::builder(
        runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: ROCOCO_LOCAL.into(),
        },
    )
    .with_name("DeServe Devnet")
    .with_id(deserve_runtime::DEVNET_PRESET)
    .with_chain_type(ChainType::Development)
    .with_properties(get_chain_properties())
    .with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
    .build()
}

pub fn testnet_chain_spec() -> ChainSpec {
    ChainSpec::builder(
        runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: PASEO.into(),
        },
    )
    .with_name("DeServe Testnet")
    .with_id(deserve_runtime::TESTNET_PRESET)
    .with_chain_type(ChainType::Live)
    .with_properties(get_chain_properties())
    .with_genesis_config_preset_name(deserve_runtime::TESTNET_PRESET)
    .build()
}
