use crate::balances;
use crate::system;

#[derive(Debug)]
pub struct Runtime {
	pub system: system::System<Self>,
	pub balances: balances::Balances<Self>,
}

impl system::Config for Runtime {
	type AccountId = &'static str;
	type BlockNumber = u32;
	type Nonce = u32;
}

impl balances::Config for Runtime {
	type Balance = u128;
}

impl Runtime {
	pub fn new() -> Self {
		Self { system: system::System::new(), balances: balances::Balances::new() }
	}

	pub fn transact(
		&mut self,
		from: <Runtime as system::Config>::AccountId,
		to: <Runtime as system::Config>::AccountId,
		amount: <Runtime as balances::Config>::Balance,
	) {
		self.system.inc_nonce(from);
		let _ = self
			.balances
			.transfer(from, to, amount)
			.map_err(|err| println!("Error during transfer: {}", err));
	}
}
