use std::collections::BTreeMap;

type AccountId = String;
type Balance = u128;

#[derive(Debug)]
pub struct Balances {
	pub balances: BTreeMap<AccountId, Balance>,
}

impl Balances {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &str, amount: Balance) {
		self.balances.insert(who.to_string(), amount);
	}

	pub fn balance(&self, who: &str) -> Balance {
		*self.balances.get(who).unwrap_or(&0)
	}

	pub fn transfer(&mut self, caller: &str, to: &str, amount: Balance) -> Result<(), &'static str> {
		let caller_balance = self.balance(caller);
		let to_balance = self.balance(to);

		let new_caller_balance = caller_balance
			.checked_sub(amount)
			.ok_or("Transfering this amount would cause underflow")?;
		let new_to_balance = to_balance
			.checked_add(amount)
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
