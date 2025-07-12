use std::sync::Arc;

use realfft::{ComplexToReal, num_complex::Complex32};
use smart_default::SmartDefault;

use crate::qam::{QAMModem, QAMOrder};

const PILOT_VALUE_TO_BE_CHANGED: Complex32 = Complex32 { re: 1.0, im: 0.0 };

pub struct OFDMModulator {
    fft: Arc<dyn ComplexToReal<f32>>,
    qam_modulator: QAMModem,
    constants: OFDMModulatorConstants,
}

impl OFDMModulator {
    pub fn new(config: OFDMModulatorConfig) -> Self {
        let qam_modulator = QAMModem::new(config.qam_order);

        let num_subcarriers = config.num_subcarriers;

        let pilot_subcarrier_indices: Vec<u32> = (1..(num_subcarriers - 1))
            .filter(|&i| i % config.pilot_subcarrier_every == 0)
            .collect();
        let num_pilot_subcarriers = pilot_subcarrier_indices.len() as u32;

        let data_subcarrier_indices: Vec<u32> = (1..(num_subcarriers - 1))
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
            cycle_prefix_length: config.cycle_prefix_length,
            data_subcarrier_indices,
            pilot_subcarrier_indices,
            bits_per_subcarrier,
            bits_per_symbol,
        };

        let fft = config.fft.unwrap_or_else(|| {
            realfft::RealFftPlanner::<f32>::new().plan_fft_inverse(num_subcarriers as usize)
        });

        OFDMModulator {
            fft,
            qam_modulator,
            constants,
        }
    }

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
        output_buffer: &mut [f32],
    ) -> Result<(), String> {
        // data prep
        let mut input: Vec<realfft::num_complex::Complex<f32>> = self.fft.make_input_vec();

        for (i, &idx) in self.constants.data_subcarrier_indices.iter().enumerate() {
            input[idx as usize] = qam_symbols[i];
        }

        for &idx in &self.constants.pilot_subcarrier_indices {
            input[idx as usize] = PILOT_VALUE_TO_BE_CHANGED;
        }

        // frequency domain to time domain
        self.fft.process(&mut input, output_buffer).unwrap();

        // add cp
        output_buffer[self.constants.cycle_prefix_length as usize..]
            .copy_from_slice(&output_buffer[..self.constants.cycle_prefix_length as usize]);

        Ok(())
    }

    pub fn get_symbol_length(&self) -> usize {
        (self.constants.num_subcarriers + self.constants.cycle_prefix_length) as usize
    }
}

#[derive(SmartDefault)]
pub struct OFDMModulatorConfig {
    #[default(1024)]
    pub num_subcarriers: u32,
    #[default(16)]
    pub cycle_prefix_length: u32,
    #[default(4)]
    pub pilot_subcarrier_every: u32,
    pub qam_order: QAMOrder,
    pub fft: Option<Arc<dyn ComplexToReal<f32>>>,
}

#[allow(dead_code)]
struct OFDMModulatorConstants {
    num_data_subcarriers: u32,
    num_pilot_subcarriers: u32,
    qam_order: QAMOrder,
    num_subcarriers: u32,
    cycle_prefix_length: u32,

    data_subcarrier_indices: Vec<u32>,
    pilot_subcarrier_indices: Vec<u32>,

    bits_per_subcarrier: u32,
    bits_per_symbol: u32,
}
