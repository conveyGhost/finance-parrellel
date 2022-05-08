
//! Autogenerated weights for `pallet_bridge`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("vanilla-dev"), DB CACHE: 1024

// Executed Command:
// ./parallel
// benchmark
// pallet
// --chain=vanilla-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_bridge
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=./runtime/vanilla/src/weights/pallet_bridge.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_bridge`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge::WeightInfo for WeightInfo<T> {
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Bridge ChainNonces (r:1 w:1)
	// Storage: Bridge BridgeRegistry (r:0 w:1)
	fn register_chain() -> Weight {
		(41_666_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Bridge ChainNonces (r:1 w:1)
	// Storage: Bridge BridgeRegistry (r:0 w:1)
	fn unregister_chain() -> Weight {
		(42_104_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Bridge BridgeTokens (r:1 w:1)
	// Storage: Bridge AssetIds (r:1 w:1)
	fn register_bridge_token() -> Weight {
		(47_088_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Bridge AssetIds (r:1 w:1)
	// Storage: Bridge BridgeTokens (r:0 w:1)
	fn unregister_bridge_token() -> Weight {
		(44_484_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Bridge AssetIds (r:1 w:0)
	// Storage: Bridge BridgeTokens (r:1 w:1)
	fn set_bridge_token_fee() -> Weight {
		(51_076_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Bridge AssetIds (r:1 w:0)
	// Storage: Bridge BridgeTokens (r:1 w:1)
	fn set_bridge_token_status() -> Weight {
		(50_462_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Bridge AssetIds (r:1 w:0)
	// Storage: Bridge BridgeTokens (r:1 w:1)
	fn set_bridge_token_cap() -> Weight {
		(50_462_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Bridge AssetIds (r:1 w:0)
	// Storage: Bridge BridgeTokens (r:1 w:1)
	fn clean_cap_accumulated_value() -> Weight {
		(50_323_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Bridge ChainNonces (r:1 w:1)
	// Storage: Bridge AssetIds (r:1 w:0)
	// Storage: Bridge BridgeTokens (r:1 w:1)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn teleport() -> Weight {
		(144_368_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: Bridge ChainNonces (r:1 w:0)
	// Storage: Bridge BridgeRegistry (r:1 w:1)
	// Storage: Bridge AssetIds (r:1 w:0)
	// Storage: Bridge BridgeTokens (r:1 w:1)
	// Storage: Bridge ProposalVotes (r:1 w:1)
	// Storage: Bridge VoteThreshold (r:1 w:0)
	// Storage: BridgeMembership Members (r:1 w:0)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: Assets Metadata (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	fn materialize() -> Weight {
		(238_013_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
}
