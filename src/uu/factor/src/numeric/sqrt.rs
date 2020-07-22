// * This file is part of the uutils coreutils package.
// *
// * (c) 2020 nicoo  <nicoo@debian.org>
// *
// * For the full copyright and license information, please view the LICENSE file
// * that was distributed with this source code.

pub fn floor_sqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut x = {
        let k = 64 - n.leading_zeros();
        // 2ᵏ⁻¹ ≤ n < 2ᵏ
        debug_assert!(n < (1 << k));
        debug_assert!((1 << k) <= 2 * n);

        1 << ((k / 2) + (k % 2))
    };
    debug_assert!(x * x >= n, "{} < √{}", x, n);

    loop {
        let y = (x + n / x) / 2;

        if y >= x {
            return x;
        } else {
            x = y;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;

    quickcheck! {
        /// Test that floor_sqrt(n) <= √n < floor_sqrt(n) + 1
        fn test_floor_sqrt(n: u64) -> bool {
            let s = floor_sqrt(n);
            let t = s + 1;
            (s * s <= n) && (t * t > n)
        }
    }
}
