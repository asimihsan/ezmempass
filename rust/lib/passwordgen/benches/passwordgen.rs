#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use passwordgen::{generate_passphrase, GeneratePassphraseInput};
    use test::Bencher;

    #[bench]
    fn bench_passphrase_eight_prefixes(b: &mut Bencher) {
        let passphrase_length = 8;
        let input = GeneratePassphraseInput {
            passphrase_length: passphrase_length,
            add_capital_letter: true,
            add_digit: true,
            add_symbol: true,
        };
        b.iter(|| generate_passphrase(&input).unwrap());
    }
}
