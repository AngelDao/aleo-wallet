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

use aleo_account::Account as AccountNative;

use colored::*;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub enum Account {
    /// Generates a new Aleo account
    New {
        /// Seed the RNG with a numeric value
        #[structopt(short = "s", long)]
        seed: Option<u64>,
    },
}

impl Account {
    pub fn parse(self) -> anyhow::Result<String> {
        match self {
            Self::New { seed } => {
                // Sample a new Aleo account.
                let account = match seed {
                    Some(seed) => AccountNative::new(&mut ChaChaRng::seed_from_u64(seed)),
                    None => AccountNative::new(&mut rand::thread_rng()),
                };

                // Print the new Aleo account.
                let mut output = format!("\n {:>12}  {}\n", "Private Key".cyan().bold(), account.private_key());
                output += &format!(" {:>12}  {}\n", "View Key".cyan().bold(), account.view_key());
                output += &format!(" {:>12}  {}\n", "Address".cyan().bold(), account.address());

                Ok(output)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::Account;
    use colored::Colorize;

    #[test]
    fn test_new() {
        for _ in 0..3 {
            let account = Account::New { seed: None };
            assert!(account.parse().is_ok());
        }
    }

    #[test]
    fn test_new_seeded() {
        let seed = Some(1231275789u64);
        let mut expected = format!(
            "\n {:>12}  {}\n",
            "Private Key".cyan().bold(),
            "APrivateKey1zkp8cC4jgHEBnbtu3xxs1Ndja2EMizcvTRDq5Nikdkukg1p"
        );
        expected += &format!(
            " {:>12}  {}\n",
            "View Key".cyan().bold(),
            "AViewKey1iAf6a7fv6ELA4ECwAth1hDNUJJNNoWNThmREjpybqder"
        );
        expected += &format!(
            " {:>12}  {}\n",
            "Address".cyan().bold(),
            "aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah"
        );
        let account = Account::New { seed };
        let actual = account.parse().unwrap();
        assert_eq!(expected, actual);
    }
}
