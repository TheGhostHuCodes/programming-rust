use std::io::Write;
use std::num::NonZeroU64;
use std::str::FromStr;

fn gcd(n: NonZeroU64, m: NonZeroU64) -> NonZeroU64 {
    let mut n_ = n.get();
    let mut m_ = m.get();
    while m_ != 0 {
        if m_ < n_ {
            let temp = m_;
            m_ = n_;
            n_ = temp
        }
        m_ = m_ % n_;
    }
    NonZeroU64::new(n_).unwrap()
}

fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(NonZeroU64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for &m in &numbers[1..] {
        d = gcd(d, m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_gcd() {
        assert_eq!(
            gcd(
                NonZeroU64::new(2 * 5 * 11 * 17).unwrap(),
                NonZeroU64::new(3 * 7 * 13 * 19).unwrap()
            )
            .get(),
            1
        );

        assert_eq!(
            gcd(
                NonZeroU64::new(2 * 3 * 5 * 11 * 17).unwrap(),
                NonZeroU64::new(3 * 7 * 11 * 13 * 19).unwrap()
            )
            .get(),
            3 * 11
        );
    }
}
