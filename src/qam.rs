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

#[derive(Default, Copy, Clone)]
/// Represents the QAM order for modulation.
pub enum QAMOrder {
    #[default]
    QAM16,
}

pub struct QAMModulator {
    qam_order: QAMOrder,
}

impl QAMModulator {
    /// Create a new QAMModulator for the specified QAM order.
    pub fn new(qam_order: QAMOrder) -> Self {
        QAMModulator { qam_order }
    }

    /// Modulate a byte array into QAM symbols based on the specified QAM order.
    ///
    /// Each byte will result in QAMModulator.bits_per_symbol() symbols,
    /// as the number of bits per symbol depends on the QAM order.
    ///
    /// # Example
    /// ```
    /// let data = "Hello, world!".as_bytes();
    /// let modulator = QAMModulator::new(QAMOrder::QAM16);
    /// let symbols = modulator.modulate_qam(data);
    ///
    /// assert_eq!(symbols.len(), data.len() * 2); // Each byte produces two QAM symbols for QAM-16
    /// ```
    pub fn modulate(&self, data: &[u8]) -> Vec<Complex32> {
        let mut symbols = Vec::new();
        match self.qam_order {
            QAMOrder::QAM16 => {
                for &byte in data {
                    let first_nibble = (byte >> 4) & 0x0F; // Get the first 4 bits
                    let second_nibble = byte & 0x0F; // Get the last 4 bits

                    symbols.push(QAM16_LOOKUP[first_nibble as usize]);
                    symbols.push(QAM16_LOOKUP[second_nibble as usize]);
                }
            }
        }
        symbols
    }

    /// Returns the number of bits per symbol for the specified QAM order.
    pub fn bits_per_symbol(&self) -> u32 {
        match self.qam_order {
            QAMOrder::QAM16 => 4, // QAM-16 uses 4 bits per symbol
        }
    }
}
