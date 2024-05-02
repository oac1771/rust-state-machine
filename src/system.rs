use num::traits::Zero;
use std::{collections::BTreeMap, ops::AddAssign};

pub trait Config {
	type AccountId: Ord + Copy;
	type BlockNumber: Zero + AddAssign + From<u32> + Copy;
	type Nonce: From<u32> + Into<u32> + Copy;
}

#[derive(Debug)]
pub struct System<T: Config> {
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> System<T> {
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	pub fn inc_block_nubmer(&mut self) {
		self.block_number += T::BlockNumber::from(1);
	}

	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	pub fn inc_nonce(&mut self, who: T::AccountId) {
		let nonce = self.nonce.get(&who);

		if let Some(val) = nonce {
			let new_nonce = (*val).into() + 1;
			self.nonce.insert(who, T::Nonce::from(new_nonce));
		} else {
			self.nonce.insert(who, T::Nonce::from(1));
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	struct TestConfig;

	impl Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn test_inc_block_number() {
		let mut system: System<TestConfig> = System::new();

		assert_eq!(system.block_number(), 0);
		system.inc_block_nubmer();
		assert_eq!(system.block_number(), 1);
	}

	#[test]
	fn test_inc_nonce() {
		let mut system: System<TestConfig> = System::new();

		system.inc_nonce("alice");
		assert_eq!(*system.nonce.get("alice").unwrap(), 1);

		system.inc_nonce("alice");
		assert_eq!(*system.nonce.get("alice").unwrap(), 2);
	}
}
