# Q2: Butterworth Filter Design

This program designs 8th-order Butterworth filters (high-pass and low-pass) for AM signal demodulation.

## Features

- **High-pass Filter**: Cutoff frequency = f_d (estimated from Q1)
- **Low-pass Filter**: Cutoff frequency = f_B = 4000 Hz
- **Filter Order**: 8th-order for sharp frequency response
- **Design Method**: Bilinear transformation from analog to digital domain

## Filter Design

### Butterworth Filter Characteristics
- Maximally flat magnitude response in passband
- Monotonic magnitude response
- 3 dB attenuation at cutoff frequency

### Design Steps
1. Calculate analog Butterworth poles
2. Scale poles by cutoff frequency
3. Apply bilinear transform: s = 2*fs*(z-1)/(z+1)
4. Calculate digital filter coefficients (b, a)

## Output Files

### Plots
- `Q2_highpass_magnitude.png` - High-pass filter magnitude response
- `Q2_highpass_magnitude_db.png` - High-pass filter magnitude (dB scale)
- `Q2_highpass_phase.png` - High-pass filter phase response
- `Q2_lowpass_magnitude.png` - Low-pass filter magnitude response
- `Q2_lowpass_magnitude_db.png` - Low-pass filter magnitude (dB scale)
- `Q2_lowpass_phase.png` - Low-pass filter phase response
- `Q2_combined_magnitude.png` - Combined magnitude responses

### Data Files
- `Q2_filter_coefficients.txt` - Filter coefficients (b and a arrays)
- `Q2_frequency_response.txt` - Frequency response statistics

## Usage

```bash
cd codes/Q2
cargo build --release
cargo run --release
```

## Dependencies

- `num-complex` - Complex number operations
- `plotters` - Visualization of frequency responses

## Mathematical Background

### Butterworth Poles (N=8)
For 8th-order filter, poles are located at:
θ_k = π(2k + 9)/(16), k = 0, 1, ..., 7

### Magnitude Response
|H(jω)|² = 1 / (1 + (ω/ω_c)^(2N))

where N = 8 (filter order), ω_c = cutoff frequency

### Bilinear Transform
Digital filter from analog filter:
H(z) = H_analog(s)|_(s = 2*f_s*(z-1)/(z+1))

Pre-warping applied to maintain cutoff frequency:
ω_c_digital = 2*f_s*tan(π*f_c/f_s)
