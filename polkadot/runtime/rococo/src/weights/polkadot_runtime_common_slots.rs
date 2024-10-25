// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `runtime_common::slots`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-02-29, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-bn-ce5rx-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("rococo-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --pallet=runtime_common::slots
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./polkadot/file_header.txt
// --output=./polkadot/runtime/rococo/src/weights/runtime_common_slots.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `runtime_common::slots`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> polkadot_runtime_common::slots::WeightInfo for WeightInfo<T> {
	/// Storage: Slots Leases (r:1 w:1)
	/// Proof Skipped: Slots Leases (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_lease() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `320`
		//  Estimated: `3785`
		// Minimum execution time: 26_570_000 picoseconds.
		Weight::from_parts(27_619_000, 0)
			.saturating_add(Weight::from_parts(0, 3785))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Paras::Parachains` (r:1 w:0)
	/// Proof: `Paras::Parachains` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Slots::Leases` (r:101 w:100)
	/// Proof: `Slots::Leases` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:200 w:200)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ActionsQueue` (r:1 w:1)
	/// Proof: `Paras::ActionsQueue` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[0, 100]`.
	/// The range of component `t` is `[0, 100]`.
	fn manage_lease_period_start(c: u32, t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `594 + c * (20 ±0) + t * (234 ±0)`
		//  Estimated: `4065 + c * (2496 ±0) + t * (2709 ±0)`
		// Minimum execution time: 729_793_000 picoseconds.
		Weight::from_parts(740_820_000, 0)
			.saturating_add(Weight::from_parts(0, 4065))
			// Standard Error: 88_206
			.saturating_add(Weight::from_parts(2_793_142, 0).saturating_mul(c.into()))
			// Standard Error: 88_206
			.saturating_add(Weight::from_parts(8_933_065, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(t.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(t.into())))
			.saturating_add(Weight::from_parts(0, 2496).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 2709).saturating_mul(t.into()))
	}
	/// Storage: `Slots::Leases` (r:1 w:1)
	/// Proof: `Slots::Leases` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:8 w:8)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn clear_all_leases() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2792`
		//  Estimated: `21814`
		// Minimum execution time: 123_888_000 picoseconds.
		Weight::from_parts(131_245_000, 0)
			.saturating_add(Weight::from_parts(0, 21814))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	/// Storage: `Slots::Leases` (r:1 w:0)
	/// Proof: `Slots::Leases` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:1 w:1)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ActionsQueue` (r:1 w:1)
	/// Proof: `Paras::ActionsQueue` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn trigger_onboard() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `612`
		//  Estimated: `4077`
		// Minimum execution time: 27_341_000 picoseconds.
		Weight::from_parts(28_697_000, 0)
			.saturating_add(Weight::from_parts(0, 4077))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
