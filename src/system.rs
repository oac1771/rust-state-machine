use std::collections::BTreeMap;

#[derive(Debug)]
pub struct System {
	block_number: u32,
	nonce: BTreeMap<String, u32>,
}

impl System {
	pub fn new() -> Self {
		Self { block_number: 0, nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> u32 {
		self.block_number
	}

	pub fn inc_block_nubmer(&mut self) {
		self.block_number += 1
	}

	pub fn inc_nonce(&mut self, who: &str) {
		let nonce = self.nonce.get(who);

		if let Some(val) = nonce {
			let new_nonce = val + 1;
			self.nonce.insert(who.to_string(), new_nonce);
		} else {
			self.nonce.insert(who.to_string(), 1);
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_inc_block_number() {
		let mut system = System::new();

		assert_eq!(system.block_number(), 0);
		system.inc_block_nubmer();
		assert_eq!(system.block_number(), 1);
	}

	#[test]
	fn test_inc_nonce() {
		let mut system = System::new();

		system.inc_nonce("alice");
		assert_eq!(*system.nonce.get("alice").unwrap(), 1);

		system.inc_nonce("alice");
		assert_eq!(*system.nonce.get("alice").unwrap(), 2);
	}
}
