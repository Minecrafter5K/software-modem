use std::sync::Arc;

use realfft::ComplexToReal;

use crate::{BYTES_PER_OFDM_SYMBOL, qam};

mod modulator;

/// Modulates a byte array of set length into one OFDM symbol.
pub fn modulate_ofdm_symbol(
    data: &[u8; BYTES_PER_OFDM_SYMBOL],
    output_buffer: &mut [f32],
    fft: Arc<dyn ComplexToReal<f32>>,
) {
    let qam_symbols = qam::modulate_qam16(data);

    modulator::modulate_ofdm_symbol(qam_symbols, output_buffer, fft).unwrap();
}
