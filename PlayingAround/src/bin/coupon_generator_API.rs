//! Coupon generation and CSV writing module.
//!
//! This module provides functionality to generate unique coupons and write them to a CSV format.
//! It's designed to be efficient and suitable for use in a web API context.

use csv::Writer;
use futures::stream::Stream;
use rand::prelude::*;
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use thiserror::Error;
use tokio::io::AsyncWrite;

/// Errors that can occur during coupon generation and CSV writing.
#[derive(Error, Debug)]
pub enum CouponError {
    /// Occurs when the initials length exceeds the total coupon length.
    #[error("Initials length ({0}) cannot be greater than the total coupon length ({1})")]
    InitialsTooLong(usize, u16),

    /// Occurs when the requested number of coupons exceeds the possible unique combinations.
    #[error("Cannot generate {0} unique coupons with the given length and character set. Maximum possible is {1}")]
    TooManyCoupons(usize, u128),

    /// Wraps CSV writing errors.
    #[error("Failed to write CSV data: {0}")]
    CsvWriteError(#[from] csv::Error),

    /// Wraps I/O errors.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

/// The character set used for generating coupons.
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
/// The length of the character set.
const CHARSET_LEN: usize = CHARSET.len();

lazy_static::lazy_static! {
    /// A lookup table for fast character conversion.
    static ref CHAR_LOOKUP: [char; 256] = {
        let mut lookup = ['\0'; 256];
        for (_i, &b) in CHARSET.iter().enumerate() {
            lookup[b as usize] = b as char;
        }
        lookup
    };
}

/// Generates a single coupon.
///
/// This function is used internally by the coupon generator.
///
/// # Arguments
///
/// * `rng` - A mutable reference to a `SmallRng` for random number generation.
/// * `code_len` - The length of the random part of the coupon.
/// * `initials` - The initials to prepend to the coupon.
///
/// # Returns
///
/// A `String` containing the generated coupon.
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

/// Generates a stream of unique coupons.
///
/// This function returns a `Stream` that yields unique coupons. It's designed to be memory-efficient
/// and suitable for use in a web API context.
///
/// # Arguments
///
/// * `len` - The total length of each coupon.
/// * `number_coupons` - The number of unique coupons to generate.
/// * `initials` - The initials to prepend to each coupon.
///
/// # Returns
///
/// A `Result` containing either a `Stream` of `Result<String, CouponError>` or a `CouponError`.
///
/// # Errors
///
/// Returns `CouponError::InitialsTooLong` if the initials are longer than the specified coupon length.
/// Returns `CouponError::TooManyCoupons` if the requested number of coupons exceeds the possible unique combinations.
pub fn coupon_generator(
    len: u16,
    number_coupons: usize,
    initials: &str,
) -> Result<impl Stream<Item = Result<String, CouponError>>, CouponError> {
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
    let initials = initials.to_string();

    Ok(futures::stream::unfold(
        (coupons, counter, initials, code_len, number_coupons),
        move |(coupons, counter, initials, code_len, number_coupons)| {
            async move {
                if counter.load(Ordering::SeqCst) >= number_coupons {
                    None
                } else {
                    let mut rng = SmallRng::from_entropy();
                    let coupon = loop {
                        let new_coupon = generate_coupon(&mut rng, code_len, &initials);
                        let mut set = coupons.lock();
                        if set.insert(new_coupon.clone()) {
                            break new_coupon;
                        }
                    };
                    counter.fetch_add(1, Ordering::SeqCst);
                    Some((Ok(coupon), (coupons, counter, initials, code_len, number_coupons)))
                }
            }
        },
    ))
}

/// Writes coupons to a CSV format.
///
/// This function takes a stream of coupons and writes them to the provided `AsyncWrite` in CSV format.
///
/// # Arguments
///
/// * `writer` - An `AsyncWrite` to which the CSV data will be written.
/// * `coupons` - A `Stream` of `Result<String, CouponError>` representing the coupons to be written.
///
/// # Returns
///
/// A `Result<(), CouponError>` indicating success or failure of the operation.
///
/// # Errors
///
/// This function will return an error if there are issues writing to the CSV or if the input stream yields an error.
pub async fn write_coupons_to_csv<W: AsyncWrite + Unpin>(
    writer: W,
    coupons: impl Stream<Item = Result<String, CouponError>>,
) -> Result<(), CouponError> {
    use tokio::io::AsyncWriteExt;

    let mut csv_writer = csv_async::AsyncWriter::from_writer(writer);
    csv_writer.write_record(&["Coupon"]).await?;

    tokio::pin!(coupons);
    while let Some(coupon_result) = coupons.next().await {
        let coupon = coupon_result?;
        csv_writer.write_record(&[&coupon]).await?;
    }

    csv_writer.flush().await?;
    Ok(())
}

// Example usage in an API context (using actix-web):
//
// ```
// #[get("/generate_coupons")]
// async fn generate_coupons(
//     query: web::Query<CouponParams>,
// ) -> Result<HttpResponse, actix_web::Error> {
//     let coupons = coupon_generator(query.length, query.count, &query.initials)?;
//     let mut buffer = Vec::new();
//     write_coupons_to_csv(&mut buffer, coupons).await?;
//     Ok(HttpResponse::Ok()
//         .content_type("text/csv")
//         .body(buffer))
// }
// ```