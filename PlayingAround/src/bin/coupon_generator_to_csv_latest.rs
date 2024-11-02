/// A Rust program to generate unique coupon codes with specified initials and code length.
/// The program generates a specified number of unique coupons, writes them to a CSV file,
/// and uses concurrency for efficient generation.

use rand::prelude::*; // Import random number generation traits and functions
use std::collections::HashSet;
use std::fs::File;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use thiserror::Error;
use csv::Writer;

/// Custom error type for the coupon generation process.
#[derive(Error, Debug)]
enum CouponError {
    /// Error when the initials length exceeds the total coupon length.
    #[error("Initials length ({0}) cannot be greater than the total coupon length ({1})")]
    InitialsTooLong(usize, u16),

    /// Error when the requested number of coupons exceeds the maximum possible combinations.
    #[error(
        "Cannot generate {0} unique coupons with the given length and character set. Maximum possible is {1}"
    )]
    TooManyCoupons(usize, u128),

    /// Error when writing to the CSV file fails.
    #[error("Failed to write to CSV file: {0}")]
    CsvWriteError(#[from] csv::Error),

    /// Error when creating the output file fails.
    #[error("Failed to create file: {0}")]
    FileCreationError(#[from] std::io::Error),
}

/// The character set used for generating the coupon codes.
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// The length of the character set.
const CHARSET_LEN: usize = CHARSET.len();

lazy_static::lazy_static! {
    /// A lookup table for mapping byte values to their corresponding characters.
    /// Initialized once at runtime for efficient character mapping.
    static ref CHAR_LOOKUP: [char; 256] = {
        let mut lookup = ['\0'; 256];
        for &b in CHARSET.iter() {
            lookup[b as usize] = b as char;
        }
        lookup
    };
}

/// Generates a single coupon code with the specified code length and initials.
///
/// # Arguments
///
/// * `rng` - A mutable reference to a random number generator.
/// * `code_len` - The length of the random part of the coupon code.
/// * `initials` - The initials to prefix the coupon code with.
///
/// # Returns
///
/// A `String` representing the generated coupon code.
fn generate_coupon(rng: &mut SmallRng, code_len: usize, initials: &str) -> String {
    // Pre-allocate a string with the required capacity for efficiency
    let mut coupon = String::with_capacity(initials.len() + code_len);
    coupon.push_str(initials); // Add the initials to the coupon code

    // Buffer to hold random bytes
    let mut buffer = vec![0u8; code_len];
    rng.fill_bytes(&mut buffer); // Fill the buffer with random bytes

    // Convert random bytes to characters from CHARSET and append to coupon
    for &byte in buffer.iter() {
        let index = byte as usize % CHARSET_LEN;
        coupon.push(CHAR_LOOKUP[CHARSET[index] as usize]);
    }

    coupon
}

/// Generates a specified number of unique coupon codes.
///
/// # Arguments
///
/// * `len` - The total length of each coupon code (initials + random code).
/// * `number_coupons` - The total number of unique coupons to generate.
/// * `initials` - The initials to prefix each coupon code with.
///
/// # Returns
///
/// A `Result` containing a vector of unique coupon codes or a `CouponError`.
fn coupon_generator(
    len: u16,
    number_coupons: usize,
    initials: &str,
) -> Result<Vec<String>, CouponError> {
    let initial_len = initials.len();
    let code_len = len as usize - initial_len;

    // Check if the initials length exceeds the total coupon length
    if initial_len > len as usize {
        return Err(CouponError::InitialsTooLong(initial_len, len));
    }

    // Calculate the maximum possible combinations based on CHARSET and code length
    let max_combinations = (CHARSET_LEN as u128).pow(code_len as u32);
    if number_coupons > max_combinations as usize {
        return Err(CouponError::TooManyCoupons(number_coupons, max_combinations));
    }

    // Use a thread-safe set to store unique coupons
    let coupons = Arc::new(parking_lot::Mutex::new(HashSet::with_capacity(number_coupons)));
    let counter = Arc::new(AtomicUsize::new(0));

    // Use Rayon for parallel execution
    rayon::scope(|s| {
        for _ in 0..rayon::current_num_threads() {
            let coupons = Arc::clone(&coupons);
            let counter = Arc::clone(&counter);
            s.spawn(move |_| {
                let mut rng = SmallRng::from_entropy();
                loop {
                    // Atomically get the next number to process
                    let my_number = counter.fetch_add(1, Ordering::SeqCst);
                    if my_number >= number_coupons {
                        break;
                    }

                    // Generate unique coupons
                    loop {
                        let coupon = generate_coupon(&mut rng, code_len, initials);
                        let mut set = coupons.lock();
                        if set.insert(coupon) {
                            break; // Break if the coupon is unique
                        }
                    }
                }
            });
        }
    });

    // Collect the generated coupons into a vector
    Ok(Arc::try_unwrap(coupons)
        .unwrap()
        .into_inner()
        .into_iter()
        .collect())
}

/// Writes the list of coupons to a CSV file.
///
/// # Arguments
///
/// * `coupons` - A slice of coupon codes to write to the file.
/// * `filename` - The name of the output CSV file.
///
/// # Returns
///
/// A `Result` indicating success or a `CouponError`.
fn write_coupons_to_csv(coupons: &[String], filename: &str) -> Result<(), CouponError> {
    let file = File::create(filename)?; // Create or overwrite the CSV file
    let mut writer = Writer::from_writer(file);

    writer.write_record(&["Coupon"])?; // Write the header

    // Write each coupon code to the CSV file
    for coupon in coupons {
        writer.write_record(&[coupon])?;
    }

    writer.flush()?; // Ensure all data is written to the file
    Ok(())
}

/// The main entry point of the program.
///
/// Generates the coupons, measures the time taken, and writes them to a CSV file.
///
/// # Returns
///
/// A `Result` indicating success or an error.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start timing the coupon generation
    let start = std::time::Instant::now();

    // Generate coupons with total length 10, 1,000,000 coupons, and initials "LISA"
    let coupons = coupon_generator(10, 1_000_000, "LISA")?;
    let generation_duration = start.elapsed(); // Measure time taken

    println!(
        "Generated {} coupons in {:?}",
        coupons.len(),
        generation_duration
    );
    println!("First few coupons: {:?}", &coupons[..5]); // Display first few coupons

    // Start timing the CSV writing
    let csv_start = std::time::Instant::now();
    write_coupons_to_csv(&coupons, "coupons.csv")?;
    let csv_duration = csv_start.elapsed(); // Measure time taken

    println!("Wrote coupons to CSV in {:?}", csv_duration);
    Ok(())
}
