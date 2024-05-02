use crate::balances;
use crate::system;
use super::types::*;

#[derive(Debug)]
pub struct Runtime {
	pub system: system::System<AccountId, BlockNumber, Nonce>,
	pub balances: balances::Balances<AccountId, Balance>,
}

impl Runtime {
	pub fn new() -> Self {
		Self { system: system::System::new(), balances: balances::Balances::new() }
	}

	pub fn transact(&mut self, from: AccountId, to: AccountId, amount: Balance) {
		self.system.inc_nonce(from);
		let _ = self
			.balances
			.transfer(from, to, amount)
			.map_err(|err| println!("Error during transfer: {}", err));
	}
}
