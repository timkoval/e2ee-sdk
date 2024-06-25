import e2ee_sdk
import os

def generate_rsa_keys():
    public_key_pem_path = "./pubkey.pem"
    private_key_pem_path = "./privkey.pem"
    e2ee_sdk.generate_rsa_keys(public_key_pem_path, private_key_pem_path)

def test_encrypt_decrypt_message():
    message = "testmessage"
    public_key_pem_path = "./pubkey.pem"
    private_key_pem_path = "./privkey.pem"
    encrypted_message = e2ee_sdk.encrypt_message(message, public_key_pem_path)
    decrypted_message = e2ee_sdk.decrypt_message(encrypted_message, private_key_pem_path)
    assert message == str(decrypted_message, encoding='utf-8')

generate_rsa_keys()
test_encrypt_decrypt_message()
os.remove("./pubkey.pem")
os.remove("./privkey.pem")

