use std::io;

use passwordgen::generate_passphrase;

fn main() -> io::Result<()> {
    let passphrase_size = 7;
    let result = generate_passphrase(passphrase_size)?;
    println!("prefixes: {:?}", result.prefixes);
    println!("passphrase: {:?}", result.passphrase);
    println!("cost: {}", result.cost);
    Ok(())
}
