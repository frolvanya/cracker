use sha2::{Digest, Sha256, Sha512};

pub fn sha256(text: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());

    format!("{:x}", hasher.finalize())
}

pub fn sha512(text: String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(text.as_bytes());

    format!("{:x}", hasher.finalize())
}

pub fn md5(text: String) -> String {
    let digest = md5::compute(text.as_bytes());

    format!("{:x}", digest)
}
