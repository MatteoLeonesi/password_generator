use rand::distributions::{Distribution, Uniform};
use rand::Rng;

const CHARACTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789\
                            !@#$%^&*()_+-=[]{}|;:,.<>?";

pub fn generate_password(length: usize) -> String {
    let rng = rand::rngs::OsRng;
    let char_dist = Uniform::from(0..CHARACTERS.len());
    let password: String = rng
        .sample_iter(&char_dist)
        .take(length)
        .map(|c| CHARACTERS[c] as char)
        .collect();
    password
}
