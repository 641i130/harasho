// AES encryption
use aes::cipher::{AsyncStreamCipher, KeyIvInit};

type Aes128CfbEnc = cfb_mode::Encryptor<aes::Aes128>;
type Aes128CfbDec = cfb_mode::Decryptor<aes::Aes128>;

pub fn aes_en(plaintext: &&str) -> Vec<u8> {
    // Encodes string with aes 128 cfb encryption
    // Return encrypted text
    // Crypto constants
    // Must have padding!!! This was overlooked a while back... 
    let ciphertext = plaintext.as_bytes().to_vec();
    let key: &[u8] = "0123456789012345".as_bytes();
    let iv: &[u8] = "0123456789012345".as_bytes();
    
    // Now we need to pad it TO BAD I HAD TO IMPLEMENT PKCS#7 in here lmao
    // PKCS#7 padding... too lazy to get a library to do this and the ones I have don't...?
    let block_size = 16;
    let padder = block_size - (ciphertext.len() % block_size);
    let padding = vec![padder as u8; padder];
    let mut padded_data = Vec::from(ciphertext);
    padded_data.extend(padding);

    // Encrypt
    Aes128CfbEnc::new(key.into(), iv.into()).encrypt(&mut padded_data);
    padded_data.into()
    
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


