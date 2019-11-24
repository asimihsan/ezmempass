#[cfg(test)]
mod test_generate_passphrase {
    use passwordgen::{generate_passphrase_internal, GeneratePassphraseInput};
    use rand::rngs::mock;

    #[test]
    fn test_digit_present_if_asking_for_digit() {
        // === given ==
        let mut rng = mock::StepRng::new(0, 1);
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
        let mut rng = mock::StepRng::new(0, 1);
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
}
