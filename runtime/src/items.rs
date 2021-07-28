////TODO
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};

//TODO
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::prelude::*;
use sp_runtime::{RuntimeDebug};

//where
use crate::{AccountId};

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ItemType {
	Check,
	Ticket,
	DebtReceivable,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ItemClassData {
	pub item_type: ItemType,
	pub info: Vec<u8>,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ItemTokenData {
	pub name: Vec<u8>,
	pub price: u64,
}

// This is returning the testnet genesis config
pub fn items_genesis(owner: &AccountId) ->
	Vec<(
		AccountId,
		Vec<u8>,
		ItemClassData,
		Vec<(AccountId, Vec<u8>, ItemTokenData)>
	)>
{
	vec![(
		owner.clone(),
		b"sword".to_vec(),
		ItemClassData {
			item_type: ItemType::Check,
			info: b"Sword".to_vec(),
		},
		vec![(
			owner.clone(),
			b"iron_sword".to_vec(),
			ItemTokenData {
				name: b"Iron Sword".to_vec(),
				price: 7,

			}
		), (
			owner.clone(),
			b"steel_sword".to_vec(),
			ItemTokenData {
				name: b"Steel Sword".to_vec(),
				price: 14,

			}
		)]
	), (
		owner.clone(),
		b"hat".to_vec(),
		ItemClassData {
			item_type: ItemType::Ticket,
			info: b"Hat".to_vec(),
		},
		vec![]
	)]
}
