use core::panic;
use std::fmt::Display;

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

#[derive(Default, Copy, Clone, Debug)]
/// Represents the QAM order for modulation.
pub enum QAMOrder {
    #[default]
    QAM16,
}
impl Display for QAMOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QAMOrder::QAM16 => write!(f, "QAM-16"),
        }
    }
}

/// A modulator and demodulator for Quadrature Amplitude Modulation (QAM).
///
/// This struct allows modulating and demodulating data (byte slices) into QAM symbols.
///
/// # Example
/// ```
/// use software_modem::qam::{ QAMModem, QAMOrder };
///
/// let data = "Hello, world!".as_bytes();
/// let modem = QAMModem::new(QAMOrder::QAM16);
///
/// let symbols = modem.modulate(data);
/// let demodulated_data = modem.demodulate(&symbols);
///
/// assert_eq!(data, demodulated_data);
/// ```
pub struct QAMModem {
    qam_order: QAMOrder,
}

impl QAMModem {
    /// Create a new QAMModem for the specified QAM order.
    pub fn new(qam_order: QAMOrder) -> Self {
        QAMModem { qam_order }
    }

    /// Modulate a byte array into QAM symbols.
    ///
    /// Each byte will result in QAMModulator.bits_per_symbol() symbols,
    /// as the number of bits per symbol depends on the QAM order.
    ///
    /// # Example
    /// ```
    /// use software_modem::qam::{ QAMModem, QAMOrder };
    ///
    /// let data = "Hello, world!".as_bytes();
    /// let modem = QAMModem::new(QAMOrder::QAM16);
    /// let symbols = modem.modulate(data);
    ///
    /// assert_eq!(symbols.len(), data.len() * 2); // Each byte produces two QAM symbols for QAM-16
    /// ```
    pub fn modulate(&self, data: &[u8]) -> Vec<Complex32> {
        let mut symbols = Vec::new();
        match self.qam_order {
            QAMOrder::QAM16 => {
                for &byte in data {
                    let first_nibble = (byte >> 4) & 0x0f; // Get the first 4 bits
                    let second_nibble = byte & 0x0f; // Get the last 4 bits

                    symbols.push(QAM16_LOOKUP[first_nibble as usize]);
                    symbols.push(QAM16_LOOKUP[second_nibble as usize]);
                }
            }
        }
        symbols
    }

    /// Demodulate QAM symbols back into bytes.
    ///
    /// Each symbol will be converted back to its corresponding number of bits,
    /// and then grouped into bytes.
    ///
    /// # Example
    /// ```
    /// use software_modem::qam::{ QAMModem, QAMOrder };
    ///
    /// let data = "Hello, world!".as_bytes();
    /// let modem = QAMModem::new(QAMOrder::QAM16);
    /// let symbols = modem.modulate(data);
    /// let demodulated_data = modem.demodulate(&symbols);
    ///
    /// assert_eq!(data, demodulated_data);
    /// ```
    pub fn demodulate(&self, symbols: &[Complex32]) -> Vec<u8> {
        match self.qam_order {
            QAMOrder::QAM16 => {
                let mut nibbles = Vec::new();
                // demulation
                for symbol in symbols {
                    QAM16_LOOKUP
                        .iter()
                        .enumerate()
                        .min_by(|(_, a), (_, b)| {
                            distance(symbol, a)
                                .partial_cmp(&distance(symbol, b))
                                .unwrap()
                        })
                        .map(|(index, _)| {
                            nibbles.push(index as u8);
                        })
                        .unwrap_or_else(|| panic!("Symbol not found in QAM-16 lookup table"));
                }
                // nubbles to bytes
                let mut bytes = Vec::new();
                for chunk in nibbles.chunks(2) {
                    if chunk.len() == 2 {
                        let byte = (chunk[0] << 4) | chunk[1]; // Combine two nibbles into a byte
                        bytes.push(byte);
                    } else {
                        panic!("Invalid chunk size on {} demodulation", self.qam_order);
                    }
                }
                bytes
            }
        }
    }

    /// Returns the number of bits per symbol for the specified QAM order.
    pub fn bits_per_symbol(&self) -> u32 {
        match self.qam_order {
            QAMOrder::QAM16 => 4, // QAM-16 uses 4 bits per symbol
        }
    }
}

fn distance(a: &Complex32, b: &Complex32) -> f32 {
    ((a.re - b.re).powi(2) + (a.im - b.im).powi(2)).sqrt()
}
