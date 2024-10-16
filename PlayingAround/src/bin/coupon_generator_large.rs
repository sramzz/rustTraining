use rand::prelude::*;
//use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::Mutex;
use thiserror::Error;

#[derive(Error, Debug)]
enum CouponError {
    #[error("Initials length ({0}) cannot be greater than the total coupon length ({1})")]
    InitialsTooLong(usize, u16),
    #[error("Cannot generate {0} unique coupons with the given length and character set. Maximum possible is {1}")]
    TooManyCoupons(u128, u128),
}

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const CHARSET_LEN: usize = CHARSET.len();

// Lookup table for faster byte-to-char conversion
lazy_static::lazy_static! {
    static ref CHAR_LOOKUP: [char; 256] = {
        let mut lookup = ['\0'; 256];
        for (_i, &b) in CHARSET.iter().enumerate() {
            lookup[b as usize] = b as char;
        }
        lookup
    };
}

fn generate_coupon(rng: &mut SmallRng, code_len: usize, initials: &str) -> String {
    let mut coupon = String::with_capacity(initials.len() + code_len);
    coupon.push_str(initials);

    let mut buffer = vec![0u8; code_len];
    rng.fill_bytes(&mut buffer);

    for &byte in buffer.iter() {
        let index = byte as usize % CHARSET_LEN;
        coupon.push(CHAR_LOOKUP[CHARSET[index] as usize]);
    }

    coupon
}

fn coupon_generator(len: u16, number_coupons: u128, initials: &str) -> Result<Vec<String>, CouponError> {
    let initial_len = initials.len();
    let code_len = len as usize - initial_len;

    if initial_len > len as usize {
        return Err(CouponError::InitialsTooLong(initial_len, len));
    }

    let max_combinations = (CHARSET_LEN as u128).pow(code_len as u32);
    if number_coupons > max_combinations {
        return Err(CouponError::TooManyCoupons(number_coupons, max_combinations));
    }

    let coupons = Mutex::new(HashSet::with_capacity(number_coupons as usize));
    let target_count = number_coupons;

    rayon::scope(|s| {
        for _ in 0..rayon::current_num_threads() {
            s.spawn(|_| {
                let mut rng = SmallRng::from_entropy();
                loop {
                    let coupon = generate_coupon(&mut rng, code_len, initials);
                    let mut set = coupons.lock().unwrap();
                    if set.insert(coupon) {
                        if set.len() as u128 >= target_count {
                            break;
                        }
                    }
                }
            });
        }
    });

    Ok(coupons.into_inner().unwrap().into_iter().collect())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    let coupons = coupon_generator(10, 10_000_000, "ABC")?;
    let duration = start.elapsed();

    println!("Generated {} coupons in {:?}", coupons.len(), duration);
    println!("First few coupons: {:?}", &coupons[..5]);
    Ok(())
}