pub(crate) const USDC_ADDRESS_STR: &str = "0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359";
pub(crate) const USDC_DECIMALS: u32 = 6;

pub(crate) mod approve;
pub(crate) mod auctions;
pub(crate) mod bonds;
pub(crate) mod bridge;
pub(crate) mod clob;
pub(crate) mod comments;
pub(crate) mod ctf;
pub(crate) mod data;
pub(crate) mod disputes;
pub(crate) mod events;
pub(crate) mod markets;
pub(crate) mod profiles;
pub(crate) mod questions;
pub(crate) mod series;
pub(crate) mod setup;
pub(crate) mod sports;
pub(crate) mod tags;
pub(crate) mod upgrade;
pub(crate) mod wallet;

pub(crate) fn is_numeric_id(id: &str) -> bool {
    id.parse::<u64>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_numeric_id_pure_digits() {
        assert!(is_numeric_id("12345"));
        assert!(is_numeric_id("0"));
    }

    #[test]
    fn is_numeric_id_rejects_non_digits() {
        assert!(!is_numeric_id("will-trump-win"));
        assert!(!is_numeric_id("0x123abc"));
        assert!(!is_numeric_id("123 456"));
    }

    #[test]
    fn is_numeric_id_rejects_empty() {
        assert!(!is_numeric_id(""));
    }
}
