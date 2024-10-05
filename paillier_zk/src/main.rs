use anyhow::{Context, Result};
use rug::{Complete, Integer};
use paillier_zk::IntegerExt;
use serde::Serialize;

fn main() -> Result<()> {
    let mut rng = rand_core::OsRng;

    {
        let p = generate_blum_prime(&mut rng, 1024);
        let q = generate_blum_prime(&mut rng, 1024);
        let n = (&p * &q).complete();

        let (s, t) = {
            let phi_n = (p.clone() - 1u8) * (q.clone() - 1u8);
            let r = Integer::gen_invertible(&n, &mut rng);
            let lambda = phi_n.random_below(&mut fast_paillier::utils::external_rand(&mut rng));

            let t = r.square().modulo(&n);
            let s = t.pow_mod_ref(&lambda, &n).unwrap().into();

            (s, t)
        };

        let aux = paillier_zk::paillier_encryption_in_range::Aux {
            s,
            t,
            rsa_modulo: n,
            multiexp: None,
            crt: None,
        };

        let aux_serializable = AuxSerializable {
            s: aux.s,
            t: aux.t,
            rsa_modulo: aux.rsa_modulo,
        };

        let aux_json = serde_json::to_vec_pretty(&aux_serializable).context("serialize aux")?;
        std::fs::write("./test-data/verifier_aux.json", aux_json).context("save aux")?;
    }

    generate_paillier_key(
        &mut rng,
        Some("./test-data/prover_decryption_key.json".as_ref()),
        Some("./test-data/prover_encryption_key.json".as_ref()),
    )?;
    generate_paillier_key(
        &mut rng,
        None,
        Some("./test-data/someone_encryption_key0.json".as_ref()),
    )?;
    generate_paillier_key(
        &mut rng,
        None,
        Some("./test-data/someone_encryption_key1.json".as_ref()),
    )?;

    Ok(())
}

fn generate_paillier_key(
    rng: &mut impl rand_core::RngCore,
    output_dk: Option<&std::path::Path>,
    output_ek: Option<&std::path::Path>,
) -> anyhow::Result<()> {

    let p = generate_blum_prime(rng, 1536);
    let q = generate_blum_prime(rng, 1536);

    let dk: fast_paillier::DecryptionKey =
        fast_paillier::DecryptionKey::from_primes(p, q).context("generated p, q are invalid")?;
    let ek = dk.encryption_key();

    if let Some(path) = output_dk {
        let dk_json = serde_json::to_vec_pretty(&dk).context("serialize decryption key")?;
        std::fs::write(path, dk_json).context("save decryption key")?;
    }

    if let Some(path) = output_ek {
        let ek_json = serde_json::to_vec_pretty(&ek).context("serialize encryption key")?;
        std::fs::write(path, ek_json).context("save encryption key")?;
    }

    Ok(())
}

fn generate_blum_prime(rng: &mut impl rand_core::RngCore, bits_size: u32) -> Integer {
    loop {
        let mut n: Integer =
            Integer::random_bits(bits_size, &mut fast_paillier::utils::external_rand(rng)).into();
        n.set_bit(bits_size - 1, true);
        n.next_prime_mut();
        if n.mod_u(4) == 3 {
            break n;
        }
    }
}

#[derive(Serialize)]
struct AuxSerializable {
    s: Integer,
    t: Integer,
    rsa_modulo: Integer,
}
