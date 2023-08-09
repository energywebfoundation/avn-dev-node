use avn_dev_runtime::{
	constants::{currency::AVT, time::MINUTES},
	AccountId, AuraId, AvnId, Balance, BlockNumber, ParachainStakingConfig, Signature, SudoConfig,
	SummaryConfig, TokenManagerConfig, ValidatorsManagerConfig, EXISTENTIAL_DEPOSIT,
};
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{ecdsa, sr25519, Pair, Public, H160, H256};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<avn_dev_runtime::GenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

pub const COLLATOR_DEPOSIT: Balance = 2_000 * AVT;
pub type EthPublicKey = ecdsa::Public;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_account_keys_from_seed(seed: &str) -> (AccountId, AuraId, AvnId) {
	(
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<AuraId>(seed),
		get_from_seed::<AvnId>(seed),
	)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(aura: AuraId, avn: AvnId) -> avn_dev_runtime::SessionKeys {
	avn_dev_runtime::SessionKeys { aura, avn }
}

const AVT_CONTRACT_ADDRESS: H160 = H160::zero();
const VOTING_PERIOD: BlockNumber = 20 * MINUTES;

const HALF_HOUR_SCHEDULE_PERIOD: BlockNumber = 30 * MINUTES;
const ENDOWMENT_AMOUNT: Balance = 10_000 * AVT;

pub fn development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "AVT".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"AvN Development",
		// ID
		"avn_dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				// initial collators.
				vec![
					get_collator_account_keys_from_seed("Eve"),
					get_collator_account_keys_from_seed("Ferdie"),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				2000.into(),
				ethereum_public_keys(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 2000,
		},
	)
}

pub fn local_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "AVT".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"AvN Local Testnet",
		// ID
		"avn_dev_local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// initial collators.
				vec![
					get_collator_account_keys_from_seed("Eve"),
					get_collator_account_keys_from_seed("Ferdie"),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				2000.into(),
				ethereum_public_keys(),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("avn_dev_local_testnet"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 2000,
		},
	)
}

fn testnet_genesis(
	invulnerables: Vec<(AccountId, AuraId, AvnId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
	eth_public_keys: Vec<EthPublicKey>,
) -> avn_dev_runtime::GenesisConfig {
	avn_dev_runtime::GenesisConfig {
		system: avn_dev_runtime::SystemConfig {
			code: avn_dev_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: avn_dev_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT_AMOUNT)).collect(),
		},
		parachain_info: avn_dev_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: avn_dev_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.clone().iter().cloned().map(|(acc, _, _)| acc).collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		session: avn_dev_runtime::SessionConfig {
			keys: invulnerables
				.clone()
				.into_iter()
				.map(|(acc, aura, avnk)| {
					(
						acc.clone(),                       // account id
						acc,                               // validator id
						template_session_keys(aura, avnk), // session keys
					)
				})
				.collect(),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		validators_manager: ValidatorsManagerConfig {
			validators: invulnerables
				.clone()
				.iter()
				.map(|x| x.0.clone())
				.zip(eth_public_keys.iter().map(|pk| pk.clone()))
				.collect::<Vec<_>>(),
		},
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		parachain_staking: ParachainStakingConfig {
			candidates: invulnerables
				.iter()
				.cloned()
				.map(|(acc, _, _)| (acc, COLLATOR_DEPOSIT))
				.collect(),
			nominations: vec![],
			min_collator_stake: COLLATOR_DEPOSIT,
			min_total_nominator_stake: 10 * AVT,
			delay: 2,
		},
		polkadot_xcm: avn_dev_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
		assets: Default::default(),
		token_manager: TokenManagerConfig {
			_phantom: Default::default(),
			lower_account_id: H256(hex!(
				"000000000000000000000000000000000000000000000000000000000000dead"
			)),
			// Address of AVT contract
			avt_token_contract: AVT_CONTRACT_ADDRESS,
		},
		sudo: SudoConfig { key: Some(get_account_id_from_seed::<sr25519::Public>("Ferdie")) },
		summary: SummaryConfig {
			schedule_period: HALF_HOUR_SCHEDULE_PERIOD,
			voting_period: VOTING_PERIOD,
		},
	}
}

fn ethereum_public_keys() -> Vec<EthPublicKey> {
	return vec![
		ecdsa::Public::from_full(&hex![
			"03471b4c1012dddf4d494c506a098c7b1b719b20bbb177b1174f2166f953c29503"
		])
		.unwrap(),
		ecdsa::Public::from_full(&hex![
			"0292a73ad9488b934fd04cb31a0f50634841f7105a5b4a8538e4bfa06aa477bed6"
		])
		.unwrap(),
		ecdsa::Public::from_full(&hex![
			"03c5527886d8e09ad1fededd3231f890685d2d5345385d54181269f80c8926ff8e"
		])
		.unwrap(),
		ecdsa::Public::from_full(&hex![
			"020e7593c534411f6f0e2fb91340751ada34ee5986f70b300443be17844416b28b"
		])
		.unwrap(),
		ecdsa::Public::from_full(&hex![
			"02fde5665a2cb42863fb312fb527f2b02110997fc6865df583ca4324be137b7894"
		])
		.unwrap(),
		ecdsa::Public::from_full(&hex![
			"031f8860a4f05ec62077a97d37af60f0229b775b98946efcb92998522abefc1b6c"
		])
		.unwrap(),
	]
}
