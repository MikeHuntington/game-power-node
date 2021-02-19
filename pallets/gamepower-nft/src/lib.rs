#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::unused_unit)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use enumflags2::BitFlags;
use frame_support::{
	decl_module, 
	decl_storage, 
	decl_event, 
	decl_error, 
	dispatch, 
	ensure,
	traits::{Currency, Get, ExistenceRequirement::KeepAlive}, 
};
use frame_system::{self as system, ensure_signed};
use codec::{Encode, Decode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::prelude::*;
use primitives::{Balance, NFTBalance};
use sp_runtime::{
	traits::{AccountIdConversion, StaticLookup, Zero},
	DispatchResult, ModuleId, RuntimeDebug,
};
use orml_traits::{BasicCurrency, BasicReservableCurrency};
use orml_utilities::with_transaction_result;
use orml_nft::{self as nft};

type AccountIdOf<T> = <T as system::Trait>::AccountId;
pub type TokenIdOf<T> = <T as orml_nft::Trait>::TokenId;
pub type ClassIdOf<T> = <T as orml_nft::Trait>::ClassId;

/// NFT Properties
pub type CID = sp_std::vec::Vec<u8>;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TokenData {
	/// The minimum balance to create token
	pub deposit: Balance,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ClassData {
	/// The minimum balance to create class
	pub deposit: Balance,
	/// Property of token
	pub properties: ClassProperties,
}

#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ClassProperties {
	/// Token can be transferred
	transferable: bool,
	/// Token can be burned
	burnable: bool,
}

#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, RuntimeDebug)]
pub struct Guild<AccountIdOf> {
  pub id: GuildId,
  pub name: Vec<u8>,
  pub members: Vec<AccountIdOf>,
}
type GuildOf<T> = Guild<AccountIdOf<T>>;


#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct GuildUpdate<AccountIdOf> {
	pub members: Vec<AccountIdOf>,
	pub name: Vec<u8>,
}

pub type GuildId = u64;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: system::Trait + nft::Trait<ClassData = ClassData, TokenData = TokenData> {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	/// The minimum balance to create class
	type CreateClassDeposit: Get<Balance>;

	/// The minimum balance to create token
	type CreateTokenDeposit: Get<Balance>;

	/// The NFT's module id
	type ModuleId: Get<ModuleId>;

	type Currency: BasicReservableCurrency<Self::AccountId, Balance = Balance>;
}


// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as GamerPowerNft {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Something get(fn something): Option<u32>;
		Guilds get(fn guild): map hasher(blake2_128_concat) T::AccountId => GuildOf<T>;
		pub NextGuildId get(fn next_guild_id): GuildId = 1;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> 
		where 
			AccountId = AccountIdOf<T>,
			ClassId = ClassIdOf<T>
	{
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, AccountId),
		GuildCreated(u64, Vec<u8>, AccountId),
		GuildUpdated(u64, Vec<u8>),
		/// Created NFT class. \[owner, class_id\]
		CreatedClass(AccountId, ClassId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// No Guild Found for user
		NoGuildFound,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// NFT Class creation failed
		FailedToCreateNFTClass,
	}
}


decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[weight = 10_000]
		pub fn create_guild(origin, guild_name: Vec<u8>) -> dispatch::DispatchResult {
			let owner = ensure_signed(origin)?;
		
			let guild_id: GuildId = Self::next_guild_id();
			let new_guild = Guild {
				id: guild_id,
				name: guild_name,
				members: vec![],
			};

			<Guilds<T>>::insert(&owner, &new_guild);

			Self::deposit_event(RawEvent::GuildCreated(new_guild.id, new_guild.name, owner));

			NextGuildId::mutate(|n| { *n += 1; });

			Ok(())
		}

		#[weight = 10_000]
		pub fn update_guild(origin, guild_id:GuildId, update: GuildUpdate<T::AccountId>) -> dispatch::DispatchResult {
			let owner = ensure_signed(origin)?;

			ensure!(<Guilds<T>>::contains_key(&owner), Error::<T>::NoGuildFound);

			Self::deposit_event(RawEvent::GuildUpdated(guild_id, update.name));

			Ok(())
		}

		#[weight = 10_000]
		pub fn create_class(origin, metadata:CID, properties:ClassProperties) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			let deposit:Balance = T::CreateClassDeposit::get();
			let next_id: T::ClassId = nft::Module::<T>::next_class_id();
			let owner: T::AccountId = T::ModuleId::get().into_sub_account(next_id);

			// it depends https://github.com/paritytech/substrate/issues/7563
			<T as Config>::Currency::transfer(&who, &owner, deposit)?;
			// Currently, use `free_balance(owner)` instead of `deposit`.
			<T as Config>::Currency::reserve(&owner, <T as Config>::Currency::free_balance(&owner))?;

			// owner add proxy delegate to origin
			<T as pallet_proxy::Trait>::Currency::transfer(&who, &owner, proxy_deposit, KeepAlive)?;
			<T as pallet_proxy::Call>::add_proxy_delegate(&owner, who, Default::default(), Zero::zero())?;

			let data = ClassData { deposit, properties };
			nft::Module::<T>::create_class(&owner, metadata, data)?;

			Self::deposit_event(RawEvent::CreatedClass(owner, next_id));
			Ok(())
		}
	}
}