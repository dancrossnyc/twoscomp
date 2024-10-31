//! Demonstrate 2's compliment integer
//!
//! "2's Compliment" is a representation for integers such that,
//! for any integer k of width n, k's negative "-k" is its
//! compliment modulo 2^n.  That is, for any `k`, `-k` is the
//! number so that $k + (-k) == 2^n$.  Another way to look at
//! this is that $(k + (-k)) mod 2^n == 0$.
//!
//! Several desirable properties fall out of this definition:
//!
//! 1. Zero has a single representation.  Since the compliment
//!    of 0 is 2^n, -0 is simply 0.
//! 2. The same arithmetic logic for arithmetic may be used for
//!    both signed and unsigned arithmetic.
//!
//! Note that there is one odd case; the most negative number
//! has no positive compliment.

use std::num::ParseIntError;

fn parse_num(num: &str) -> Result<u128, ParseIntError> {
    let pos = !num.starts_with('-');
    let (radix, numstr) = match &num[if pos { 0 } else { 1 }..] {
        "0" => (10, "0"),
        s if s.starts_with("0x") || s.starts_with("0X") => (16, &s[2..]),
        s if s.starts_with("0t") || s.starts_with("0T") => (10, &s[2..]),
        s if s.starts_with("0b") || s.starts_with("0B") => (2, &s[2..]),
        s if s.starts_with("0") => (8, &s[0..]),
        s => (10, s),
    };
    let num = u128::from_str_radix(numstr, radix)?;
    Ok(if pos { num } else { 0u128.wrapping_sub(num) })
}

fn signextend(n: u128, nbits: usize) -> u128 {
    let mask = !0u128 >> (128 - nbits);
    let neg = (n >> (nbits - 1)) & 0b1 == 1;
    if neg {
        n | !mask
    } else {
        n & mask
    }
}

fn twoscomp(n: u128) -> u128 {
    let onescomp = !n;
    onescomp.wrapping_add(1)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: twoscomp bits num");
        std::process::exit(1);
    }
    let nbitstr = &args[1];
    let nbits = nbitstr.parse::<usize>().unwrap_or_else(|e| {
        eprintln!("twoscomp: failed to parse nbits {nbitstr}: {e:?}");
        std::process::exit(1);
    });
    if !nbits.is_power_of_two() {
        eprintln!("twoscomp: number of bits not a power of two: {nbits}");
        std::process::exit(1);
    }
    if !(4..=128).contains(&nbits) {
        eprintln!("twoscomp: number of bits out of range (4-128): {nbits}");
        std::process::exit(1);
    }
    let width = nbits / 4;
    let numstr = &args[2];
    let num = parse_num(numstr).unwrap_or_else(|e| {
        eprintln!("twoscomp: failed to parse number {numstr}: {e:?}");
        std::process::exit(1);
    });
    let senum = signextend(num, nbits);
    if num != senum && num >> nbits != 0 {
        eprintln!("twoscomp: number {numstr} out of range for width {nbits} bits");
        std::process::exit(1);
    }
    let num = senum;
    let n2c = signextend(twoscomp(num), nbits);

    // Signed, for printing as decimal.
    let snum = num as i128;
    let sn2c = n2c as i128;

    let mask = !0u128 >> (128 - nbits);
    let num = num & mask;
    let n2c = n2c & mask;
    println!("number:  0x{num:0>width$x} ({num:0>nbits$b})  [{snum} from {numstr}]");
    println!("2s cmpl: 0x{n2c:0>width$x} ({n2c:0>nbits$b})  [{sn2c}]");
}
