mod balances;
mod runtime;
mod support;
mod system;

fn main() {
	let alice = "Alice";
	let bob = "Bob";

	let mut runtime = runtime::Runtime::new();
	runtime.balances.set_balance(alice, 100);

	let block_1 = support::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![support::Extrinsic {
			caller: alice,
			call: runtime::RuntimeCall::Balances(balances::Call::BalancesTranster{ to: bob, amount: 10 })
		}],
	};

	runtime.execute_block(block_1).unwrap();

	println!("{:?}", runtime);
}
