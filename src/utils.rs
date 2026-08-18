use primitive_types::U256;

pub fn from_hex_u64(hex: &str) -> u64 {
    let digits = hex
        .strip_prefix("0x")
        .or_else(|| hex.strip_prefix("0X"))
        .unwrap();
    u64::from_str_radix(digits, 16).unwrap()
}

pub fn from_hex_u256(hex: &str) -> U256 {
    hex.parse().unwrap()
}
