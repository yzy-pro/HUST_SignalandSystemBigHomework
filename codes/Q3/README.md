# Q3: Time-Domain Demodulation

This program implements time-domain demodulation of an AM signal using IIR filters.

## Algorithm

1. **High-pass Filtering**: Apply 8th-order Butterworth high-pass filter (cutoff = f_d)
   - Remove low-frequency components
   - Extract AM signal centered at f_d

2. **Carrier Multiplication**: Multiply with local carrier cos(2πf_d·t)
   - Frequency shift: shifts spectrum to baseband and 2f_d
   - Creates sum and difference frequencies

3. **Low-pass Filtering**: Apply 8th-order Butterworth low-pass filter (cutoff = f_B)
   - Extract baseband signal (difference frequency)
   - Remove high-frequency component at 2f_d

4. **Spectrum Analysis**: Compute FFT for each stage
   - Visualize frequency content changes
   - Verify demodulation success

## IIR Filter Implementation

Uses Direct Form II structure:
```
y[n] = (1/a[0]) * [Σ(b[k]·x[n-k]) - Σ(a[k]·y[n-k])]
```

- Maintains input and output history buffers
- Efficient for real-time processing
- Numerically stable for 8th-order filters

## Modules

- `audio_reader.rs`: Read WAV files
- `iir_filter.rs`: IIR filter implementation (Direct Form II)
- `demodulator.rs`: Carrier generation and multiplication
- `spectrum_analyzer.rs`: FFT and spectrum plotting
- `audio_writer.rs`: Write demodulated WAV file

## Building and Running

```bash
cargo build --release
cargo run --release
```

## Input Files

- `../Q1/output/Q1_results.txt`: Carrier frequency f_d
- `../Q2/output/Q2_filter_coefficients.txt`: Filter coefficients
- `../../工程设计问题-2022/工程设计题15. 调幅信号的解调/ctfymod.wav`: Modulated signal

## Output Files

- `Q3_original_spectrum.png`: Original signal spectrum
- `Q3_xh_spectrum.png`: After high-pass filter
- `Q3_xb_spectrum.png`: After carrier multiplication
- `Q3_xl_spectrum.png`: After low-pass filter (demodulated)
- `Q3_demodulated.wav`: Demodulated audio (can be played)
- `Q3_results.txt`: Numerical analysis results

## Theory

The demodulation process exploits the property:
```
x(t)·cos(2πf_d·t) = (1/2)[x(t-f_d) + x(t+f_d)]
```

For AM signal: s(t) = [A + m(t)]·cos(2πf_c·t)

After high-pass (removing DC):
```
x_h(t) ≈ m(t)·cos(2πf_d·t)
```

After multiplication:
```
x_b(t) = m(t)·cos²(2πf_d·t) = (m(t)/2)·[1 + cos(4πf_d·t)]
```

After low-pass (removing 2f_d component):
```
x_l(t) = m(t)/2
```

Result: Recovered baseband signal m(t) (scaled by 1/2)
