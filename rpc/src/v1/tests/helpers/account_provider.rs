// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use std::sync::RwLock;
use std::collections::HashMap;
use std::io;
use util::hash::{Address, H256};
use util::crypto::{Secret, Signature};
use util::keys::store::{AccountProvider, SigningError, EncryptedHashMapError};

/// Account mock.
#[derive(Clone)]
pub struct TestAccount {
	/// True if account is unlocked.
	pub unlocked: bool,
	/// Account's password.
	pub password: String,
}

impl TestAccount {
	pub fn new(password: &str) -> Self {
		TestAccount {
			unlocked: false,
			password: password.to_owned(),
		}
	}
}

/// Test account provider.
pub struct TestAccountProvider {
	accounts: RwLock<HashMap<Address, TestAccount>>,
}

impl TestAccountProvider {
	/// Basic constructor.
	pub fn new(accounts: HashMap<Address, TestAccount>) -> Self {
		TestAccountProvider {
			accounts: RwLock::new(accounts),
		}
	}
}

impl AccountProvider for TestAccountProvider {
	fn accounts(&self) -> Result<Vec<Address>, io::Error> {
		Ok(self.accounts.read().unwrap().keys().cloned().collect())
	}

	fn unlock_account(&self, account: &Address, pass: &str) -> Result<(), EncryptedHashMapError> {
		match self.accounts.write().unwrap().get_mut(account) {
			Some(ref mut acc) if acc.password == pass => {
				acc.unlocked = true;
				Ok(())
			},
			Some(_) => Err(EncryptedHashMapError::InvalidPassword),
			None => Err(EncryptedHashMapError::UnknownIdentifier),
		}
	}

	fn new_account(&self, _pass: &str) -> Result<Address, io::Error> {
		unimplemented!()
	}
	fn account_secret(&self, _account: &Address) -> Result<Secret, SigningError> {
		unimplemented!()
	}

	fn sign(&self, _account: &Address, _message: &H256) -> Result<Signature, SigningError> {
		unimplemented!()
	}

}
