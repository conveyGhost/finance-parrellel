//! Autogenerated weights for pallet_multisig
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-08-04, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("heiko"), DB CACHE: 128

// Executed Command:
// ./target/release/parallel
// benchmark
// --chain=heiko
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_multisig
// --extrinsic=*
// --steps=50
// --repeat=20
// --raw
// --output=./runtime/heiko/src/weights//pallet_multisig.rs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::all)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_multisig.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
    fn as_multi_threshold_1(z: u32) -> Weight {
        (22_365_000 as Weight)
            // Standard Error: 0
            .saturating_add((1_000 as Weight).saturating_mul(z as Weight))
    }
    fn as_multi_create(s: u32, z: u32) -> Weight {
        (70_880_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((196_000 as Weight).saturating_mul(s as Weight))
            // Standard Error: 0
            .saturating_add((1_000 as Weight).saturating_mul(z as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn as_multi_create_store(s: u32, z: u32) -> Weight {
        (79_276_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((204_000 as Weight).saturating_mul(s as Weight))
            // Standard Error: 0
            .saturating_add((3_000 as Weight).saturating_mul(z as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn as_multi_approve(s: u32, z: u32) -> Weight {
        (39_981_000 as Weight)
            // Standard Error: 0
            .saturating_add((201_000 as Weight).saturating_mul(s as Weight))
            // Standard Error: 0
            .saturating_add((1_000 as Weight).saturating_mul(z as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn as_multi_approve_store(s: u32, z: u32) -> Weight {
        (75_521_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((218_000 as Weight).saturating_mul(s as Weight))
            // Standard Error: 0
            .saturating_add((3_000 as Weight).saturating_mul(z as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn as_multi_complete(s: u32, z: u32) -> Weight {
        (99_859_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((372_000 as Weight).saturating_mul(s as Weight))
            // Standard Error: 0
            .saturating_add((5_000 as Weight).saturating_mul(z as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn approve_as_multi_create(s: u32) -> Weight {
        (70_413_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((215_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn approve_as_multi_approve(s: u32) -> Weight {
        (39_819_000 as Weight)
            // Standard Error: 0
            .saturating_add((207_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn approve_as_multi_complete(s: u32) -> Weight {
        (161_760_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((375_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn cancel_as_multi(s: u32) -> Weight {
        (118_928_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((212_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
}
