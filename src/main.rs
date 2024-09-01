use rand::Rng;
use num_bigint::{BigUint, RandBigInt};
use sha2::{Sha256, Digest};

// Parameters for the discrete logarithm problem
struct Params {
    p: BigUint, // A large prime
    g: BigUint, // A generator of the multiplicative group modulo p
}

// User's credentials
struct UserCredentials {
    username: String,
    password: String,
}

// Server's stored data for a user
struct StoredCredentials {
    username: String,
    y: BigUint, // g^x mod p, where x is derived from the password
}

fn main() {
    // Set up system parameters
    let params = generate_params();

    // User registration
    let user = UserCredentials {
        username: "alice".to_string(),
        password: "secret_password".to_string(),
    };
    let stored_creds = register_user(&params, &user);

    // Login attempt
    let login_result = login(&params, &stored_creds, &user);
    println!("Login successful: {}", login_result);

    // Simulated wrong password
    let wrong_user = UserCredentials {
        username: "alice".to_string(),
        password: "wrong_password".to_string(),
    };
    let wrong_login_result = login(&params, &stored_creds, &wrong_user);
    println!("Login with wrong password successful: {}", wrong_login_result);
}

fn generate_params() -> Params {
    // In a real system, these would be carefully chosen and much larger
    Params {
        p: BigUint::from(23u32),
        g: BigUint::from(5u32),
    }
}

fn register_user(params: &Params, user: &UserCredentials) -> StoredCredentials {
    let x = hash_to_int(&user.password);
    let y = params.g.modpow(&x, &params.p);
    
    StoredCredentials {
        username: user.username.clone(),
        y,
    }
}

fn login(params: &Params, stored: &StoredCredentials, user: &UserCredentials) -> bool {
    // Prover (user) generates a random number and computes commitment
    let mut rng = rand::thread_rng();
    let k: BigUint = rng.gen_biguint_below(&(&params.p - 1u32)) + 1u32;
    let r = params.g.modpow(&k, &params.p);

    // Verifier (server) generates a challenge
    let e: BigUint = rng.gen_biguint_below(&(&params.p - 1u32)) + 1u32;

    // Prover computes the response
    let x = hash_to_int(&user.password);
    let s = (&k + &e * &x) % (&params.p - 1u32);

    // Verifier checks the proof
    let left = params.g.modpow(&s, &params.p);
    let right = (r * stored.y.modpow(&e, &params.p)) % &params.p;

    left == right
}

fn hash_to_int(input: &str) -> BigUint {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    BigUint::from_bytes_be(&result)
}