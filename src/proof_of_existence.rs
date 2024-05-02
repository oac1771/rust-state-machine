use crate::support::{Dispatch, DispatchResult};
use crate::system::Config as SysConfig;
use std::collections::BTreeMap;

pub trait Config: SysConfig {
	type Claim: Ord;
}

#[derive(Debug)]
pub struct Proof<T: Config> {
	claims: BTreeMap<T::Claim, T::AccountId>,
}

pub enum Call<T: Config> {
	CreateClaim { claim: T::Claim },
	RevokeClaim { claim: T::Claim },
}

impl<T: Config> Proof<T> {
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	fn get_claim(&self, claim: &T::Claim) -> Option<&T::AccountId> {
		self.claims.get(claim)
	}

	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Claim) -> DispatchResult {
		if self.claims.contains_key(&claim) {
			return Err("This claim has already been recorded on chain");
		} else {
			self.claims.insert(claim, caller);
		}

		Ok(())
	}

	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Claim) -> DispatchResult {
		let current_owner = self.get_claim(&claim);

		if let Some(&owner) = current_owner {
			if owner == caller {
				self.claims.remove(&claim);
				return Ok(());
			} else {
				Err("Caller did not match claim owner")
			}
		} else {
			Err("Claim did not exist on chain")
		}
	}
}

impl<T: Config> Dispatch for Proof<T> {
	type Call = Call<T>;
	type Caller = T::AccountId;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
		match call {
			Call::CreateClaim { claim } => {
				self.create_claim(caller, claim)?;
			},
			Call::RevokeClaim { claim } => {
				self.revoke_claim(caller, claim)?;
			},
		}

		Ok(())
	}
}
