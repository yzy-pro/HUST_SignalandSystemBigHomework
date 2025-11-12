use num_complex::Complex;

/// Apply ideal high-pass filter in frequency domain
/// H_h(f) = 0 for |f| < f_c, 1 for |f| >= f_c
pub fn apply_highpass(
    spectrum: &[Complex<f64>],
    f_c: f64,
    f_s: f64,
    n: usize,
) -> Vec<Complex<f64>> {
    let df = f_s / n as f64;
    let mut result = spectrum.to_vec();
    
    for i in 0..n {
        // Calculate frequency for this bin
        let freq = if i <= n / 2 {
            i as f64 * df
        } else {
            (i as f64 - n as f64) * df
        };
        
        // Apply ideal high-pass filter
        if freq.abs() < f_c {
            result[i] = Complex::new(0.0, 0.0);
        }
    }
    
    result
}

/// Apply ideal low-pass filter in frequency domain
/// H_l(f) = 1 for |f| <= f_c, 0 for |f| > f_c
pub fn apply_lowpass(
    spectrum: &[Complex<f64>],
    f_c: f64,
    f_s: f64,
    n: usize,
) -> Vec<Complex<f64>> {
    let df = f_s / n as f64;
    let mut result = spectrum.to_vec();
    
    for i in 0..n {
        // Calculate frequency for this bin
        let freq = if i <= n / 2 {
            i as f64 * df
        } else {
            (i as f64 - n as f64) * df
        };
        
        // Apply ideal low-pass filter
        if freq.abs() > f_c {
            result[i] = Complex::new(0.0, 0.0);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highpass_filter() {
        let n = 100;
        let f_s = 1000.0;
        let f_c = 100.0;
        
        // Create test spectrum
        let spectrum: Vec<Complex<f64>> = (0..n)
            .map(|_| Complex::new(1.0, 0.0))
            .collect();
        
        let filtered = apply_highpass(&spectrum, f_c, f_s, n);
        
        // Check that low frequencies are zeroed
        assert_eq!(filtered[0].norm(), 0.0); // DC component
        assert_eq!(filtered[5].norm(), 0.0); // 50 Hz
    }

    #[test]
    fn test_lowpass_filter() {
        let n = 100;
        let f_s = 1000.0;
        let f_c = 100.0;
        
        let spectrum: Vec<Complex<f64>> = (0..n)
            .map(|_| Complex::new(1.0, 0.0))
            .collect();
        
        let filtered = apply_lowpass(&spectrum, f_c, f_s, n);
        
        // Check that high frequencies are zeroed
        let high_freq_idx = (200.0 / f_s * n as f64) as usize;
        assert_eq!(filtered[high_freq_idx].norm(), 0.0);
    }
}
