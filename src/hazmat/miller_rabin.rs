//! Miller-Rabin primality test; the numbers that pass it
//! are also called "strong probable-/pseudoprimes".
//!
//! See C. Pomerance, J. L. Selfridge, S. S. Wagstaff "The Pseudoprimes to 25*10^9",
//! Math. Comp. 35 1003-1026 (1980) (DOI: [10.2307/2006210](https://dx.doi.org/10.2307/2006210))

use rand_core::{CryptoRng, RngCore};

use crypto_bigint::{
    modular::{
        runtime_mod::{DynResidue, DynResidueParams},
        MulResidue, PowResidue,
    },
    Integer, NonZero, RandomMod, Uint,
};

pub struct MillerRabin<const L: usize> {
    candidate: Uint<L>,
    montgomery_params: DynResidueParams<L>,
    one_m: DynResidue<L>,
    cand_minus_one_m: DynResidue<L>,
    s: u32,
    d: Uint<L>,
}

impl<const L: usize> MillerRabin<L> {
    pub fn new(candidate: &Uint<L>) -> Self {
        let params = DynResidueParams::<L>::new(*candidate);
        let one_m = DynResidue::<L>::new(Uint::<L>::ONE, params);
        let cand_minus_one = candidate.wrapping_sub(&Uint::<L>::ONE);
        let cand_minus_one_m = DynResidue::<L>::new(cand_minus_one, params);
        let (s, d) = decompose(candidate);
        Self {
            candidate: *candidate,
            montgomery_params: params,
            one_m,
            cand_minus_one_m,
            s,
            d,
        }
    }

    pub fn check(&self, basis: &Uint<L>) -> bool {
        // TODO: it may be faster to first check that gcd(basis, candidate) == 1,
        // otherwise we can return `false` right away.

        let basis_m = DynResidue::<L>::new(*basis, self.montgomery_params);
        let mut test_m = basis_m.pow(&self.d);

        if test_m == self.one_m || test_m == self.cand_minus_one_m {
            return true;
        }
        for _ in 1..self.s {
            test_m = test_m.square();
            if test_m == self.one_m {
                return false;
            } else if test_m == self.cand_minus_one_m {
                return true;
            }
        }
        false
    }

    pub fn check_basis_two(&self) -> bool {
        self.check(&Uint::<L>::from(2u32))
    }

    pub fn check_random_basis<R: CryptoRng + RngCore>(&self, rng: &mut R) -> bool {
        // We sample a random basis from the range `[3, candidate-2]`:
        // - we have a separate method for basis 2;
        // - the test holds trivially for bases 1 or `candidate-1`.
        let range = self.candidate.wrapping_sub(&Uint::<L>::from(4u32));
        let range_nonzero = NonZero::new(range).unwrap();
        let random =
            Uint::<L>::random_mod(rng, &range_nonzero).wrapping_add(&Uint::<L>::from(3u32));
        self.check(&random)
    }
}

/// For the given odd `n`, finds `s` and odd `d` such that `n - 1 == 2^s * d`.
fn decompose<const L: usize>(n: &Uint<L>) -> (u32, Uint<L>) {
    let mut d = n.wrapping_sub(&Uint::<L>::ONE);
    let mut s = 0;

    while d.is_even().into() {
        d >>= 1;
        s += 1;
    }

    (s, d)
}

#[cfg(test)]
mod tests {
    use crypto_bigint::{Uint, U1536};
    use number_theory::NumberTheory;
    use rand_chacha::ChaCha8Rng;
    use rand_core::{CryptoRng, RngCore, SeedableRng};

    use super::MillerRabin;
    use crate::hazmat::{pseudoprimes, random_odd_uint, Sieve};

    fn is_spsp(num: u32) -> bool {
        pseudoprimes::STRONG_BASE_2
            .iter()
            .position(|x| *x == num)
            .is_some()
    }

    fn random_checks<const L: usize>(
        rng: &mut (impl CryptoRng + RngCore),
        mr: &MillerRabin<L>,
        count: usize,
    ) -> usize {
        (0..count)
            .map(|_| if mr.check_random_basis(rng) { 1 } else { 0 })
            .sum()
    }

