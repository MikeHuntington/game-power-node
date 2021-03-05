#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::unused_unit)]
#![allow(clippy::upper_case_acronyms)]

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use frame_support::{pallet_prelude::*, transactional,};
use frame_system::pallet_prelude::*;
use sp_runtime::{
	RuntimeDebug,
};


pub use module::*;


#[frame_support::pallet]
pub mod module {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}


	#[pallet::error]
	pub enum Error<T> {

	}


	#[pallet::storage]
	#[pallet::getter(fn next_guild_id)]
	pub(crate) type NextGuildId<T: Config> = StorageValue<_, u32, ValueQuery>;



	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(T::AccountId = "AccountId")]
	pub enum Event<T: Config> {
		/// Game Started Event. \[player\]
		GameStarted(T::AccountId),

        /// Game Ended Event. \[player\]
        GameEnded(T::AccountId),
	}



	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn start_game(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			Self::deposit_event(Event::<T>::GameStarted(who));

			Ok(().into())
		}

        #[pallet::weight(10_000)]
		#[transactional]
		pub fn end_game(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			Self::deposit_event(Event::<T>::GameEnded(who));

			Ok(().into())
		}
	}
}