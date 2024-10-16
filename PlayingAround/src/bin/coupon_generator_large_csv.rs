use rand::prelude::*;
//use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use thiserror::Error;
use csv::Writer;
use std::fs::File;

#[derive(Error, Debug)]
enum CouponError {
    #[error("Initials length ({0}) cannot be greater than the total coupon length ({1})")]
    InitialsTooLong(usize, u16),
    #[error("Cannot generate {0} unique coupons with the given length and character set. Maximum possible is {1}")]
    TooManyCoupons(usize, u128),
    #[error("Failed to write to CSV file: {0}")]
    CsvWriteError(#[from] csv::Error),
    #[error("Failed to create file: {0}")]
    FileCreationError(#[from] std::io::Error),
}

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const CHARSET_LEN: usize = CHARSET.len();

lazy_static::lazy_static! {
    static ref CHAR_LOOKUP: [char; 256] = {
        let mut lookup = ['\0'; 256];
        for (i, &b) in CHARSET.iter().enumerate() {
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

fn coupon_generator(len: u16, number_coupons: usize, initials: &str) -> Result<Vec<String>, CouponError> {
    let initial_len = initials.len();
    let code_len = len as usize - initial_len;

    if initial_len > len as usize {
        return Err(CouponError::InitialsTooLong(initial_len, len));
    }

    let max_combinations = (CHARSET_LEN as u128).pow(code_len as u32);
    if number_coupons > max_combinations as usize {
        return Err(CouponError::TooManyCoupons(number_coupons, max_combinations));
    }

    let coupons = Arc::new(parking_lot::Mutex::new(HashSet::with_capacity(number_coupons)));
    let counter = Arc::new(AtomicUsize::new(0));

    rayon::scope(|s| {
        for _ in 0..rayon::current_num_threads() {
            let coupons = Arc::clone(&coupons);
            let counter = Arc::clone(&counter);
            s.spawn(move |_| {
                let mut rng = SmallRng::from_entropy();
                loop {
                    let my_number = counter.fetch_add(1, Ordering::SeqCst);
                    if my_number >= number_coupons {
                        break;
                    }
                    
                    loop {
                        let coupon = generate_coupon(&mut rng, code_len, initials);
                        let mut set = coupons.lock();
                        if set.insert(coupon) {
                            break;
                        }
                    }
                }
            });
        }
    });

    Ok(Arc::try_unwrap(coupons).unwrap().into_inner().into_iter().collect())
}

fn write_coupons_to_csv(coupons: &[String], filename: &str) -> Result<(), CouponError> {
    let file = File::create(filename)?;
    let mut writer = Writer::from_writer(file);

    writer.write_record(&["Coupon"])?;

    for coupon in coupons {
        writer.write_record(&[coupon])?;
    }

    writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    let coupons = coupon_generator(10, 10_000_000, "ABC")?;
    let generation_duration = start.elapsed();

    println!("Generated {} coupons in {:?}", coupons.len(), generation_duration);
    println!("First few coupons: {:?}", &coupons[..5]);

    let csv_start = std::time::Instant::now();
    write_coupons_to_csv(&coupons, "coupons.csv")?;
    let csv_duration = csv_start.elapsed();

    println!("Wrote coupons to CSV in {:?}", csv_duration);
    Ok(())
}
