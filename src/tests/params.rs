#![allow(warnings)]
#[cfg(test)]
mod params_tests {
    use crate::params::*;

    #[test]
    fn sec_level_test() {
        assert_eq!(
            SecurityLevel::new(K::Two),
            SecurityLevel::FiveOneTwo {
                k: K::Two,
                eta_1: Eta::Three,
                eta_2: Eta::Two,
            }
        );
        assert_eq!(
            SecurityLevel::new(K::Three),
            SecurityLevel::SevenSixEight {
                k: K::Three,
                eta_1: Eta::Two,
                eta_2: Eta::Two,
            }
        );
        assert_eq!(
            SecurityLevel::new(K::Four),
            SecurityLevel::TenTwoFour {
                k: K::Four,
                eta_1: Eta::Two,
                eta_2: Eta::Two,
            }
        );
    }

    #[test]
    fn poly_compressed_bytes_test() {
        assert_eq!(SecurityLevel::new(K::Two).poly_compressed_bytes(), 128);
        assert_eq!(SecurityLevel::new(K::Three).poly_compressed_bytes(), 128);
        assert_eq!(SecurityLevel::new(K::Four).poly_compressed_bytes(), 160);
    }

    #[test]
    fn poly_vec_bytes_test() {
        assert_eq!(SecurityLevel::new(K::Two).poly_vec_bytes(), 768);
        assert_eq!(SecurityLevel::new(K::Three).poly_vec_bytes(), 1152);
        assert_eq!(SecurityLevel::new(K::Four).poly_vec_bytes(), 1536);
    }

    #[test]
    fn poly_vec_compressed_bytes_test() {
        assert_eq!(SecurityLevel::new(K::Two).poly_vec_compressed_bytes(), 640);
        assert_eq!(
            SecurityLevel::new(K::Three).poly_vec_compressed_bytes(),
            960
        );
        assert_eq!(
            SecurityLevel::new(K::Four).poly_vec_compressed_bytes(),
            1408
        );
    }

    #[test]
    fn indcpa_public_key_bytes_test() {
        assert_eq!(SecurityLevel::new(K::Two).indcpa_public_key_bytes(), 800);
        assert_eq!(SecurityLevel::new(K::Three).indcpa_public_key_bytes(), 1184);
        assert_eq!(SecurityLevel::new(K::Four).indcpa_public_key_bytes(), 1568);
    }

    #[test]
    fn indcpa_private_key_bytes_test() {
        assert_eq!(SecurityLevel::new(K::Two).indcpa_private_key_bytes(), 768);
        assert_eq!(
            SecurityLevel::new(K::Three).indcpa_private_key_bytes(),
            1152
        );
        assert_eq!(SecurityLevel::new(K::Four).indcpa_private_key_bytes(), 1536);
    }

    #[test]
    fn indcpa_bytes_test() {
        assert_eq!(SecurityLevel::new(K::Two).indcpa_bytes(), 768);
        assert_eq!(SecurityLevel::new(K::Three).indcpa_bytes(), 1088);
        assert_eq!(SecurityLevel::new(K::Four).indcpa_bytes(), 1568);
    }

    #[test]
    fn public_key_bytes_test() {
        assert_eq!(SecurityLevel::new(K::Two).public_key_bytes(), 800);
        assert_eq!(SecurityLevel::new(K::Three).public_key_bytes(), 1184);
        assert_eq!(SecurityLevel::new(K::Four).public_key_bytes(), 1568);
    }

    #[test]
    fn private_key_bytes_test() {
        assert_eq!(SecurityLevel::new(K::Two).private_key_bytes(), 1632);
        assert_eq!(SecurityLevel::new(K::Three).private_key_bytes(), 2400);
        assert_eq!(SecurityLevel::new(K::Four).private_key_bytes(), 3168);
    }

    #[test]
    fn cipher_text_bytes_test() {
        assert_eq!(SecurityLevel::new(K::Two).cipher_text_bytes(), 768);
        assert_eq!(SecurityLevel::new(K::Three).cipher_text_bytes(), 1088);
        assert_eq!(SecurityLevel::new(K::Four).cipher_text_bytes(), 1568);
    }
}
