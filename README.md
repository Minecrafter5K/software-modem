# software-modem

A software implementation of a QAM/OFDM modem written in pure rust.

The goal is have a fully working OFDM modulator and demodulator for use in RF Systems.

## Modules

1. **QAM**:
    The QAM modulator and demodulator, which maps bits to Complex Numbers representing amplitude/phase combinations and vice verca.

2. **OFDM**
    1. **Modulator**
        Here lives the main code to modulate QAM Symbols (or just any Coordinates on the Complex Plane) to a number of samples in the time domain.

## Example

```rs
use software_modem::qam::QAMOrder;
use software_modem::ofdm::modulator::{OFDMModulator, OFDMModulatorConfig};

let ofdm_modulator = OFDMModulator::new(OFDMModulatorConfig {
  num_subcarriers: 64,
  cyclic_prefix_length: 4,
  pilot_subcarrier_every: 4,
  qam_order: QAMOrder::QAM16,
 fft: None,
});

let mut output_buffer = vec![0.0; ofdm_modulator.get_symbol_length()];
let mut data_buffer = vec![0; 32 - 6 - 2]; // 16 bytes for QAM16 * 64 Subcarriers minus 6 pilot subcarriers and first and last subcarrier

let test_data = "Hello, OFDM!";
data_buffer[..test_data.len()].copy_from_slice(test_data.as_bytes());

ofdm_modulator.modulate_buffer_as_symbol(&data_buffer, &mut output_buffer);
```

Now `output_buffer` contains the modulated samples.
