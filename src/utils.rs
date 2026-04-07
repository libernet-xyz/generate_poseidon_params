use primitive_types::U256;

pub fn from_hex(hex: &str) -> U256 {
    hex.parse().unwrap()
}
