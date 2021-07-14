#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
		pallet_prelude::*,
		inherent::Vec
	};
	use frame_support::{dispatch::DispatchResultWithPostInfo};
	use frame_system::pallet_prelude::*;

    //这是一个类型别名定义。ClassId在orml_nft中是一个bound by 多个trait的类型
    pub type ClassIdOf<T> = <T as orml_nft::Config>::ClassId;
    pub type ClassDataOf<T> = <T as orml_nft::Config>::ClassData;
	pub type TokenIdOf<T> = <T as orml_nft::Config>::TokenId;
	pub type TokenDataOf<T> = <T as orml_nft::Config>::TokenData;

	/// Configure the pallet by specifying the parameters and types on which it depends.
    /// 继承被依赖的trait，在impl这个trait的时候也需要impl被继承的super trait
	#[pallet::config]
	pub trait Config: frame_system::Config + orml_nft::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	//#[pallet::storage]
	//#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	//pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
        ItemMinted(T::AccountId, (T::ClassId, T::TokenId), Vec<u8>),

        CateCrated(T::AccountId, T::ClassId, Vec<u8>),

        ItemTransferred(T::AccountId, T::AccountId, (T::ClassId, T::TokenId))
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,

        ClassNotExists,

        NotClassOwner,

        MintItemError,

        NotItemOwner,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn mint_item(origin: OriginFor<T>, cid: ClassIdOf<T>, metadata: Vec<u8>, data: TokenDataOf<T>) -> DispatchResultWithPostInfo {

            let who = ensure_signed(origin)?;
            //这里的<T>的用法？
            let class = orml_nft::Pallet::<T>::classes(cid).ok_or(Error::<T>::ClassNotExists)?;

            if class.owner != who {
                return Err(Error::<T>::NotClassOwner)?
            }

            let tid = orml_nft::Pallet::<T>::mint(&who, cid, metadata.clone(), data).map_err(|_| Error::<T>::MintItemError)?;
            Self::deposit_event(Event::ItemMinted(who, (cid, tid), metadata));
            Ok(().into())
		}

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create_cate(origin: OriginFor<T>, class_name: Vec<u8>, class_data: ClassDataOf<T>) -> DispatchResultWithPostInfo
        {
            let who = ensure_signed(origin)?;
            let cid = orml_nft::Pallet::<T>::create_class(&who, class_name.clone(), class_data.clone())?;
            Self::deposit_event(Event::CateCrated(who.clone(), cid, class_name.clone()));
            Ok(().into())
        }

        #[pallet::weight(10_000)]
		pub fn transfer_item(
			origin: OriginFor<T>,
			to: T::AccountId,
			token: (ClassIdOf<T>, TokenIdOf<T>)
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			// check: if the origin is the owner of the token, implicitly checking the existence of the
			//   class and token ID
			if !orml_nft::Pallet::<T>::is_owner(&who, token.clone()) {
				Err(Error::<T>::NotItemOwner)?
			}

			// execute: actualize the transfer
			orml_nft::Pallet::<T>::transfer(&who, &to, token.clone())?;
			Self::deposit_event(Event::ItemTransferred(who, to, token));
			Ok(().into())
		}
	}
}
