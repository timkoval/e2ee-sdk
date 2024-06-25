use std::{fs, path::PathBuf, str::FromStr};

use e2ee;

#[uniffi::export]
fn generate_rsa_keys(public_key_pem_path: String, private_key_pem_path: String) {
    let public_key_pem_path =
        PathBuf::from_str(&public_key_pem_path).expect("unable to parse public key pem path");
    let private_key_pem_path =
        PathBuf::from_str(&private_key_pem_path).expect("unable to parse private key pem path");
    let (public_key_pem, private_key_pem) = e2ee::generate_keys();
    fs::write(public_key_pem_path, public_key_pem).expect("unable to write public key");
    fs::write(private_key_pem_path, private_key_pem).expect("unable to write private key");
}

#[uniffi::export]
fn encrypt_message(message: String, public_key_pem_path: String) -> Vec<u8> {
    let public_key_pem_path =
        PathBuf::from_str(&public_key_pem_path).expect("unable to parse public key pem path");
    let public_key_pem =
        fs::read_to_string(public_key_pem_path).expect("unable to read public key pem");
    e2ee::encrypt_message(message, public_key_pem)
}

#[uniffi::export]
fn decrypt_message(encrypted_message: Vec<u8>, private_key_pem_path: String) -> Vec<u8> {
    let private_key_pem_path =
        PathBuf::from_str(&private_key_pem_path).expect("unable to parse private key pem path");
    let private_key_pem =
        fs::read_to_string(private_key_pem_path).expect("unable to read private key pem");
    e2ee::decrypt_message(encrypted_message, private_key_pem)
}

uniffi::setup_scaffolding!();
