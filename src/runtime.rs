use crate::balances;
use crate::system;

#[derive(Debug)]
pub struct Runtime {
	pub system: system::System,
	pub balances: balances::Balances,
}

impl Runtime {
	pub fn new() -> Self {
		Self { system: system::System::new(), balances: balances::Balances::new() }
	}

	pub fn transact(&mut self, from: &str, to: &str, amount: u128) {
		self.system.inc_nonce(from);
		let _ = self.balances
			.transfer(from, to, amount)
			.map_err(|err| println!("Error during transfer: {}", err));
	}
}