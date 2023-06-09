// use rand::{thread_rng, Rng};

// pub fn private_key(p: u64) -> u64 {
//     if p < 1 {
//         panic!("Wrong P!!");
//     }
//     thread_rng().gen_range(2..p)

//     // unimplemented!("Pick a private key greater than 1 and less than {p}")
// }

// pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
//     let mut result = 1;
//     let mut base = g % p;
//     let mut exp = a;
//     while exp > 0 {
//         if exp % 2 == 1 {
//             result = (result * base) % p;
//         }
//         base = (base * base) % p;
//         exp /= 2;
//     }
//     result
// }

// pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
//     public_key(p, b_pub, a)
//     // unimplemented!(
//     //     "Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}"
//     // )
// }

extern crate rand;
use rand::{thread_rng, Rng};

/// Right-to-left modular exponentiation implementation
/// For more information see https://en.wikipedia.org/wiki/Modular_exponentiation
fn modular_exponentiation(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;

    let mut e = exponent;

    let mut b = base;

    while e > 0 {
        if (e & 1) == 1 {
            result = (result * b) % modulus;
        }

        e >>= 1;

        b = (b * b) % modulus;
    }

    result
}

pub fn private_key(p: u64) -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub, a, p)
}
