use std::sync::Arc;

use realfft::{ComplexToReal, num_complex::Complex32};
use smart_default::SmartDefault;

use crate::qam::{QAMModem, QAMOrder};

const PILOT_VALUE_TO_BE_CHANGED: Complex32 = Complex32 { re: 1.0, im: 0.0 };

/// OFDM Modulator
///
/// With this modulator, you can modulate data into OFDM symbols.
/// It supports QAM modulation and allows for pilot subcarriers.
/// The modulator can be configured with the number of subcarriers, cyclic prefix length,
/// pilot subcarrier interval, and QAM order.
pub struct OFDMModulator {
    fft: Arc<dyn ComplexToReal<f32>>,
    qam_modulator: QAMModem,
    constants: OFDMModulatorConstants,
}

impl OFDMModulator {
    /// Creates a new OFDM modulator with the given [configuration](OFDMModulatorConfig).
    pub fn new(config: OFDMModulatorConfig) -> Self {
        let qam_modulator = QAMModem::new(config.qam_order);

        let num_subcarriers = config.num_subcarriers;

        let pilot_subcarrier_indices: Vec<u32> = (1..num_subcarriers)
            .filter(|&i| i % config.pilot_subcarrier_every == 0)
            .collect();
        let num_pilot_subcarriers = pilot_subcarrier_indices.len() as u32;

        let data_subcarrier_indices: Vec<u32> = (1..num_subcarriers)
            .filter(|&i| i % config.pilot_subcarrier_every != 0)
            .collect();
        let num_data_subcarriers = data_subcarrier_indices.len() as u32;

        let bits_per_subcarrier = qam_modulator.bits_per_symbol();
        let bits_per_symbol = num_data_subcarriers * bits_per_subcarrier;

        let constants = OFDMModulatorConstants {
            num_data_subcarriers,
            num_pilot_subcarriers,
            qam_order: config.qam_order,
            num_subcarriers,
            cyclic_prefix_length: config.cyclic_prefix_length,
            data_subcarrier_indices,
            pilot_subcarrier_indices,
            bits_per_subcarrier,
            bits_per_symbol,
        };

        let fft = config.fft.unwrap_or_else(|| {
            realfft::RealFftPlanner::<f32>::new().plan_fft_inverse(2 * num_subcarriers as usize)
        });

        OFDMModulator {
            fft,
            qam_modulator,
            constants,
        }
    }

    /// Modulates the given data buffer into an OFDM symbol.
    ///
    /// The data buffer must have a length equal to the number of bytes per symbol,
    /// which is determined by the QAM order and the number of data subcarriers.
    ///
    /// The length of the output buffer must be double the total length of the OFDM symbol plus the cyclic prefix length.
    /// You can calculate the expected length of the output buffer using `get_symbol_length()`.
    ///
    /// # Panics
    /// If the data length does not match the expected length,
    /// which is `bits_per_symbol / 8`.
    ///
    /// # Arguments
    /// - `data` - A slice of bytes to be modulated.
    /// - `output_buffer` - A mutable slice where the modulated OFDM symbol will be written.
    ///
    /// # Example
    /// ```
    /// use software_modem::ofdm::modulator::{OFDMModulator, OFDMModulatorConfig};
    /// use software_modem::qam::QAMOrder;
    ///
    /// let ofdm_modulator = OFDMModulator::new(OFDMModulatorConfig {
    ///   num_subcarriers: 64,
    ///   cyclic_prefix_length: 4,
    ///   pilot_subcarrier_every: 4,
    ///   qam_order: QAMOrder::QAM16,
    ///  fft: None,
    /// });
    ///
    /// let mut output_buffer = vec![0.0; ofdm_modulator.get_symbol_length()];
    /// let mut data_buffer = vec![0; 32 - 6 - 2]; // 16 bytes for QAM16 * 64 Subcarriers minus 6 pilot subcarriers and first and last subcarrier
    /// let test_data = "Hello, OFDM!";
    /// data_buffer[..test_data.len()].copy_from_slice(test_data.as_bytes());
    ///
    /// ofdm_modulator.modulate_buffer_as_symbol(&data_buffer, &mut output_buffer);
    /// ```
    pub fn modulate_buffer_as_symbol(&self, data: &[u8], output_buffer: &mut [f32]) {
        if data.len() != ((self.constants.bits_per_symbol / 8) as usize) {
            panic!(
                "Data length must be {} bytes, but got {} bytes",
                self.constants.bits_per_symbol / 8,
                data.len()
            );
        }

        let qam_symbols = self.qam_modulator.modulate(data);

        self.modulate_ofdm_symbol(qam_symbols, output_buffer)
            .unwrap();
    }

    fn modulate_ofdm_symbol(
        &self,
        qam_symbols: Vec<realfft::num_complex::Complex<f32>>,
        output: &mut [f32],
    ) -> Result<(), String> {
        // data prep
        let mut input: Vec<realfft::num_complex::Complex<f32>> = self.fft.make_input_vec();

        for (i, &idx) in self.constants.data_subcarrier_indices.iter().enumerate() {
            input[idx as usize] = qam_symbols[i];
        }

        for &idx in &self.constants.pilot_subcarrier_indices {
            input[idx as usize] = PILOT_VALUE_TO_BE_CHANGED;
        }

        let mut output_buffer = self.fft.make_output_vec();

        // frequency domain to time domain
        self.fft.process(&mut input, &mut output_buffer).unwrap();

        // add cp
        output[..self.get_symbol_length() - self.constants.cyclic_prefix_length as usize]
            .copy_from_slice(&output_buffer);

        output[self.get_symbol_length() - self.constants.cyclic_prefix_length as usize..]
            .copy_from_slice(&output_buffer[..self.constants.cyclic_prefix_length as usize]);

        Ok(())
    }

    /// Returns the length of the OFDM symbol, including the cyclic prefix.
    ///
    /// The length is calculated as:
    /// `2 * num_subcarriers + cyclic_prefix_length`.
    pub fn get_symbol_length(&self) -> usize {
        (2 * self.constants.num_subcarriers + self.constants.cyclic_prefix_length) as usize
    }
}

/// Configuration for the [OFDM Modulator](OFDMModulator).
///
/// Just contruct this struct with the desired parameters and pass it to the `OFDMModulator::new()` method.
#[derive(SmartDefault)]
pub struct OFDMModulatorConfig {
    pub num_subcarriers: u32,
    /// Length of the cyclic prefix in samples.
    ///
    /// One OFDM symbol double num_subcarriers samples. If you want to have a CP of 1/4 you need to set this to `(2 * num_subcarriers) / 4`
    pub cyclic_prefix_length: u32,
    /// Interval for pilot subcarriers.
    ///
    /// Inserts pilot subcarriers every `pilot_subcarrier_every` subcarrier.
    #[default(4)]
    pub pilot_subcarrier_every: u32,
    pub qam_order: QAMOrder,
    /// Optional FFT implementation/planner to use.
    ///
    /// If `None`, a default FFT planner will be used.
    pub fft: Option<Arc<dyn ComplexToReal<f32>>>,
}

#[allow(dead_code)]
struct OFDMModulatorConstants {
    num_data_subcarriers: u32,
    num_pilot_subcarriers: u32,
    qam_order: QAMOrder,
    num_subcarriers: u32,
    cyclic_prefix_length: u32,

    data_subcarrier_indices: Vec<u32>,
    pilot_subcarrier_indices: Vec<u32>,

    bits_per_subcarrier: u32,
    bits_per_symbol: u32,
}
