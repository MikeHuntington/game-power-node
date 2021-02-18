#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get, RuntimeDebug};
use frame_system::{self as system, ensure_signed};
use codec::{Encode, Decode};
use sp_std::prelude::*;


type AccountIdOf<T> = <T as system::Trait>::AccountId;

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
pub trait Trait: system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}


// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
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
			AccountId = AccountIdOf<T>
	{
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, AccountId),
		GuildCreated(u64, Vec<u8>, AccountId),
		GuildUpdated(u64, Vec<u8>),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
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
			let _owner = ensure_signed(origin)?;

			Self::deposit_event(RawEvent::GuildUpdated(guild_id, update.name));

			Ok(())
		}
	}
}