    fn test_composites(numbers: &[u32], expected_result: bool) {
        let mut rng = ChaCha8Rng::from_seed(*b"01234567890123456789012345678901");
        for num in numbers.iter() {
            let base_2_false_positive = is_spsp(*num);
            let actual_expected_result = if base_2_false_positive {
                true
            } else {
                expected_result
            };

            // Test both single-limb and multi-limb, just in case.

            // A random base MR test is expected to report a composite as a prime
            // with about 1/4 probability. So we're expecting less than
            // 35 out of 100 false positives, seems to work.

            let mr = MillerRabin::new(&Uint::<1>::from(*num));
            assert_eq!(mr.check_basis_two(), actual_expected_result);
            let reported_prime = random_checks(&mut rng, &mr, 100);
            assert!(reported_prime < 35);

            let mr = MillerRabin::new(&Uint::<2>::from(*num));
            assert_eq!(mr.check_basis_two(), actual_expected_result);
            let reported_prime = random_checks(&mut rng, &mr, 100);
            assert!(reported_prime < 35);
        }
    }

    #[test]
    fn trivial() {
        let mut rng = ChaCha8Rng::from_seed(*b"01234567890123456789012345678901");
        let start = random_odd_uint::<16, _>(&mut rng, 1024);
        for num in Sieve::new(&start, 1024).take(10) {
            let mr = MillerRabin::new(&num);

            // Trivial tests, must always be true.
            assert!(mr.check(&1u32.into()));
            assert!(mr.check(&num.wrapping_sub(&1u32.into())));
        }
    }

    #[test]
    fn mersenne_prime() {
        let mut rng = ChaCha8Rng::from_seed(*b"01234567890123456789012345678901");

        // Mersenne prime 2^127-1
        let num = Uint::<2>::from_be_hex("7fffffffffffffffffffffffffffffff");

        let mr = MillerRabin::new(&num);
        assert!(mr.check_basis_two());
        for _ in 0..10 {
            assert!(mr.check_random_basis(&mut rng));
        }
    }

    #[test]
    fn strong_fibonacci_pseudoprimes() {
        let mut rng = ChaCha8Rng::from_seed(*b"01234567890123456789012345678901");

        for num in pseudoprimes::STRONG_FIBONACCI.iter() {
            let mr = MillerRabin::new(&num);
            assert!(!mr.check_basis_two());
            for _ in 0..1000 {
                assert!(!mr.check_random_basis(&mut rng));
            }
        }
    }

    #[test]
    fn strong_pseudoprimes_base_2() {
        // These known exceptions for the base 2 MR test.
        test_composites(pseudoprimes::STRONG_BASE_2, true);
    }

    #[test]
    fn lucas_pseudoprimes() {
        // Cross-test against the pseudoprimes that circumvent the Lucas test.
        // We expect the MR test to correctly classify them as composites (most of the time).
        test_composites(pseudoprimes::EXTRA_STRONG_LUCAS, false);
        test_composites(pseudoprimes::STRONG_LUCAS, false);
        test_composites(pseudoprimes::ALMOST_EXTRA_STRONG_LUCAS, false);
        test_composites(pseudoprimes::FIBONACCI, false);
        test_composites(pseudoprimes::BRUCKMAN_LUCAS, false);
        test_composites(pseudoprimes::LUCAS, false);
    }

    #[test]
    fn large_carmichael_number() {
        let mr = MillerRabin::new(&pseudoprimes::LARGE_CARMICHAEL_NUMBER);

        // It is known to pass MR tests for all prime bases <307
        assert!(mr.check_basis_two());
        assert!(mr.check(&U1536::from(293u64)));

        // A test with basis 307 correctly reports the number as composite.
        assert!(!mr.check(&U1536::from(307u64)));
    }

    #[test]
    fn exhaustive() {
        // Test all the odd numbers up to the limit where we know the false positives,
        // and compare the results with the reference.
        for num in (3..pseudoprimes::EXHAUSTIVE_TEST_LIMIT).step_by(2) {
            let res_ref = num.is_prime();

            let spsp = is_spsp(num);

            let mr = MillerRabin::new(&Uint::<1>::from(num));
            let res = mr.check_basis_two();
            let expected = spsp || res_ref;
            assert_eq!(
                res, expected,
                "Miller-Rabin: n={}, expected={}, actual={}",
                num, expected, res,
            );
        }
    }
}
