use fernet::Fernet;
use fernet::DecryptionError;

pub fn encrypt(f: &mut Fernet, content: Vec<u8>) -> String {
    f.encrypt(&content)
}
pub fn decrypt(f: &mut Fernet, content: String) -> Result<Vec<u8>, DecryptionError> {
    f.decrypt(&content)
}
