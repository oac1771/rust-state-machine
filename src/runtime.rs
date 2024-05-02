use crate::balances;
use crate::proof_of_existence;
use crate::support;
use crate::support::Dispatch;
use crate::system;

mod types {

	use super::*;

	pub type AccountId = &'static str;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;

	pub type Extrinsic = support::Extrinsic<AccountId, RuntimeCall>;
	pub type Header = support::Header<BlockNumber>;
	pub type Block = support::Block<Header, Extrinsic>;

	pub type Content = &'static str;
}

#[derive(Debug)]
pub struct Runtime {
	pub system: system::System<Self>,
	pub balances: balances::Balances<Self>,
	pub proof_of_existence: proof_of_existence::Proof<Self>,
}

pub enum RuntimeCall {
	Balances(balances::Call<Runtime>),
	PoE(proof_of_existence::Call<Runtime>),
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
	type Claim = types::Content;
}

impl Runtime {
	pub fn new() -> Self {
		Self {
			system: system::System::new(),
			balances: balances::Balances::new(),
			proof_of_existence: proof_of_existence::Proof::new(),
		}
	}

	pub fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		self.system.inc_block_nubmer();

		if block.header.block_number != self.system.block_number() {
			return Err("Block header does not match current block number");
		}

		for support::Extrinsic { caller, call } in block.extrinsics.into_iter() {
			match call {
				RuntimeCall::Balances(inner_call) => {
					self.balances.dispatch(caller, inner_call)?;
				},
				RuntimeCall::PoE(inner_call) => {
					self.proof_of_existence.dispatch(caller, inner_call)?;
				},
			}
			self.system.inc_nonce(caller);
		}
		Ok(())
	}
}
