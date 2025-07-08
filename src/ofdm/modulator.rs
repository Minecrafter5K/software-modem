use std::sync::Arc;

use realfft::ComplexToReal;
use smart_default::SmartDefault;

use crate::qam::{QAMModulator, QAMOrder};

pub struct OFDMModulator {
    fft: Arc<dyn ComplexToReal<f32>>,
    qam_modulator: QAMModulator,
    constants: OFDMModulatorConstants,
}

impl OFDMModulator {
    pub fn new(config: OFDMModulatorConfig) -> Self {
        let qam_modulator = QAMModulator::new(config.qam_order);

        let num_subcarriers = config.num_subcarriers;
        let num_data_subcarriers = num_subcarriers - config.pilot_subcarrier_every;
        let num_pilot_subcarriers = num_subcarriers / config.pilot_subcarrier_every;

        let bits_per_subcarrier = qam_modulator.bits_per_symbol();
        let bits_per_symbol = num_data_subcarriers * bits_per_subcarrier;

        let data_subcarrier_indices: Vec<u32> = (0..num_subcarriers)
            .filter(|&i| i % config.pilot_subcarrier_every != 0)
            .collect();

        let pilot_subcarrier_indices: Vec<u32> = (0..num_subcarriers)
            .filter(|&i| i % config.pilot_subcarrier_every == 0)
            .collect();

        let constants = OFDMModulatorConstants {
            num_data_subcarriers,
            num_pilot_subcarriers,
            qam_order: config.qam_order,
            num_subcarriers,
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
        let mut input: Vec<realfft::num_complex::Complex<f32>> = self.fft.make_input_vec();
        input[1..=qam_symbols.len()].copy_from_slice(&qam_symbols);

        self.fft.process(&mut input, output_buffer).unwrap();

        Ok(())
    }
}

#[derive(SmartDefault)]
pub struct OFDMModulatorConfig {
    #[default(1024)]
    num_subcarriers: u32,
    #[default(4)]
    pilot_subcarrier_every: u32,
    qam_order: QAMOrder,
    fft: Option<Arc<dyn ComplexToReal<f32>>>,
}

#[allow(dead_code)]
struct OFDMModulatorConstants {
    num_data_subcarriers: u32,
    num_pilot_subcarriers: u32,
    qam_order: QAMOrder,
    num_subcarriers: u32,

    data_subcarrier_indices: Vec<u32>,
    pilot_subcarrier_indices: Vec<u32>,

    bits_per_subcarrier: u32,
    bits_per_symbol: u32,
}
