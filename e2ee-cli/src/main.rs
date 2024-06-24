use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use e2ee::{decrypt_message, encrypt_message, generate_keys};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Generate {
        public_key_pem_path: PathBuf,
        private_key_pem_path: PathBuf,
    },
    Encrypt {
        message_path: PathBuf,
        output_path: PathBuf,
        public_key_pem_path: PathBuf,
    },
    Decrypt {
        encrypted_message_path: PathBuf,
        output_path: PathBuf,
        private_key_pem_path: PathBuf,
    },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Generate {
            public_key_pem_path,
            private_key_pem_path,
        } => generate(public_key_pem_path, private_key_pem_path),
        Commands::Encrypt {
            message_path,
            output_path,
            public_key_pem_path,
        } => encrypt(message_path, output_path, public_key_pem_path),
        Commands::Decrypt {
            encrypted_message_path,
            output_path,
            private_key_pem_path,
        } => decrypt(encrypted_message_path, output_path, private_key_pem_path),
    }
}

fn generate(public_key_pem_path: PathBuf, private_key_pem_path: PathBuf) {
    let (public_key_pem, private_key_pem) = generate_keys();
    fs::write(public_key_pem_path, public_key_pem).expect("unable to write public key");
    fs::write(private_key_pem_path, private_key_pem).expect("unable to write private key");
}

fn encrypt(message_path: PathBuf, output_path: PathBuf, public_key_pem_path: PathBuf) {
    let public_key_pem =
        fs::read_to_string(public_key_pem_path).expect("unable to read public key pem");
    let message = fs::read_to_string(message_path).expect("unable to read message");
    let encrypted_message = encrypt_message(message, public_key_pem);
    fs::write(output_path, encrypted_message).expect("unable to write output file");
}

fn decrypt(encrypted_message_path: PathBuf, output_path: PathBuf, private_key_pem_path: PathBuf) {
    let private_key_pem =
        fs::read_to_string(private_key_pem_path).expect("unable to read private key pem");
    let encrypted_message =
        fs::read(encrypted_message_path).expect("unable to read encrypted message");
    let encrypted_message = decrypt_message(encrypted_message, private_key_pem);
    fs::write(output_path, encrypted_message).expect("unable to write output file");
}
