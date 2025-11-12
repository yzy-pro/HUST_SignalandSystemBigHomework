# Q4: Frequency-Domain Demodulation

This program implements frequency-domain demodulation of an AM signal using ideal filters.

## Algorithm

1. **FFT Transformation**: Convert time-domain signal to frequency domain
   - Input: x(t) → X(f) via FFT

2. **Ideal High-Pass Filtering**: Apply brick-wall high-pass filter
   - H_h(f) = 0 for |f| < f_d, 1 for |f| ≥ f_d
   - X_h(f) = H_h(f) · X(f)

3. **Frequency Shift**: Shift spectrum to baseband
   - X_b(f) = 0.5 · [X_h(f - f_d) + X_h(f + f_d)]
   - Equivalent to carrier multiplication in time domain

4. **Ideal Low-Pass Filtering**: Extract baseband signal
   - H_l(f) = 1 for |f| ≤ f_B, 0 for |f| > f_B
   - X_l(f) = H_l(f) · X_b(f)

5. **IFFT Transformation**: Convert back to time domain
   - Output: X_l(f) → x_l(t) via IFFT

6. **Comparison with Q3**: Compare frequency-domain vs time-domain methods

## Key Differences from Q3 (Time-Domain)

### Q3 (Time-Domain Method)
- Uses 8th-order Butterworth IIR filters
- Gradual transition band
- Non-linear phase response
- Convolution in time domain
- Direct Form II implementation

### Q4 (Frequency-Domain Method)
- Uses ideal brick-wall filters
- Sharp cutoff (perfect selectivity)
- No phase distortion from filters
- Multiplication in frequency domain
- Circular shift for frequency translation

## Advantages of Frequency-Domain Method

1. **Perfect Frequency Selectivity**: Ideal filters have sharp cutoffs
2. **No Phase Distortion**: Filters don't introduce phase distortion
3. **Computational Efficiency**: FFT-based processing is efficient for large signals
4. **Flexibility**: Easy to implement any frequency response

## Disadvantages

1. **Time-Domain Artifacts**: Ideal filters cause ringing (Gibbs phenomenon)
2. **Block Processing**: Requires processing entire signal at once
3. **Memory Usage**: Must store entire FFT
4. **Non-Causal**: Cannot be implemented in real-time

## Modules

- `audio_reader.rs`: Read WAV files
- `ideal_filter.rs`: Ideal high-pass and low-pass filters
- `frequency_shifter.rs`: Frequency shift (circular shift in FFT)
- `spectrum_analyzer.rs`: Spectrum plotting
- `audio_writer.rs`: Write demodulated WAV file
- `comparator.rs`: Compare Q3 and Q4 results

## Building and Running

```bash
cargo build --release
cargo run --release
```

## Input Files

- `../Q1/output/Q1_results.txt`: Carrier frequency f_d
- `../../工程设计问题-2022/工程设计题15. 调幅信号的解调/project.wav`: Modulated signal
- `../Q3/output/Q3_demodulated.wav`: Q3 results for comparison

## Output Files

- `Q4_original_spectrum.png`: Original signal spectrum
- `Q4_xh_spectrum.png`: After ideal high-pass filter
- `Q4_xb_spectrum.png`: After frequency shift
- `Q4_xl_spectrum.png`: After ideal low-pass filter (demodulated)
- `Q4_demodulated.wav`: Demodulated audio (can be played)
- `Q4_results.txt`: Numerical analysis results
- `Q4_comparison.txt`: Q3 vs Q4 comparison metrics
- `Q4_vs_Q3_comparison.png`: Visual comparison plot

## Theory

### Ideal High-Pass Filter

```
H_h(f) = { 0,  |f| < f_d
         { 1,  |f| ≥ f_d
```

### Frequency Shift

Multiplication by cos(2πf_d·t) in time domain is equivalent to:
```
X_b(f) = 0.5 · [X_h(f - f_d) + X_h(f + f_d)]
```

Implemented as circular shift in FFT domain.

### Ideal Low-Pass Filter

```
H_l(f) = { 1,  |f| ≤ f_B
         { 0,  |f| > f_B
```

### Complete Process

```
x(t) → FFT → X(f)
         ↓
    H_h(f)·X(f) → X_h(f)
         ↓
  Shift ±f_d → X_b(f)
         ↓
    H_l(f)·X_b(f) → X_l(f)
         ↓
      IFFT → x_l(t)
```

## Comparison Metrics

- **MSE**: Mean Squared Error between Q3 and Q4
- **Max Difference**: Maximum absolute difference
- **Correlation**: Correlation coefficient (ideally close to 1.0)
- **SNR**: Signal-to-Noise Ratio (treating difference as noise)

## Expected Results

- High correlation (> 0.95) between Q3 and Q4
- Some differences due to filter characteristics
- Q4 may have sharper transitions
- Q4 may show time-domain ringing near edges
