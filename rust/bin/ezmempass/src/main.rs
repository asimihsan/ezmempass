use std::{env, io};

use passwordgen::{generate_passphrase, GeneratePassphraseInput};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let passphrase_length: i32 = (args[1]).parse().unwrap();
    let input = GeneratePassphraseInput {
        passphrase_length: passphrase_length,
        add_capital_letter: true,
        add_digit: true,
        add_symbol: true,
    };
    let result = generate_passphrase(&input)?;
    println!("result: {:#?}", result);
    Ok(())
}
