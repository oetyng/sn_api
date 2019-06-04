// Copyright 2019 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use structopt::StructOpt;

use safe_cli::{BlsKeyPair, Safe};
use unwrap::unwrap;

// TODO: move these to helper file
use crate::cli::{get_target_location, prompt_user};
use crate::subcommands::keys::create_new_key;

#[derive(StructOpt, Debug)]
pub enum WalletSubCommands {
    #[structopt(name = "insert")]
    /// Insert a spendable balance into a wallet
    Insert {
        /// The source wallet for funds
        payee: String,
        /// The target wallet to store the spendable balance.
        target: Option<String>,
        /// The existing keys safe://xor-url to add to the wallet
        key: Option<String>,
        /// The name to give this wallet
        #[structopt(long = "name")]
        name: String,
        /// Create a Key, allocate test-coins onto it, and add it to the wallet
        #[structopt(long = "test-coins")]
        test_coins: bool,
        /// Preload the key with a coinbalance
        #[structopt(long = "preload")]
        preload: Option<String>,
        /// Set the sub name as default for this public name
        #[structopt(long = "default")]
        default: bool,
    },
    #[structopt(name = "balance")]
    /// Query a new Wallet or PublicKeys CoinBalance
    Balance {},
    #[structopt(name = "check-tx")]
    /// Check the status of a given transaction
    CheckTx {},
    #[structopt(name = "create")]
    /// Create a new Wallet/CoinBalance
    Create {},
    #[structopt(name = "transfer")]
    Transfer {
        /// target wallet
        to: String,

        /// source wallet, or pulled from stdin if not present
        from: Option<String>,
    },
    #[structopt(name = "sweep")]
    /// Move all coins within a wallet to a given balance
    Sweep {
        /// The source wallet for funds
        #[structopt(long = "from")]
        from: String,
        /// The receiving wallet/ballance
        #[structopt(long = "to")]
        to: String,
    },
}

pub fn wallet_commander(
    cmd: Option<WalletSubCommands>,
    target: Option<String>,
    safe: &mut Safe,
) -> Result<(), String> {
    match cmd {
        Some(WalletSubCommands::Create {}) => {
            let xorname = safe.wallet_create();
            println!("Wallet created at XOR name: \"{}\"", xorname);
            // Ok(())
        }
        Some(WalletSubCommands::Balance {}) => {
            let sk =
                String::from("391987fd429b4718a59b165b5799eaae2e56c697eb94670de8886f8fb7387058"); // FIXME: get sk from args or from the account
            let target = get_target_location(target)?;
            let balance = safe.wallet_balance(&target, &sk);
            println!(
                "Wallet at XOR name \"{}\" has a total balance of {} safecoins",
                target, balance
            );
            // Ok(())
        }
        Some(WalletSubCommands::Insert {
            preload,
            payee,
            test_coins,
            target,
            key,
            name,
            default,
        }) => {
            let target = get_target_location(target)?;
            let (xorname, key_pair) = match key {
                Some(linked_key) => {
                    // Get pk payee Key, and prompt user for the corresponding sk
                    let sk = prompt_user(
                        &format!(
                            "Enter secret key corresponding to public key at XOR name \"{}\": ",
                            linked_key
                        ),
                        "Invalid input",
                    );
                    let pk = safe.keys_fetch_pk(&linked_key, &sk);
                    println!(
						"Spendable balance added with name '{}' in wallet located at XOR name \"{}\"",
						name, target
					);
                    (linked_key, Some(BlsKeyPair { pk, sk }))
                }
                None => {
                    let new_key = create_new_key(safe, test_coins, payee, preload, None);
                    println!("New spendable balance generated with name '{}' in wallet located at XOR name \"{}\"", name, target);
                    new_key
                }
            };
            safe.wallet_add(&target, &name, default, &unwrap!(key_pair), &xorname);
            // Ok(())
        }
        Some(WalletSubCommands::Transfer { from, to }) => {
            let xorname = safe.wallet_create();
            println!("Wallet created at XOR name: \"{}\"", xorname);
            // Ok(())
        }
        _ => return Err("Sub-command not supported yet".to_string()),
    };
    Ok(())
}