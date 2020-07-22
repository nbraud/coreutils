// * This file is part of the uutils coreutils package.
// *
// * (c) 2020 nicoo <nicoo@debian.org>
// *
// * For the full copyright and license information, please view the LICENSE file
// * that was distributed with this source code.

use crate::numeric::*;

// This ought to be in `numeric`, but it's used by `build.rs` and so cannot
// contain tables generated at build-time.
include!(concat!(env!("OUT_DIR"), "/square_table.rs"));
pub fn exact_sqrt(n: u64) -> Option<u64> {
    // Eliminate most non-squares with a table-based approximation of the
    // Legendre symbol.
    // See H. Cohen's _Course in Computational Algebraic Number Theory_,
    //  algorithm 1.7.3, page 40.
    let _n = n as usize;
    if Q11[_n % 11] || Q63[_n % 63] || Q64[_n % 64] || Q65[_n % 65] {
        return None;
    }

    let r = floor_sqrt(n);
    if r * r == n {
        Some(r)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;

    quickcheck! {
        fn exact_sqrt_matches(n: u64) -> bool {
            if let Some(r) = exact_sqrt(n) {
                r * r == n
            } else {
                true
            }
        }

        fn exact_sqrt_square(n: u64) -> bool {
            exact_sqrt(n * n) == Some(n)
        }
    }
}
