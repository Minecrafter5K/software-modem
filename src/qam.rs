use realfft::num_complex::Complex32;

// QAM-16 lookup table
const QAM16_LOOKUP: [Complex32; 16] = [
    Complex32::new(1.0, 1.0),   // 0000
    Complex32::new(1.0, 3.0),   // 0001
    Complex32::new(3.0, 1.0),   // 0010
    Complex32::new(3.0, 3.0),   // 0011
    Complex32::new(1.0, -1.0),  // 0100
    Complex32::new(1.0, -3.0),  // 0101
    Complex32::new(3.0, -1.0),  // 0110
    Complex32::new(3.0, -3.0),  // 0111
    Complex32::new(-1.0, 1.0),  // 1000
    Complex32::new(-1.0, 3.0),  // 1001
    Complex32::new(-3.0, 1.0),  // 1010
    Complex32::new(-3.0, 3.0),  // 1011
    Complex32::new(-1.0, -1.0), // 1100
    Complex32::new(-1.0, -3.0), // 1101
    Complex32::new(-3.0, -1.0), // 1110
    Complex32::new(-3.0, -3.0), // 1111
];

/// Modulate a byte array into QAM-16 symbols.
pub fn modulate_qam16(data: &[u8]) -> Vec<Complex32> {
    let mut symbols = Vec::new();

    for &byte in data {
        let first_nibble = (byte >> 4) & 0x0F; // Get the first 4 bits
        let second_nibble = byte & 0x0F; // Get the last 4 bits

        symbols.push(QAM16_LOOKUP[first_nibble as usize]);
        symbols.push(QAM16_LOOKUP[second_nibble as usize]);
    }

    symbols
}
