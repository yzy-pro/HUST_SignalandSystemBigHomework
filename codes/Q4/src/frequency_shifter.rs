use num_complex::Complex;

/// Perform frequency shift: X_b(f) = X_h(f - f_d) + X_h(f + f_d)
/// This is equivalent to multiplying by cos(2πf_d·t) in time domain
pub fn frequency_shift(
    spectrum: &[Complex<f64>],
    f_d: f64,
    f_s: f64,
    n: usize,
) -> Vec<Complex<f64>> {
    // Calculate shift amount in bins
    let shift_bins = (f_d * n as f64 / f_s).round() as isize;
    
    // Create result vector
    let mut result = vec![Complex::new(0.0, 0.0); n];
    
    // For each output frequency bin, find contributions from shifted input
    for i in 0..n {
        // Contribution from f - f_d (shift down)
        let idx_minus = (i as isize + shift_bins).rem_euclid(n as isize) as usize;
        result[i] = result[i] + spectrum[idx_minus] * 0.5;
        
        // Contribution from f + f_d (shift up)
        let idx_plus = (i as isize - shift_bins).rem_euclid(n as isize) as usize;
        result[i] = result[i] + spectrum[idx_plus] * 0.5;
    }
    
    result
}

/// Alternative implementation using explicit frequency mapping
pub fn frequency_shift_explicit(
    spectrum: &[Complex<f64>],
    f_d: f64,
    f_s: f64,
    n: usize,
) -> Vec<Complex<f64>> {
    let df = f_s / n as f64;
    let mut result = vec![Complex::new(0.0, 0.0); n];
    
    for i in 0..n {
        // Current frequency
        let f_i = if i <= n / 2 {
            i as f64 * df
        } else {
            (i as f64 - n as f64) * df
        };
        
        // Find indices for f ± f_d
        let f_minus = f_i - f_d;
        let f_plus = f_i + f_d;
        
        // Map back to bin indices
        let idx_minus = freq_to_bin(f_minus, f_s, n);
        let idx_plus = freq_to_bin(f_plus, f_s, n);
        
        // Accumulate contributions
        if idx_minus < n {
            result[i] = result[i] + spectrum[idx_minus] * 0.5;
        }
        if idx_plus < n {
            result[i] = result[i] + spectrum[idx_plus] * 0.5;
        }
    }
    
    result
}

fn freq_to_bin(freq: f64, f_s: f64, n: usize) -> usize {
    let normalized = if freq >= 0.0 {
        freq
    } else {
        freq + f_s
    };
    
    let bin = (normalized * n as f64 / f_s).round() as isize;
    bin.rem_euclid(n as isize) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_shift() {
        let n = 1000;
        let f_s = 10000.0;
        let f_d = 1000.0;
        
        // Create impulse at 2000 Hz
        let mut spectrum = vec![Complex::new(0.0, 0.0); n];
        let impulse_idx = (2000.0 / f_s * n as f64) as usize;
        spectrum[impulse_idx] = Complex::new(1.0, 0.0);
        
        let shifted = frequency_shift(&spectrum, f_d, f_s, n);
        
        // Should have peaks at 1000 Hz and 3000 Hz
        let idx_1000 = (1000.0 / f_s * n as f64) as usize;
        let idx_3000 = (3000.0 / f_s * n as f64) as usize;
        
        assert!(shifted[idx_1000].norm() > 0.4);
        assert!(shifted[idx_3000].norm() > 0.4);
    }

    #[test]
    fn test_freq_to_bin() {
        let n = 100;
        let f_s = 1000.0;
        
        assert_eq!(freq_to_bin(0.0, f_s, n), 0);
        assert_eq!(freq_to_bin(100.0, f_s, n), 10);
        assert_eq!(freq_to_bin(-100.0, f_s, n), 90);
    }
}
