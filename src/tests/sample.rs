#![allow(warnings)]
#[cfg(test)]
pub(in crate::tests) mod sample_tests {
    use crate::{params::*, polynomials::*};
    extern crate std;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};
    use std::{collections::HashMap, ops::Range};

    const EPSILON: f64 = 0.1;
    pub(in crate::tests) fn generate_random_seed() -> [u8; 32] {
        let mut rng = StdRng::from_entropy();
        let mut seed = [0u8; 32];
        rng.fill(&mut seed);
        seed
    }

    pub(in crate::tests) fn generate_random_nonce() -> u8 {
        let mut rng = StdRng::from_entropy();
        rng.gen::<u8>()
    }

    pub(in crate::tests) fn range_test(poly: &Poly, eta: usize) {
        let range_lim = eta as i16;
        let range: Range<i16> = -range_lim..range_lim + 1;

        for coeff in poly.coeffs.iter() {
            assert!(
                range.contains(coeff),
                "coefficient {} not in valid range",
                coeff
            );
        }
    }

    pub(in crate::tests) fn dist_test(poly: &Poly, eta: usize) {
        let expected_probabilities: HashMap<i16, f64>;
        match eta {
            2 => {
                expected_probabilities = [
                    (-2, 1.0 / 16.0),
                    (-1, 1.0 / 4.0),
                    (0, 3.0 / 8.0),
                    (1, 1.0 / 4.0),
                    (2, 1.0 / 16.0),
                ]
                .iter()
                .cloned()
                .collect();
            }
            3 => {
                expected_probabilities = [
                    (-3, 1.0 / 64.0),
                    (-2, 3.0 / 32.0),
                    (-1, 15.0 / 64.0),
                    (0, 5.0 / 16.0),
                    (1, 16.0 / 64.0),
                    (2, 3.0 / 32.0),
                    (3, 1.0 / 64.0),
                ]
                .iter()
                .cloned()
                .collect();
            }
            _ => panic!("invalid eta in test"),
        }

        let mut actual_counts: HashMap<i16, usize> = HashMap::new();
        for &coeff in poly.coeffs.iter() {
            *actual_counts.entry(coeff).or_insert(0) += 1;
        }

        for (coeff, expected_prob) in expected_probabilities.iter() {
            let actual_count = *actual_counts.get(coeff).unwrap_or(&0);
            let total_samples = poly.coeffs.len() as f64;
            let actual_prob = (actual_count as f64) / total_samples;

            assert!(
                (actual_prob - expected_prob).abs() < EPSILON,
                "Actual probability {} does not match expected {} within boundries for coeff {}",
                actual_prob,
                expected_prob,
                coeff
            );
        }
    }

    #[test]
    fn derive_noise_2_range_test() {
        let seed = generate_random_seed();
        let nonce = generate_random_nonce();

        let mut poly = Poly::new();
        poly.derive_noise(&seed, nonce, Eta::Two);

        range_test(&poly, 2);
    }

    #[test]
    fn derive_noise_3_range_test() {
        let seed = generate_random_seed();
        let nonce = generate_random_nonce();

        let mut poly = Poly::new();
        poly.derive_noise(&seed, nonce, Eta::Three);

        range_test(&poly, 3);
    }

    #[test]
    fn derive_noise_2_dist_test() {
        let seed = generate_random_seed();
        let nonce = generate_random_nonce();

        let mut poly = Poly::new();
        poly.derive_noise(&seed, nonce, Eta::Two);

        dist_test(&poly, 2);
    }

    #[test]
    fn derive_noise_3_dist_test() {
        let seed = generate_random_seed();
        let nonce = generate_random_nonce();

        let mut poly = Poly::new();
        poly.derive_noise(&seed, nonce, Eta::Three);

        dist_test(&poly, 3);
    }
}
