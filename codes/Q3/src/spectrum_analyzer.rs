use rustfft::{FftPlanner, num_complex::Complex};
use plotters::prelude::*;

/// Compute magnitude spectrum of a signal
pub fn compute_spectrum(signal: &[f64], f_s: f64) -> Vec<(f64, f64)> {
    let n = signal.len();
    
    // Prepare FFT input
    let mut buffer: Vec<Complex<f64>> = signal
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();
    
    // Perform FFT
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n);
    fft.process(&mut buffer);
    
    // Compute magnitude spectrum and frequency axis
    let mut spectrum = Vec::with_capacity(n / 2);
    let df = f_s / n as f64;
    
    for i in 0..n / 2 {
        let freq = i as f64 * df;
        let magnitude = buffer[i].norm() / n as f64;
        spectrum.push((freq, magnitude));
    }
    
    spectrum
}

/// Plot spectrum
pub fn plot_spectrum(spectrum: &[(f64, f64)], filename: &str, title: &str) {
    let root = BitMapBackend::new(filename, (1200, 800)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    
    // Find max magnitude for y-axis
    let max_mag = spectrum.iter()
        .map(|(_, m)| *m)
        .fold(0.0f64, f64::max);
    
    let max_freq = spectrum.last().unwrap().0;
    
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..max_freq, 0.0..max_mag * 1.1)
        .unwrap();
    
    chart
        .configure_mesh()
        .x_desc("Frequency (Hz)")
        .y_desc("Magnitude")
        .x_label_formatter(&|x| format!("{:.0}", x))
        .y_label_formatter(&|y| format!("{:.3}", y))
        .draw()
        .unwrap();
    
    chart
        .draw_series(LineSeries::new(
            spectrum.iter().map(|(f, m)| (*f, *m)),
            &BLUE,
        ))
        .unwrap();
    
    root.present().unwrap();
    println!("  Saved: {}", filename);
}

/// Plot spectrum in dB scale
pub fn plot_spectrum_db(spectrum: &[(f64, f64)], filename: &str, title: &str) {
    let root = BitMapBackend::new(filename, (1200, 800)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    
    // Convert to dB
    let spectrum_db: Vec<(f64, f64)> = spectrum
        .iter()
        .map(|(f, m)| {
            let db = if *m > 1e-10 {
                20.0 * m.log10()
            } else {
                -200.0
            };
            (*f, db)
        })
        .collect();
    
    let max_freq = spectrum_db.last().unwrap().0;
    
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..max_freq, -100.0..0.0)
        .unwrap();
    
    chart
        .configure_mesh()
        .x_desc("Frequency (Hz)")
        .y_desc("Magnitude (dB)")
        .x_label_formatter(&|x| format!("{:.0}", x))
        .y_label_formatter(&|y| format!("{:.0}", y))
        .draw()
        .unwrap();
    
    chart
        .draw_series(LineSeries::new(
            spectrum_db.iter().map(|(f, m)| (*f, *m)),
            &RED,
        ))
        .unwrap();
    
    root.present().unwrap();
    println!("  Saved: {}", filename);
}
