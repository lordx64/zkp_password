# Zero-Knowledge Proof Password Verification

This Rust project demonstrates a zero-knowledge proof (ZKP) system for password verification using the Schnorr protocol. The system allows a user to prove they know a password without revealing the password itself, enhancing security in authentication processes.

## Features

- Zero-knowledge password verification
- User registration and login simulation
- Based on the discrete logarithm problem
- Demonstrates core ZKP concepts: commitment, challenge, response, and verification

## Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)
- Cargo (Rust's package manager, included with Rust)

## Dependencies

This project uses the following external crates:

- `rand`: For random number generation
- `num-bigint`: For big integer arithmetic
- `sha2`: For SHA-256 hashing

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/lordx64/zkp_password.git
   cd zkp-password-verification
   ```

2. Build the project:
   ```
   cargo build
   ```

## Usage

Run the program with:
