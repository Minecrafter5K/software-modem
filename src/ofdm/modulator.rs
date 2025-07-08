use std::sync::Arc;

use realfft::ComplexToReal;

pub(crate) fn modulate_ofdm_symbol(
    qam_symbols: Vec<realfft::num_complex::Complex<f32>>,
    output_buffer: &mut [f32],
    fft: Arc<dyn ComplexToReal<f32>>,
) -> Result<(), String> {
    let mut input: Vec<realfft::num_complex::Complex<f32>> = fft.make_input_vec();
    input[1..=qam_symbols.len()].copy_from_slice(&qam_symbols);

    fft.process(&mut input, output_buffer).unwrap();

    Ok(())
}
