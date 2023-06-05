pub fn get_bit_value(&byte: &u8, index: u8) -> u8 {
    (byte >> index) & 1
}
