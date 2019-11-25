#[cfg(test)]
mod test_generate_passphrase {
    use passwordgen::{generate_passphrase_internal, GeneratePassphraseInput};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_digit_present_if_asking_for_digit() {
        // === given ==
        let mut rng: StdRng = SeedableRng::seed_from_u64(0);
        let input = GeneratePassphraseInput {
            passphrase_length: 7,
            add_digit: true,
            add_capital_letter: false,
            add_symbol: false,
        };

        // == when ==
        let result = generate_passphrase_internal(&input, &mut rng).unwrap();

        // == then ==
        assert!(result.password.chars().any(char::is_numeric));
    }

    #[test]
    fn test_digit_not_present_if_not_asking_for_digit() {
        // === given ==
        let mut rng: StdRng = SeedableRng::seed_from_u64(0);
        let input = GeneratePassphraseInput {
            passphrase_length: 7,
            add_digit: false,
            add_capital_letter: false,
            add_symbol: false,
        };

        // == when ==
        let result = generate_passphrase_internal(&input, &mut rng).unwrap();

        // == then ==
        assert!(!result.password.chars().any(char::is_numeric));
    }

    /// Generate a lot of passwords. Eventually can be used for benchmarking. But for now this
    /// also reproduces the short-circuit bug, where our use of the graph is broken.
    /// TODO little bit slow, ignore for now.
    #[test]
    #[ignore]
    fn test_stress_test() {
        // === given ==
        let mut rng: StdRng = SeedableRng::seed_from_u64(0);
        let input = GeneratePassphraseInput {
            passphrase_length: 25,
            add_digit: true,
            add_capital_letter: true,
            add_symbol: true,
        };

        // == when ==
        for _i in 0..10 {
            let result = generate_passphrase_internal(&input, &mut rng).unwrap();
            assert_eq!(25, result.prefixes.len());
            assert_eq!(25, result.words.len());
        }
    }
}
