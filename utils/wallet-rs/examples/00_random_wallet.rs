use std::io::BufWriter;

use solana_sdk::{
    signature::Signer,
    signer::keypair::{write_keypair, Keypair},
};
fn main() {
    let keypair = Keypair::new();
    let secret = keypair.secret();

    dbg!(&keypair);
    dbg!(secret);

    let buf = Vec::new();
    let mut buffer = BufWriter::new(buf);

    let seed = write_keypair(&keypair, &mut buffer).unwrap();
    dbg!(seed);

    let private = keypair.to_base58_string();
    dbg!(private);

    let public = keypair.pubkey();

    dbg!(public);
}
