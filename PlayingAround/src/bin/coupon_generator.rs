use rand::Rng;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Error, Debug)]
enum CouponError {
    #[error("Initials length ({0}) cannot be greater than the total coupon length ({1})")]
    InitialsTooLong(usize, u16),
    #[error("Cannot generate {0} unique coupons with the given length and character set. Maximum possible is {1}")]
    TooManyCoupons(u128, u128),
}

fn coupon_generator(len: u16, number_coupons: u128, initials: &str) -> Result<Vec<String>, CouponError> {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    
    let initials_len = initials.len();
    let code_len = len as usize - initials_len;

    if initials_len > len as usize {
        return Err(CouponError::InitialsTooLong(initials_len, len));
    }

    let max_combinations = (CHARSET.len() as u128).pow(code_len as u32);
    if number_coupons > max_combinations {
        return Err(CouponError::TooManyCoupons(number_coupons, max_combinations));
    }

    let mut rng = rand::thread_rng();
    let mut coupons = HashSet::with_capacity(number_coupons as usize);

    while (coupons.len() as u128) < number_coupons {
        let mut coupon = String::with_capacity(len as usize);
        coupon.push_str(initials);

        for _ in 0..code_len {
            let idx = rng.gen_range(0..CHARSET.len());
            coupon.push(CHARSET[idx] as char);
        }

        coupons.insert(coupon);
    }

    Ok(coupons.into_iter().collect())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let coupons = coupon_generator(10, 2, "")?;
    for coupon in coupons.iter(){
        print!("{}\n",coupon)
    }
    Ok(())
}