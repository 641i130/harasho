// AES encryption
use aes::cipher::{AsyncStreamCipher, KeyIvInit};

type Aes128CfbEnc = cfb_mode::Encryptor<aes::Aes128>;
type Aes128CfbDec = cfb_mode::Decryptor<aes::Aes128>;

pub fn aes_en(plaintext: &&str) -> Vec<u8> {
    // Encodes string with aes 128 cfb encryption
    // Return encrypted text
    // Crypto constants
    let mut ciphertext = plaintext.as_bytes().to_vec();
    let key: &[u8] = "0123456789012345".as_bytes();
    let iv: &[u8] = "0123456789012345".as_bytes();

    // Encrypt
    Aes128CfbEnc::new(key.into(), iv.into()).encrypt(&mut ciphertext);
    ciphertext.into()
}

pub fn aes_dec(ciphertext: &[u8]) -> String {
    // Decodes string with aes 128 cfb encryption
    let mut plaintext = ciphertext.to_vec();
    let key: &[u8] = "0123456789012345".as_bytes();
    let iv: &[u8] = "0123456789012345".as_bytes();
    // Decrypt
    Aes128CfbDec::new(key.into(), iv.into()).decrypt(&mut plaintext);
    String::from_utf8_lossy(&plaintext).into()
}


