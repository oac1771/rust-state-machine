use std::collections::BTreeMap;

pub struct Pallet {
	pub balances: BTreeMap<String, u128>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &str, amount: u128) {
		self.balances.insert(who.to_string(), amount);
	}

	pub fn balance(&self, who: &str) -> u128 {
		*self.balances.get(who).unwrap_or(&0)
	}

	pub fn transfer(
		&mut self,
		caller: &str,
		to: &str,
		amount: u128,
	) -> Result<(), &'static str> {
        let caller_balance = self.balance(caller);
        let to_balance = self.balance(to);

        let new_caller_balance = caller_balance.checked_sub(amount).ok_or("Transfering this amount would cause underflow")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("Transfering this amount would cause overflow")?;

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
		let mut pallet = Pallet::new();
		let balance_amount = 42;

		assert_eq!(pallet.balance("foo"), 0);

		pallet.set_balance("foo", balance_amount);

		assert_eq!(pallet.balance("foo"), balance_amount);
	}

    #[test]
    fn test_transfer_works() {
		let mut pallet = Pallet::new();

        pallet.set_balance("foo", 100);
        pallet.transfer("foo", "bar", 100).unwrap();

        assert_eq!(pallet.balance("foo"), 0);
        assert_eq!(pallet.balance("bar"), 10);

    }
}
