// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_bridge
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("vanilla-dev"), DB CACHE: 128

// Executed Command:
// target/release/parallel
// benchmark
// --chain=vanilla-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet-bridge
// --extrinsic=*
// --steps=50
// --repeat=20
// --heap-pages=4096
// --template=./.maintain/frame-weight-template.hbs
// --output=./pallets/bridge/src/weights.rs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::all)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_bridge.
pub trait WeightInfo {
    fn set_vote_threshold() -> Weight;
    fn register_chain() -> Weight;
    fn unregister_chain() -> Weight;
    fn register_bridge_token() -> Weight;
    fn unregister_bridge_token() -> Weight;
    fn set_bridge_token_fee() -> Weight;
    fn teleport() -> Weight;
    fn materialize() -> Weight;
}

/// Weights for pallet_bridge using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn set_vote_threshold() -> Weight {
        (23_513_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn register_chain() -> Weight {
        (24_264_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn unregister_chain() -> Weight {
        (25_070_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn register_bridge_token() -> Weight {
        (30_817_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn unregister_bridge_token() -> Weight {
        (30_536_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn set_bridge_token_fee() -> Weight {
        (31_773_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn teleport() -> Weight {
        (101_743_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn materialize() -> Weight {
        (136_015_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(7 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn set_vote_threshold() -> Weight {
        (23_513_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn register_chain() -> Weight {
        (24_264_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn unregister_chain() -> Weight {
        (25_070_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn register_bridge_token() -> Weight {
        (30_817_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn unregister_bridge_token() -> Weight {
        (30_536_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn set_bridge_token_fee() -> Weight {
        (31_773_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn teleport() -> Weight {
        (101_743_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn materialize() -> Weight {
        (136_015_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(7 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
}
