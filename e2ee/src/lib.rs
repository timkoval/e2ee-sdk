use rand;
use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
    pkcs8::LineEnding,
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};

pub fn generate_keys() -> (String, String) {
    let mut range = rand::thread_rng();
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut range, bits).expect("failed to generate key");
    let public_key = RsaPublicKey::from(&private_key);
    let private_key_encoded =
        EncodeRsaPrivateKey::to_pkcs1_pem(&private_key, LineEnding::default())
            .expect("unable to encode private key pem");
    let public_key_encoded = EncodeRsaPublicKey::to_pkcs1_pem(&public_key, LineEnding::default())
        .expect("unable to encode public key pem");
    (public_key_encoded, private_key_encoded.to_string())
}

pub fn encrypt_message(message: String, public_key_pem: String) -> Vec<u8> {
    let mut range = rand::thread_rng();
    let public_key =
        RsaPublicKey::from_pkcs1_pem(&public_key_pem).expect("failed to load public key");

    public_key
        .encrypt(&mut range, Pkcs1v15Encrypt, &message.as_bytes())
        .expect("failed to encrypt message")
}

pub fn decrypt_message(encrypted_message: Vec<u8>, private_key_pem: String) -> Vec<u8> {
    let private_key =
        RsaPrivateKey::from_pkcs1_pem(&private_key_pem).expect("failed to load private key");
    private_key
        .decrypt(Pkcs1v15Encrypt, &encrypted_message)
        .expect("failed to decrypt message")
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertables::{
        assert_contains, assert_contains_as_result, assert_not_contains,
        assert_not_contains_as_result,
    };

    #[test]
    fn test_keys_generation() {
        let (public_key, private_key) = generate_keys();
        assert_contains!(public_key, "BEGIN RSA PUBLIC KEY");
        assert_contains!(public_key, "END RSA PUBLIC KEY");
        assert_not_contains!(public_key, "BEGIN RSA PRIVATE KEY");
        assert_not_contains!(public_key, "END RSA PRIVATE KEY");

        assert_contains!(private_key, "BEGIN RSA PRIVATE KEY");
        assert_contains!(private_key, "END RSA PRIVATE KEY");
        assert_not_contains!(private_key, "BEGIN RSA PUBLIC KEY");
        assert_not_contains!(private_key, "END RSA PUBLIC KEY");
    }

    #[test]
    fn test_encryption_decryption() {
        let (public_key, private_key) = generate_keys();

        let message = "test message";
        let encrypted_message = encrypt_message(message.to_string(), public_key);
        let decrypted_message = decrypt_message(encrypted_message, private_key);

        assert_eq!(message.as_bytes(), decrypted_message);
    }
}
