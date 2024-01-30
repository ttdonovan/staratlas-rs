pub(crate) fn str_to_u8_32(s: &str) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        array[i] = byte;
    }

    array
}
