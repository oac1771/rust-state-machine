use num::traits::Zero;
use std::{collections::BTreeMap, ops::AddAssign};

#[derive(Debug)]
pub struct System<AccountId, BlockNumber, Nonce> {
	block_number: BlockNumber,
	nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> System<AccountId, BlockNumber, Nonce>
where
	AccountId: Ord,
	BlockNumber: Zero + AddAssign + From<u32> + Copy,
	Nonce: From<u32> + Into<u32> + Copy,
{
	pub fn new() -> Self {
		Self { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}

	pub fn inc_block_nubmer(&mut self) {
		self.block_number += BlockNumber::from(1);
	}

	pub fn inc_nonce(&mut self, who: AccountId) {
		let nonce = self.nonce.get(&who);

		if let Some(val) = nonce {
			let new_nonce = (*val).into() + 1;
			self.nonce.insert(who, Nonce::from(new_nonce));
		} else {
			self.nonce.insert(who, Nonce::from(1));
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use super::super::types::*;

	#[test]
	fn test_inc_block_number() {
		let mut system: System<&str, BlockNumber, Nonce> = System::new();

		assert_eq!(system.block_number(), 0);
		system.inc_block_nubmer();
		assert_eq!(system.block_number(), 1);
	}

	#[test]
	fn test_inc_nonce() {
		let mut system: System<&str, BlockNumber, Nonce> = System::new();

		system.inc_nonce("alice");
		assert_eq!(*system.nonce.get("alice").unwrap(), 1);

		system.inc_nonce("alice");
		assert_eq!(*system.nonce.get("alice").unwrap(), 2);
	}
}
