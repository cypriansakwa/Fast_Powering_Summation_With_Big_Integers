use num_bigint::BigUint;
use num_traits::{One, Zero};

fn mod_exp(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    let mut result = BigUint::one();
    let mut base = base % modulus;
    let mut exp = exp.clone();

    while exp > BigUint::zero() {
        if &exp % 2u32 == BigUint::one() {
            result = (result * &base) % modulus;
        }
        exp >>= 1;
        base = (&base * &base) % modulus;
    }

    result
}

fn main() {
    let base1 = BigUint::parse_bytes(b"207827272827282873738336262", 10).unwrap();
    let exp1 = BigUint::parse_bytes(b"6267776557776667764444", 10).unwrap();
    let base2 = BigUint::parse_bytes(b"5546474747476647447647333", 10).unwrap();
    let exp2 = BigUint::parse_bytes(b"991111111111111111333422223", 10).unwrap();
    let modulus = BigUint::parse_bytes(b"1789289289289282928226262", 10).unwrap();

    let result1 = mod_exp(&base1, &exp1, &modulus);
    let result2 = mod_exp(&base2, &exp2, &modulus);

    let final_result = (result1 + result2) % modulus;

    println!("The result of (20782727282728287373833^626777655777666776 + 5546474747476647447647^9911111111111111113334) % 17892892892892829282 is {}", final_result);
}


