pub fn simple_hash(message: &str) -> [u16; 16] {
    let mut hash: [u16; 16] = [0; 16];
    let mut i = 0;

    // this is a very simple hash function
    // it XOR the current hash value with the next character
    // and then adds the next character to the hash value
    // and then rotates the hash value
    for c in message.chars() {
        //cast c char to u16 to safely store in [u16; 16] array
        let c_value = c as u16;

        // XOR with the current hash value
        hash[i] ^= c_value;

        // Add the next character to the hash value and take modulo 256
        hash[(i + 1) % 16] = (hash[(i + 1) % 16].wrapping_add(c_value)) % 256;

        // Bitwise left rotation
        hash[(i + 2) % 16] = hash[(i + 2) % 16].rotate_left(3);

        // Bitwise right rotation
        hash[(i + 3) % 16] = hash[(i + 3) % 16].rotate_right(2);

        i = (i + 1) % 16;
    }

    // Final mixing
    for j in 0..16 {
        hash[j] ^= hash[(j + 5) % 16];
        hash[j] = hash[j].wrapping_add(0xA5);
    }

    hash
}

pub fn simple_2to1(hash1: &[u16; 16], hash2: &[u16; 16]) -> [u16; 16] {
    let mut hash: [u16; 16] = [0; 16];

    // XOR the two hashes bit by bit
    for i in 0..16 {
        hash[i] = hash1[i] ^ hash2[i];
    }

    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_hash() {
        println!("{:?}", simple_hash("Hello, world!"));
        assert_eq!(
            simple_hash("Hello, world!"),
            [176, 198, 259, 189, 195, 258, 169, 183, 222, 194, 195, 349, 296, 455, 354, 360]
        );
    }
}
