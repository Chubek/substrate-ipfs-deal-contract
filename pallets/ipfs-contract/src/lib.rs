#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet::*;

#[cfg(test)]
mod tests;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	struct Contract<T> {
		provider: u64,
		beneficiary: u64,
		contact_between_num: u32,
		price_paid: u64,
		provider_signature: Vec<u8>,
		beneficiary_signature: Vec<u8>

	}


	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn contract_holder)]
	pub(super) type ContractHolder<T: Config> = StorageMap<
    _,
 		Twox64Concat,
  		T::Hash,
  		Contract<T>,
	>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {	
		ContractSigned(Vec<u8>, Vec<u8>),
	}


	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
	

	}
}
