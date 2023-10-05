use crate::provider::Provider;
use ring::aead::{Aad, LessSafeKey, Nonce, SealingKey, UnboundKey, AES_256_GCM};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};

const CREDENTIAL_LEN: usize = 32; // AES-256 needs 32 bytes
const N_ITER: u32 = 100_000;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppConfig {
    pub provider: Option<Provider>,
    pub e_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token;

impl Token {
    fn derive_from_passwd(password: &[u8], salt: &[u8; SALT_LEN]) -> [u8; CREDENTIAL_LEN] {
        let mut key = [0u8; CREDENTIAL_LEN];
        pbkdf2::pbkdf2_hmac::<sha2::Sha256>(password, salt, N_ITER, &mut key);
        key
    }
    pub fn encrypt(token: &str, passphrase: &str) -> Result<Vec<u8>, crate::error::Error> {
        let rng = SystemRandom::new();
        let mut salt = [0u8; SALT_LEN];
        rng.fill(&mut salt)?;

        let key = Self::derive_from_passwd(passphrase.as_bytes(), &salt);

        let unbound_key = UnboundKey::new(&AES_256_GCM, &key)?;
        let less_safe_key = LessSafeKey::new(unbound_key);

        let mut nonce = [0u8; NONCE_LEN];
        rng.fill(&mut nonce)?;

        let plaintext = token.as_bytes();
        let mut in_out = plaintext.to_vec();

        let nonce_s = Nonce::assume_unique_for_key(nonce);
        less_safe_key.seal_in_place_append_tag(nonce_s, Aad::empty(), &mut in_out)?;

        Ok([salt.to_vec(), nonce.as_ref().to_vec(), in_out].concat())
    }

    pub fn decrypt(value: &[u8], key: &str) -> Result<String, crate::error::Error> {
        let (salt, remaining) = value.split_at(SALT_LEN);
        let (nonce, ciphertext_and_tag) = remaining.split_at(NONCE_LEN);

        let salt_array: &[u8; SALT_LEN] = salt.try_into().expect("Wrong salt length");
        let derived_key = Self::derive_from_passwd(key.as_bytes(), salt_array);

        let unbound_key = UnboundKey::new(&AES_256_GCM, &derived_key)?;
        let less_safe_key = LessSafeKey::new(unbound_key);

        let nonce_array: &[u8; NONCE_LEN] = nonce.try_into().expect("Wrong nonce length");
        let mut in_out = ciphertext_and_tag.to_vec();

        less_safe_key
            .open_in_place(
                Nonce::assume_unique_for_key(*nonce_array),
                Aad::empty(),
                &mut in_out,
            )
            .map_err(|_| crate::error::Error::InvalidToken)?;

        in_out.truncate(in_out.len() - AES_256_GCM.tag_len());

        let decrypted_data = String::from_utf8(in_out)?;
        Ok(decrypted_data)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_basic_encryption() {
        let token = "mysecretvalue";
        let key = "mysecretkey";

        let encrypted = Token::encrypt(token, key).unwrap();

        let decrypted = Token::decrypt(&encrypted, key).unwrap();

        assert_eq!(token, decrypted);
    }
}
