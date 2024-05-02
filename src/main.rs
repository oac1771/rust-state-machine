mod balances;
mod runtime;
mod system;

pub mod types {
	pub type AccountId = &'static str;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
}

fn main() {
	let alice = "Alice";
	let bob = "Bob";
	let charlie = "Charlie";

	let mut runtime = runtime::Runtime::new();
	runtime.balances.set_balance(alice, 100);
	runtime.system.inc_block_nubmer();

	runtime.transact(alice, bob, 30);
	runtime.transact(alice, charlie, 20);

	println!("{:?}", runtime);
}
