// This file is part of CORD – https://cord.network

// Copyright (C) 2019-2023 Dhiway Networks Pvt. Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// CORD is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// CORD is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with CORD. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_democracy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for WeightInfo<T> {
	// Storage: Democracy PublicPropCount (r:1 w:1)
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	// Storage: Democracy DepositOf (r:0 w:1)
	fn propose() -> Weight {
		// Minimum execution time: 42_967 nanoseconds.
		Weight::from_ref_time(43_770_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy DepositOf (r:1 w:1)
	fn second() -> Weight {
		// Minimum execution time: 39_741 nanoseconds.
		Weight::from_ref_time(41_106_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_new() -> Weight {
		// Minimum execution time: 50_419 nanoseconds.
		Weight::from_ref_time(51_479_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_existing() -> Weight {
		// Minimum execution time: 51_170 nanoseconds.
		Weight::from_ref_time(51_755_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Cancellations (r:1 w:1)
	fn emergency_cancel() -> Weight {
		// Minimum execution time: 21_378 nanoseconds.
		Weight::from_ref_time(21_957_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Blacklist (r:0 w:1)
	fn blacklist() -> Weight {
		// Minimum execution time: 77_526 nanoseconds.
		Weight::from_ref_time(79_470_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	fn external_propose() -> Weight {
		// Minimum execution time: 16_498 nanoseconds.
		Weight::from_ref_time(17_076_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_majority() -> Weight {
		// Minimum execution time: 4_502 nanoseconds.
		Weight::from_ref_time(4_690_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_default() -> Weight {
		// Minimum execution time: 4_599 nanoseconds.
		Weight::from_ref_time(4_750_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn fast_track() -> Weight {
		// Minimum execution time: 20_905 nanoseconds.
		Weight::from_ref_time(21_387_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:1)
	fn veto_external() -> Weight {
		// Minimum execution time: 26_946 nanoseconds.
		Weight::from_ref_time(27_534_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn cancel_proposal() -> Weight {
		// Minimum execution time: 65_837 nanoseconds.
		Weight::from_ref_time(66_889_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn cancel_referendum() -> Weight {
		// Minimum execution time: 13_714 nanoseconds.
		Weight::from_ref_time(14_237_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:2 w:0)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base(r: u32, ) -> Weight {
		// Minimum execution time: 6_405 nanoseconds.
		Weight::from_ref_time(9_620_132 as u64)
			// Standard Error: 3_895
			.saturating_add(Weight::from_ref_time(2_076_637 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy LastTabledWasExternal (r:1 w:0)
	// Storage: Democracy NextExternal (r:1 w:0)
	// Storage: Democracy PublicProps (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:2 w:0)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
		// Minimum execution time: 8_766 nanoseconds.
		Weight::from_ref_time(12_133_052 as u64)
			// Standard Error: 4_049
			.saturating_add(Weight::from_ref_time(2_074_530 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy VotingOf (r:3 w:3)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:2 w:2)
	/// The range of component `r` is `[0, 99]`.
	fn delegate(r: u32, ) -> Weight {
		// Minimum execution time: 42_683 nanoseconds.
		Weight::from_ref_time(48_330_501 as u64)
			// Standard Error: 7_233
			.saturating_add(Weight::from_ref_time(2_994_008 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Democracy VotingOf (r:2 w:2)
	// Storage: Democracy ReferendumInfoOf (r:2 w:2)
	/// The range of component `r` is `[0, 99]`.
	fn undelegate(r: u32, ) -> Weight {
		// Minimum execution time: 25_626 nanoseconds.
		Weight::from_ref_time(29_036_412 as u64)
			// Standard Error: 4_605
			.saturating_add(Weight::from_ref_time(2_952_295 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Democracy PublicProps (r:0 w:1)
	fn clear_public_proposals() -> Weight {
		// Minimum execution time: 4_943 nanoseconds.
		Weight::from_ref_time(5_130_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_remove(r: u32, ) -> Weight {
		// Minimum execution time: 25_894 nanoseconds.
		Weight::from_ref_time(32_320_132 as u64)
			// Standard Error: 1_445
			.saturating_add(Weight::from_ref_time(30_390 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_set(r: u32, ) -> Weight {
		// Minimum execution time: 30_098 nanoseconds.
		Weight::from_ref_time(31_959_025 as u64)
			// Standard Error: 673
			.saturating_add(Weight::from_ref_time(70_608 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	/// The range of component `r` is `[1, 100]`.
	fn remove_vote(r: u32, ) -> Weight {
		// Minimum execution time: 15_813 nanoseconds.
		Weight::from_ref_time(18_185_628 as u64)
			// Standard Error: 850
			.saturating_add(Weight::from_ref_time(78_098 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	/// The range of component `r` is `[1, 100]`.
	fn remove_other_vote(r: u32, ) -> Weight {
		// Minimum execution time: 15_738 nanoseconds.
		Weight::from_ref_time(18_380_060 as u64)
			// Standard Error: 970
			.saturating_add(Weight::from_ref_time(77_708 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}