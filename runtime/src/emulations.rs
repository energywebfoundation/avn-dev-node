use crate::{AccountId, Vec};
use frame_support::pallet_prelude::{DispatchError, DispatchResult};

use pallet_ethereum_transactions::CandidateTransactionSubmitter;

use pallet_ethereum_transactions::ethereum_transaction::{EthTransactionType, TransactionId};
use sp_avn_common::event_types::EthEventId;
use sp_core::ecdsa;

pub struct CandidateTransactionSubmitterEmulation;

// This trait is implemented in ethereum-transactions pallet and used instead when included in the runtime.
// This is a mock implementation that shouldn't be used on production systems

impl CandidateTransactionSubmitter<AccountId> for CandidateTransactionSubmitterEmulation {
	fn reserve_transaction_id(
		_candidate_type: &EthTransactionType,
	) -> Result<TransactionId, DispatchError> {
		return Ok(0 as TransactionId)
	}

	fn submit_candidate_transaction_to_tier1(
		_candidate_type: EthTransactionType,
		_tx_id: TransactionId,
		_submitter: AccountId,
		_signatures: Vec<ecdsa::Signature>,
	) -> DispatchResult {
		Ok(())
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn set_transaction_id(candidate_type: &EthTransactionType, id: TransactionId) {}
}

pub struct ProcessedEventsCheckerEmulation;

impl pallet_avn::ProcessedEventsChecker for ProcessedEventsCheckerEmulation {
	fn check_event(_event_id: &EthEventId) -> bool {
		return false
	}
}
