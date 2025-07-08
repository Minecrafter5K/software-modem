use realfft::RealFftPlanner;

use crate::ofdm::modulate_ofdm_symbol;

mod ofdm;
mod qam;

const SUBCARRIER_COUNT: usize = 62;
const QAM_ORDER: usize = 16; // QAM-16
const BITS_PER_QAM_SYMBOL: usize = QAM_ORDER.ilog2() as usize;
const BITS_PER_OFDM_SYMBOL: usize = SUBCARRIER_COUNT * BITS_PER_QAM_SYMBOL;
const BYTES_PER_OFDM_SYMBOL: usize = BITS_PER_OFDM_SYMBOL / 8;

fn main() {
    let data = "Hello, world!".as_bytes();
    let data_for_symbol: [u8; BYTES_PER_OFDM_SYMBOL] = {
        let mut arr = [0u8; BYTES_PER_OFDM_SYMBOL];
        arr[..data.len()].copy_from_slice(data);
        arr
    };

    let mut planner = RealFftPlanner::<f32>::new();
    let fft = planner.plan_fft_inverse(128);

    let mut buffer = fft.make_output_vec();
    modulate_ofdm_symbol(&data_for_symbol, &mut buffer, fft);
}
