use udigest::hash;

#[derive(udigest::Digestable)]
struct Person {
    name: String,
    occupation: String,
}

fn main() {
    let alice = Person {
        name: "Alice".into(),
        occupation: "Engineer".into(),
    };

    // Hash the `alice` instance using SHA-256
    let hash_value = hash::<sha2::Sha256>(&alice);

    // Print the resulting hash
    println!("Hash of Alice: {:?}", hash_value);
}