pub mod summary;

// Description: Pricing constants
// Date: 2024-09-02
// Pricing in Credits
// Each credit is worth 0.03 USD
// 200 credits = 6 USD
// 2000 credits = 50 USD

pub static CREDIT_PRICE: f64 = 0.03;
pub static DEFAULT_STARTER_CREDITS: i64 = 5;

pub static TOKEN_WEIGHT: usize = 4;
pub static DEFAULT_MODEL: &str = "claude-3-haiku-20240307";
pub static DEFAULT_MAX_OUTPUT_TOKENS: usize = 257 as usize;