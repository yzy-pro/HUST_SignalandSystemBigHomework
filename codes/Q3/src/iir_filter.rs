/// Apply IIR filter using Direct Form II structure
/// y[n] = sum(b[i] * x[n-i]) - sum(a[j] * y[n-j]) for j > 0
pub fn apply_filter(input: &[f64], b: &[f64], a: &[f64]) -> Vec<f64> {
    let n = input.len();
    let mut output = vec![0.0; n];
    
    let order = b.len().max(a.len());
    let mut x_history = vec![0.0; order]; // Input history
    let mut y_history = vec![0.0; order]; // Output history
    
    for i in 0..n {
        // Shift histories
        for j in (1..order).rev() {
            x_history[j] = x_history[j - 1];
            y_history[j] = y_history[j - 1];
        }
        x_history[0] = input[i];
        
        // Calculate output using difference equation
        let mut y = 0.0;
        
        // Feedforward part: sum(b[k] * x[n-k])
        for k in 0..b.len() {
            y += b[k] * x_history[k];
        }
        
        // Feedback part: -sum(a[k] * y[n-k]) for k > 0
        for k in 1..a.len() {
            y -= a[k] * y_history[k];
        }
        
        // Normalize by a[0] (usually 1.0)
        y /= a[0];
        
        y_history[0] = y;
        output[i] = y;
    }
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_filter() {
        // Simple moving average filter: y[n] = 0.5*x[n] + 0.5*x[n-1]
        let b = vec![0.5, 0.5];
        let a = vec![1.0];
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        
        let output = apply_filter(&input, &b, &a);
        
        // Expected: [0.5, 1.5, 2.5, 3.5, 4.5]
        assert!((output[0] - 0.5).abs() < 1e-10);
        assert!((output[1] - 1.5).abs() < 1e-10);
        assert!((output[2] - 2.5).abs() < 1e-10);
    }
}
