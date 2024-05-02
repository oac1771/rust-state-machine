mod balances;
mod proof_of_existence;
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
			call: runtime::RuntimeCall::Balances(balances::Call::BalancesTranster {
				to: bob,
				amount: 10,
			}),
		}],
	};

	let block_2 = support::Block {
		header: support::Header { block_number: 2 },
		extrinsics: vec![support::Extrinsic {
			caller: alice,
			call: runtime::RuntimeCall::PoE(proof_of_existence::Call::CreateClaim { claim: "alices claim" }),
		}],
	};

	let block_3 = support::Block {
		header: support::Header { block_number: 3 },
		extrinsics: vec![support::Extrinsic {
			caller: alice,
			call: runtime::RuntimeCall::PoE(proof_of_existence::Call::RevokeClaim { claim: "alices claim" }),
		}],
	};

	runtime.execute_block(block_1).unwrap();
	runtime.execute_block(block_2).unwrap();
	runtime.execute_block(block_3).unwrap();

	println!("{:?}", runtime);
}
