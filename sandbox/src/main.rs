
use aleo_account::{Account as AccountNative, PrivateKey};

use rand::{rngs::StdRng, SeedableRng};
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use regex::Regex;


fn main() {
    println!("testing regex");
    let test_target = "test";
    let van_acct = Account::new_vanity(test_target);
}

pub struct Account {
    pub(crate) account: AccountNative,
}

impl Account {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {

        let rng = &mut StdRng::from_entropy();
        Self {
            account: AccountNative::new(rng),
        }
    }

    fn new_vanity(target: &str) -> Self {
        let rng = &mut StdRng::from_entropy();
        let rgx = Regex::new(format!("^{target}").as_str()).unwrap();
        let mut search_status = false;
        let trial = AccountNative::new(rng);

        while  !search_status {
            let trial = AccountNative::new(rng);
            println!("trial {:?}", trial.address().to_string());
            let trial_addr = &trial.address().to_string();
            let candidate = rgx.find(trial_addr);
            // if target == (rgx.find(&trial.address().to_string()).unwrap().as_str()) {
            let search_status = match candidate {
                Some(str) => {search_status = true},
                None => (),
            };


        }
        Self {
            account: trial,
        }


    }

    pub fn from_private_key(private_key: &str) -> Self {
        let private_key = PrivateKey::from_str(private_key).unwrap();

        Self {
            account: AccountNative::from(private_key),
        }
    }

    pub fn to_private_key(&self) -> String {
        self.account.private_key().to_string()
    }

    pub fn to_view_key(&self) -> String {
        self.account.view_key().to_string()
    }

    pub fn to_address(&self) -> String {
        self.account.address().to_string()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     use wasm_bindgen_test::*;
//
//     const ALEO_PRIVAT_KEY: &str = "APrivateKey1zkp8cC4jgHEBnbtu3xxs1Ndja2EMizcvTRDq5Nikdkukg1p";
//     const ALEO_VIEW_KEY: &str = "AViewKey1iAf6a7fv6ELA4ECwAth1hDNUJJNNoWNThmREjpybqder";
//     const ALEO_ADDRESS: &str = "aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah";
//
//     pub fn from_private_key_test() {
//         let account = Account::from_private_key(ALEO_PRIVATE_KEY);
//
//         println!("{} == {}", ALEO_PRIVATE_KEY, account.account.private_key().to_string());
//         assert_eq!(ALEO_PRIVATE_KEY, account.account.private_key().to_string());
//
//         println!("{} == {}", ALEO_VIEW_KEY, account.account.view_key().to_string());
//         assert_eq!(ALEO_VIEW_KEY, account.account.view_key().to_string());
//
//         println!("{} == {}", ALEO_ADDRESS, account.account.address().to_string());
//         assert_eq!(ALEO_ADDRESS, account.account.address().to_string());
//     }
//
//     pub fn to_private_key_test() {
//         let account = Account::from_private_key(ALEO_PRIVATE_KEY);
//
//         println!("{} == {}", ALEO_PRIVATE_KEY, account.to_private_key());
//         assert_eq!(ALEO_PRIVATE_KEY, account.to_private_key());
//     }
//
//     pub fn to_view_key_test() {
//         let account = Account::from_private_key(ALEO_PRIVATE_KEY);
//
//         println!("{} == {}", ALEO_VIEW_KEY, account.to_view_key());
//         assert_eq!(ALEO_VIEW_KEY, account.to_view_key());
//     }
//
//     pub fn to_address_test() {
//         let account = Account::from_private_key(ALEO_PRIVATE_KEY);
//
//         println!("{} == {}", ALEO_ADDRESS, account.to_address());
//         assert_eq!(ALEO_ADDRESS, account.to_address());
//     }
// }
