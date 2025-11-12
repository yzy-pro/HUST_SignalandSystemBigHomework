use std::f64::consts::PI;

/// Multiply signal with carrier cos(2*pi*f_d*t)
pub fn multiply_with_carrier(signal: &[f64], f_d: f64, f_s: f64) -> Vec<f64> {
    let n = signal.len();
    let mut output = vec![0.0; n];
    
    for i in 0..n {
        let t = i as f64 / f_s;
        let carrier = (2.0 * PI * f_d * t).cos();
        // Multiply by 2 to compensate for the 1/2 factor from cos²(x) = (1 + cos(2x))/2
        output[i] = signal[i] * carrier * 2.0;
    }
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carrier_multiplication() {
        let f_d = 1000.0;
        let f_s = 8000.0;
        let signal = vec![1.0; 100];
        
        let output = multiply_with_carrier(&signal, f_d, f_s);
        
        // Output should oscillate with carrier frequency
        assert_eq!(output.len(), 100);
        // At t=0, cos(0) = 1.0
        assert!((output[0] - 1.0).abs() < 1e-10);
    }
}
