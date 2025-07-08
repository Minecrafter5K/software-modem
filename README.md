# software-modem

A software implementation of a QAM/OFDM modem written in pure rust.

The goal is have a fully working OFDM modulator and demodulator for use in RF Systems.

## Modules

1. **QAM**:
    The QAM modulator and demodulator, which maps bits to Complex Numbers representing amplitude/phase combinations and vice verca.

2. **OFDM**
    1. **Modulator**
        Here lives the main code to modulate QAM Symbols (or just any Coordinates on the Complex Plane) to a number of samples in the time domain.
