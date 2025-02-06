## Exploring and Implementing the following under [LFDT-Lockness Repository](https://github.com/LFDT-Lockness)
LFDT-Lockness is an open source advanced cryptography library by Linux Foundation focused on the implementation of multiple cryptographic protocols and techniques. It is designed to support a variety of cryptographic primitives and zero-knowledge proof systems in a secure and efficient manner. This repository builds on its core features and contributes to the implementation of key exchange protocols, encryption schemes, and more.

### 1. Generic-ec: Generic Elliptic Curve Cryptography
- Implemented Elliptic Curve Diffie-Hellman (ECDH) key exchange.

### 2. Pailliar_zk: Zero-Knowledge Proofs for Paillier Encryption
- Implemented basic key generation and auxiliary data for a zero-knowledge proof system using the Paillier encryption scheme.

### 3. Slip-10: Deterministic Key Generation
- Implemented the derivation of a hierarchical deterministic (HD) wallet key pair, including master and child key pairs from a high-entropy seed.

### 4. Udigest: Utilities for Unambiguous Hashing of Structured Data
- Implemented how to create a basic custom data structure and generate a SHA-256 hash using the Udigest crate.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
