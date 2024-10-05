use slip_10::supported_curves::Secp256k1;
use slip_10::{self, ExtendedKeyPair};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let seed = b"16-64 bytes of high entropy";

    let master_key = slip_10::derive_master_key::<Secp256k1>(seed)?;
    let master_key_pair = ExtendedKeyPair::from(master_key);

    let child_key_pair = slip_10::derive_child_key_pair_with_path(
        &master_key_pair,
        [1 + slip_10::H, 10],
    );

    println!("Child Key Pair: {:?}", child_key_pair);

    Ok(())
}
