use anyhow::Context;
use clap::{arg, command};
use rand::{rngs::StdRng, SeedableRng};
use std::str::FromStr;

use snarkvm_n2::account::Address as NativeAddressN2;
use snarkvm_n2::account::PrivateKey as NativePrivateKeyN2;
use snarkvm_n2::network::testnet2::Testnet2;

type PrivateKeyN2 = NativePrivateKeyN2<Testnet2>;
type AddressN2 = NativeAddressN2<Testnet2>;

fn main() -> anyhow::Result<()> {
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!(
                --private_key <PRIVATE_KEY> "Your Testnet 2 PrivateKey"
            ).long("private-key")
            .required(true),
        )
        .arg(
            arg!(
                --message <MESSAGE> "Message to sign"
            )
            .required(true),
        )
        .get_matches();

    let raw_sk = matches.get_one::<String>("private_key").unwrap();
    let skn2 = PrivateKeyN2::from_str(raw_sk.as_str())
        .with_context(|| format!("Failed to parse private key: {}", raw_sk))?;
    let addrn2 = AddressN2::try_from(&skn2)?.to_string();
    let message = matches.get_one::<String>("message").unwrap();

    let sig = skn2.sign(message.as_bytes(), &mut StdRng::from_entropy())?;

    println!("");
    println!("Message ({} bytes):\n{}\n", message.len(), message);
    println!("Testnet 2 Address:\n{}\n", addrn2);
    println!("Testnet 2 Signature:\n{}\n", sig);

    Ok(())
}
