use pwhash::bcrypt;

pub struct BcryptHelper {}

impl BcryptHelper {
    pub fn hash_text(text: &str) -> pwhash::Result<String> {
        bcrypt::hash(text)
    }

    pub fn verify_hash(text: &str, hash: &str) -> bool {
        bcrypt::verify(text, hash)
    }
}
