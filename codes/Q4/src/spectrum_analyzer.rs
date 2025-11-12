use plotters::prelude::*;

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
