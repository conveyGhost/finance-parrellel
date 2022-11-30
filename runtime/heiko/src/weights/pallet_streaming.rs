
//! Autogenerated weights for `pallet_streaming`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-88-3-164`, CPU: `Intel(R) Xeon(R) Platinum 8124M CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("heiko-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/parallel
// benchmark
// pallet
// --chain=heiko-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_streaming
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=./runtime/heiko/src/weights/pallet_streaming.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_streaming`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_streaming::WeightInfo for WeightInfo<T> {
	// Storage: Streaming MinimumDeposits (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Streaming NextStreamId (r:1 w:1)
	// Storage: Streaming StreamLibrary (r:4 w:4)
	// Storage: Streaming Streams (r:0 w:1)
	fn create() -> Weight {
		Weight::from_ref_time(123_707_000 as u64)
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(10 as u64))
	}
	// Storage: Streaming Streams (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Streaming StreamLibrary (r:2 w:2)
	fn cancel() -> Weight {
		Weight::from_ref_time(124_340_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: Streaming Streams (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn withdraw() -> Weight {
		Weight::from_ref_time(99_552_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Streaming MinimumDeposits (r:0 w:1)
	fn set_minimum_deposit() -> Weight {
		Weight::from_ref_time(25_143_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
