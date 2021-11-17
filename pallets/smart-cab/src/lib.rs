#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use codec::{Decode, Encode};
use sp_runtime::{
	RuntimeDebug,
	traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize, Member}
};
use sp_std::prelude::*;


#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct Contract {
	contract_code: Vec<u8>,
	package_code: Vec<u8>,
	warrant_list: Vec<Vec<u8>>,
	status: Vec<u8>,
	user: Vec<u8>
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct ContractData<BlockNumber, AccountId> {
	contract_code: Vec<u8>,
	package_code: Vec<u8>,
	warrant_list: Vec<Vec<u8>>,
	status: Vec<u8>,
	create_block_num: BlockNumber,
	creater: AccountId,
	modify_user: AccountId,
	modify_block_num: BlockNumber,
	user: Vec<u8>
}


#[frame_support::pallet]
pub mod pallet {
	//use crate::Contract;

use super::*;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type ContractCode:  Parameter + Member + AtLeast32BitUnsigned + Default + Copy + MaybeSerializeDeserialize + codec::FullCodec;

		//type ClassId: Parameter + Member + AtLeast32BitUnsigned + Default + Copy + MaybeSerializeDeserialize + codec::FullCodec;
	}

	pub type ContractDataOf<T> = ContractData<<T as frame_system::Config>::BlockNumber, <T as frame_system::Config>::AccountId>;
	// pub type ClassInfoOf<T> =
	// 	ClassInfo<<T as Config>::TokenId, <T as frame_system::Config>::AccountId, <T as Config>::ClassData>;
	
	

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn contracts)]
	pub type Contracts<T: Config> = StorageMap<_, Twox64Concat, Vec<u8>, ContractDataOf<T>>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		ContractCreate(Vec<u8>, Vec<u8>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}


	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_contract(origin: OriginFor<T>, contract: Contract) -> DispatchResultWithPostInfo {

			let _who = ensure_signed(origin)?;
			let c_code = contract.contract_code.clone();

			let c_data = ContractData {
				contract_code: c_code.clone(),
				package_code: contract.package_code.clone(),
				warrant_list: contract.warrant_list.clone(),
				status: contract.status.clone(),
				create_block_num: <frame_system::Pallet<T>>::block_number(),
				creater: _who.clone(),
				modify_user:  _who.clone(),
				modify_block_num: <frame_system::Pallet<T>>::block_number(),
				user: contract.user.clone()
			};

			Contracts::<T>::insert(c_code.clone(), c_data.clone());
			Self::deposit_event(Event::ContractCreate(c_code.clone(), contract.user.clone()));

			Ok(().into())
		}

		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn borrow_contract(origin: OriginFor<T>, contract_code: Vec<u8>, user: Vec<u8>) -> DispatchResultWithPostInfo {
			
		// 	//Ok(().into())
		// 	ensure_signed(origin);
		// 	Ok(().into())
		// }



	}
}
