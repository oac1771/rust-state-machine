use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Balances<AccountId, Balance> {
	pub balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Balances<AccountId, Balance>
where
	AccountId: Ord + Copy,
	Balance: CheckedAdd + CheckedSub + Zero + Copy,
{
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: AccountId, amount: Balance) {
		self.balances.insert(who, amount);
	}

	pub fn balance(&self, who: AccountId) -> Balance {
		*self.balances.get(&who).unwrap_or(&Balance::zero())
	}

	pub fn transfer(
		&mut self,
		caller: AccountId,
		to: AccountId,
		amount: Balance,
	) -> Result<(), &'static str> {
		let caller_balance = self.balance(caller);
		let to_balance = self.balance(to);

		let new_caller_balance = caller_balance
			.checked_sub(&amount)
			.ok_or("Transfering this amount would cause underflow")?;
		let new_to_balance = to_balance
			.checked_add(&amount)
			.ok_or("Transfering this amount would cause overflow")?;

		self.set_balance(caller, new_caller_balance);
		self.set_balance(to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_set_balance() {
		let mut balances = Balances::new();
		let balance_amount = 42;

		assert_eq!(balances.balance("foo"), 0);

		balances.set_balance("foo", balance_amount);

		assert_eq!(balances.balance("foo"), balance_amount);
	}

	#[test]
	fn test_transfer_works() {
		let mut balances = Balances::new();

		balances.set_balance("foo", 100);
		balances.transfer("foo", "bar", 100).unwrap();

		assert_eq!(balances.balance("foo"), 0);
		assert_eq!(balances.balance("bar"), 100);
	}
}
