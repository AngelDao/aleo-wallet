// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Aleo library.

// The Aleo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.

use aleo_account::{Account as AccountNative, PrivateKey};

use rand::{rngs::StdRng, SeedableRng};
use regex::Regex;
use std::str::FromStr;
use std::time::SystemTime;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Account {
    pub(crate) account: AccountNative,
    pub iter_count: u64,
    pub iter_time: u64,
}

#[wasm_bindgen]
impl Account {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();

        let rng = &mut StdRng::from_entropy();
        Self {
            account: AccountNative::new(rng),
            iter_time: 0,
            iter_count: 0,
        }
    }

    #[wasm_bindgen]
    pub fn new_vanity(target: &str) -> Self {
        let rng = &mut StdRng::from_entropy();
        let rgx = Regex::new(format!("^(aleo1{})", target).as_str()).unwrap();
        let mut search_status = false;
        let trial = AccountNative::new(rng);
        let mut iter_count = 0;
        let now = SystemTime::now();

        while !search_status {
            iter_count += 1;
            let trial = AccountNative::new(rng);
            // println!("trial {:?}", trial.address().to_string());
            // println!("aleo1wqzxnzthjqcax0knf83mv27s89gs33xj99c6rhqpuvyspvp765rq5pjnlv");
            let trial_addr = &trial.address().to_string();
            // let trial_addr = "aleo1bqzxnzthjqcax0knf83mv27s89gs33xj99c6rhqpuvyspvp765rq5pjnlv";
            let candidate = rgx.is_match(trial_addr);
            // if target == (rgx.find(&trial.address().to_string()).unwrap().as_str()) {
            if candidate {
                // println!("is canidate");
                // println!("trial addr {}", trial_addr);
                // println!("iteractions to create {}", iter);
                // println!("time taken {:#?}", now.elapsed().unwrap().as_millis());
                search_status = true;
            }
        }
        Self {
            account: trial,
            iter_count,
            iter_time: now.elapsed().unwrap().as_millis() as u64,
        }
    }

    #[wasm_bindgen]
    pub fn from_private_key(private_key: &str) -> Self {
        let private_key = PrivateKey::from_str(private_key).unwrap();

        Self {
            account: AccountNative::from(private_key),
            iter_time: 0,
            iter_count: 0,
        }
    }

    #[wasm_bindgen]
    pub fn to_private_key(&self) -> String {
        self.account.private_key().to_string()
    }

    #[wasm_bindgen]
    pub fn to_view_key(&self) -> String {
        self.account.view_key().to_string()
    }

    #[wasm_bindgen]
    pub fn to_address(&self) -> String {
        self.account.address().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;

    const ALEO_PRIVATE_KEY: &str = "APrivateKey1zkp8cC4jgHEBnbtu3xxs1Ndja2EMizcvTRDq5Nikdkukg1p";
    const ALEO_VIEW_KEY: &str = "AViewKey1iAf6a7fv6ELA4ECwAth1hDNUJJNNoWNThmREjpybqder";
    const ALEO_ADDRESS: &str = "aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah";

    #[wasm_bindgen_test]
    pub fn from_private_key_test() {
        let account = Account::from_private_key(ALEO_PRIVATE_KEY);

        println!("{} == {}", ALEO_PRIVATE_KEY, account.account.private_key().to_string());
        assert_eq!(ALEO_PRIVATE_KEY, account.account.private_key().to_string());

        println!("{} == {}", ALEO_VIEW_KEY, account.account.view_key().to_string());
        assert_eq!(ALEO_VIEW_KEY, account.account.view_key().to_string());

        println!("{} == {}", ALEO_ADDRESS, account.account.address().to_string());
        assert_eq!(ALEO_ADDRESS, account.account.address().to_string());
    }

    #[wasm_bindgen_test]
    pub fn to_private_key_test() {
        let account = Account::from_private_key(ALEO_PRIVATE_KEY);

        println!("{} == {}", ALEO_PRIVATE_KEY, account.to_private_key());
        assert_eq!(ALEO_PRIVATE_KEY, account.to_private_key());
    }

    #[wasm_bindgen_test]
    pub fn to_view_key_test() {
        let account = Account::from_private_key(ALEO_PRIVATE_KEY);

        println!("{} == {}", ALEO_VIEW_KEY, account.to_view_key());
        assert_eq!(ALEO_VIEW_KEY, account.to_view_key());
    }

    #[wasm_bindgen_test]
    pub fn to_address_test() {
        let account = Account::from_private_key(ALEO_PRIVATE_KEY);

        println!("{} == {}", ALEO_ADDRESS, account.to_address());
        assert_eq!(ALEO_ADDRESS, account.to_address());
    }
}
