pub mod log;
pub mod jwt;

pub fn random_int() -> u32 {
    rand::random::<u32>()
} 