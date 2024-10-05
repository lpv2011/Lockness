use generic_ec::{Point, SecretScalar, curves::Secp256k1};
use rand::thread_rng;
use sha2::{Sha256, Digest};

fn generate_keypair() -> (SecretScalar<Secp256k1>, Point<Secp256k1>) {
    let private_key = SecretScalar::<Secp256k1>::random(&mut thread_rng());
    let public_key = Point::generator() * &private_key;
    (private_key, public_key)
}

fn hash_to_secret(point: &Point<Secp256k1>) -> Vec<u8> {
    let point_bytes = point.to_bytes(true);
    let mut hasher = Sha256::new();
    hasher.update(point_bytes.as_bytes());
    hasher.finalize().to_vec()
}

fn ecdh_key_exchange() {
    // Alice keypair
    let (alice_private_key, alice_public_key) = generate_keypair();

    // Bob keypair
    let (bob_private_key, bob_public_key) = generate_keypair();

    // Alice shared secret: Bob's public key * Alice's private key
    let alice_shared_secret_point = bob_public_key * &alice_private_key;

    // Bob shared secret: Alice's public key * Bob's private key
    let bob_shared_secret_point = alice_public_key * &bob_private_key;

    // Hashing shared point
    let shared_secret = hash_to_secret(&alice_shared_secret_point);

    if alice_shared_secret_point == bob_shared_secret_point {
        println!("ECDH key exchange successful!");
        println!("Shared Secret: {:?}", shared_secret);
    }
    
}

fn main() {
    ecdh_key_exchange();
}