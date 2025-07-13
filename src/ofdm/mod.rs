//! This module provides the OFDM implementation.
//!
//! The [OFDM Modulator](modulator) modulates data into OFDM symbols.
//! And the [OFDM Demodulator](demodulator) demodulates OFDM symbols back into data.

use crate::qam::QAMOrder;

pub mod demodulator;
pub mod modulator;

#[allow(dead_code)]
struct OFDMConstants {
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
impl OFDMConstants {
    fn new(
        num_subcarriers: u32,
        pilot_subcarrier_every: u32,
        cyclic_prefix_length: u32,
        qam_order: QAMOrder,
        bits_per_subcarrier: u32,
    ) -> Self {
        let pilot_subcarrier_indices: Vec<u32> = (1..num_subcarriers)
            .filter(|&i| i % pilot_subcarrier_every == 0)
            .collect();
        let num_pilot_subcarriers = pilot_subcarrier_indices.len() as u32;

        let data_subcarrier_indices: Vec<u32> = (1..num_subcarriers)
            .filter(|&i| i % pilot_subcarrier_every != 0)
            .collect();
        let num_data_subcarriers = data_subcarrier_indices.len() as u32;

        let bits_per_symbol = num_data_subcarriers * bits_per_subcarrier;

        OFDMConstants {
            num_data_subcarriers,
            num_pilot_subcarriers,
            qam_order,
            num_subcarriers,
            cyclic_prefix_length,
            data_subcarrier_indices,
            pilot_subcarrier_indices,
            bits_per_subcarrier,
            bits_per_symbol,
        }
    }
}
