mod audio_reader;
mod iir_filter;
mod demodulator;
mod spectrum_analyzer;
mod audio_writer;

use std::f64::consts::PI;

fn main() {
    println!("Q3: Time-Domain Demodulation");
    println!("================================");

    // Step 1: Read Q1 results to get f_d, f_s, f_B
    println!("\n[Step 1] Reading Q1 results...");
    let (f_d, f_s, f_b) = match read_q1_results() {
        Ok(params) => params,
        Err(e) => {
            eprintln!("Error reading Q1 results: {}", e);
            return;
        }
    };
    println!("  f_d = {:.4} Hz", f_d);
    println!("  f_s = {:.4} Hz", f_s);
    println!("  f_B = {:.4} Hz", f_b);

    // Step 2: Read Q2 filter coefficients
    println!("\n[Step 2] Reading Q2 filter coefficients...");
    let (hp_b, hp_a, lp_b, lp_a) = match read_q2_filters() {
        Ok(filters) => filters,
        Err(e) => {
            eprintln!("Error reading Q2 filters: {}", e);
            return;
        }
    };
    println!("  High-pass filter: {} b coefficients, {} a coefficients", hp_b.len(), hp_a.len());
    println!("  Low-pass filter: {} b coefficients, {} a coefficients", lp_b.len(), lp_a.len());

    // Step 3: Read audio signal
    println!("\n[Step 3] Reading audio signal...");
    let audio_samples = match audio_reader::read_wav("../../工程设计问题-2022/工程设计题15. 调幅信号的解调/project.wav") {
        Ok(samples) => samples,
        Err(e) => {
            eprintln!("Error reading audio: {}", e);
            return;
        }
    };
    println!("  Number of samples: {}", audio_samples.len());
    let max_orig = audio_samples.iter().fold(0.0f64, |max, &x| max.max(x.abs()));
    println!("  Signal max: {:.6}", max_orig);

    // Step 4: Apply high-pass filter
    println!("\n[Step 4] Applying high-pass filter...");
    let x_h = iir_filter::apply_filter(&audio_samples, &hp_b, &hp_a);
    println!("  Output samples: {}", x_h.len());
    let max_xh = x_h.iter().fold(0.0f64, |max, &x| max.max(x.abs()));
    println!("  Signal max: {:.6}", max_xh);

    // Step 5: Generate carrier and multiply
    println!("\n[Step 5] Multiplying with carrier signal (f_d = {:.4} Hz)...", f_d);
    let x_b = demodulator::multiply_with_carrier(&x_h, f_d, f_s);
    println!("  Output samples: {}", x_b.len());
    let max_xb = x_b.iter().fold(0.0f64, |max, &x| max.max(x.abs()));
    println!("  Signal max: {:.6}", max_xb);

    // Step 6: Apply low-pass filter
    println!("\n[Step 6] Applying low-pass filter...");
    let x_l = iir_filter::apply_filter(&x_b, &lp_b, &lp_a);
    println!("  Output samples: {}", x_l.len());
    
    // Debug: Check signal statistics
    let max_val = x_l.iter().fold(0.0f64, |max, &x| max.max(x.abs()));
    let mean_val = x_l.iter().sum::<f64>() / x_l.len() as f64;
    println!("  Signal range: max = {:.6}, mean = {:.6}", max_val, mean_val);

    // Step 7: Spectrum analysis
    println!("\n[Step 7] Performing spectrum analysis...");
    let original_spectrum = spectrum_analyzer::compute_spectrum(&audio_samples, f_s);
    let xh_spectrum = spectrum_analyzer::compute_spectrum(&x_h, f_s);
    let xb_spectrum = spectrum_analyzer::compute_spectrum(&x_b, f_s);
    let xl_spectrum = spectrum_analyzer::compute_spectrum(&x_l, f_s);

    // Step 8: Create output directory
    std::fs::create_dir_all("output").expect("Failed to create output directory");

    // Step 9: Plot spectra
    println!("\n[Step 8] Plotting spectra...");
    spectrum_analyzer::plot_spectrum(&original_spectrum, "output/Q3_original_spectrum.png", "Original Signal X(f)");
    spectrum_analyzer::plot_spectrum(&xh_spectrum, "output/Q3_xh_spectrum.png", "After High-Pass X_h(f)");
    spectrum_analyzer::plot_spectrum(&xb_spectrum, "output/Q3_xb_spectrum.png", "After Multiplication X_b(f)");
    spectrum_analyzer::plot_spectrum(&xl_spectrum, "output/Q3_xl_spectrum.png", "After Low-Pass X_l(f) - Demodulated");

    // Step 10: Save demodulated audio
    println!("\n[Step 9] Saving demodulated audio...");
    match audio_writer::write_wav("output/Q3_demodulated.wav", &x_l, f_s as u32) {
        Ok(_) => println!("  Saved to: output/Q3_demodulated.wav"),
        Err(e) => eprintln!("  Error saving audio: {}", e),
    }

    // Step 11: Save analysis results
    println!("\n[Step 10] Saving analysis results...");
    save_results(&original_spectrum, &xh_spectrum, &xb_spectrum, &xl_spectrum, f_d, f_s);

    println!("\nQ3 Time-Domain Demodulation completed successfully!");
    println!("Output files saved in: codes/Q3/output/");
}

fn read_q1_results() -> Result<(f64, f64, f64), String> {
    let content = std::fs::read_to_string("../Q1/output/Q1_results.txt")
        .map_err(|e| format!("Failed to read Q1 results: {}", e))?;

    let mut f_d = None;
    let mut f_s = None;

    for line in content.lines() {
        if line.contains("频率偏差") || line.contains("f_d") {
            if let Some(value_str) = line.split('=').nth(1) {
                if let Ok(value) = value_str.trim().split_whitespace().next().unwrap_or("0").parse::<f64>() {
                    f_d = Some(value);
                }
            }
        } else if line.contains("采样频率") || line.contains("f_s") {
            if let Some(value_str) = line.split('=').nth(1) {
                if let Ok(value) = value_str.trim().split_whitespace().next().unwrap_or("0").parse::<f64>() {
                    f_s = Some(value);
                }
            }
        }
    }

    let f_d = f_d.ok_or_else(|| "Could not find f_d in Q1 results".to_string())?;
    let f_s = f_s.ok_or_else(|| "Could not find f_s in Q1 results".to_string())?;
    let f_b = 4000.0; // Given in problem statement

    Ok((f_d, f_s, f_b))
}

fn read_q2_filters() -> Result<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>), String> {
    let content = std::fs::read_to_string("../Q2/output/Q2_filter_coefficients.txt")
        .map_err(|e| format!("Failed to read Q2 filters: {}", e))?;

    let mut hp_b = Vec::new();
    let mut hp_a = Vec::new();
    let mut lp_b = Vec::new();
    let mut lp_a = Vec::new();
    
    let mut current_section = "";
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        if line.contains("High-pass Filter") {
            current_section = "hp";
        } else if line.contains("Low-pass Filter") {
            current_section = "lp";
        } else if line.starts_with("b[") {
            if let Some(value_str) = line.split('=').nth(1) {
                if let Ok(value) = value_str.trim().parse::<f64>() {
                    match current_section {
                        "hp" => hp_b.push(value),
                        "lp" => lp_b.push(value),
                        _ => {}
                    }
                }
            }
        } else if line.starts_with("a[") {
            if let Some(value_str) = line.split('=').nth(1) {
                if let Ok(value) = value_str.trim().parse::<f64>() {
                    match current_section {
                        "hp" => hp_a.push(value),
                        "lp" => lp_a.push(value),
                        _ => {}
                    }
                }
            }
        }
    }

    if hp_b.is_empty() || hp_a.is_empty() || lp_b.is_empty() || lp_a.is_empty() {
        return Err("Failed to parse filter coefficients".to_string());
    }

    Ok((hp_b, hp_a, lp_b, lp_a))
}

fn save_results(
    original: &[(f64, f64)],
    xh: &[(f64, f64)],
    xb: &[(f64, f64)],
    xl: &[(f64, f64)],
    f_d: f64,
    f_s: f64,
) {
    let mut content = String::new();
    content.push_str("Q3 Time-Domain Demodulation Results\n");
    content.push_str("=====================================\n\n");
    content.push_str(&format!("Carrier frequency: f_d = {:.4} Hz\n", f_d));
    content.push_str(&format!("Sampling frequency: f_s = {:.4} Hz\n\n", f_s));

    // Spectral peaks for each stage
    content.push_str("Spectral Analysis:\n");
    content.push_str("------------------\n");
    
    // Original signal peak
    let orig_peak = original.iter()
        .filter(|(f, _)| *f > 1000.0)
        .max_by(|(_, mag1), (_, mag2)| mag1.partial_cmp(mag2).unwrap())
        .unwrap();
    content.push_str(&format!("Original signal X(f) peak: f = {:.2} Hz, magnitude = {:.6}\n", 
        orig_peak.0, orig_peak.1));
    
    // After high-pass
    let xh_peak = xh.iter()
        .filter(|(f, _)| *f > 1000.0)
        .max_by(|(_, mag1), (_, mag2)| mag1.partial_cmp(mag2).unwrap())
        .unwrap();
    content.push_str(&format!("After high-pass X_h(f) peak: f = {:.2} Hz, magnitude = {:.6}\n", 
        xh_peak.0, xh_peak.1));
    
    // After multiplication
    let xb_low_peak = xb.iter()
        .filter(|(f, _)| *f > 10.0 && *f < 5000.0)
        .max_by(|(_, mag1), (_, mag2)| mag1.partial_cmp(mag2).unwrap())
        .unwrap();
    content.push_str(&format!("After multiplication X_b(f) peak (baseband): f = {:.2} Hz, magnitude = {:.6}\n", 
        xb_low_peak.0, xb_low_peak.1));
    
    // Demodulated signal peak
    let xl_peak = xl.iter()
        .filter(|(f, _)| *f > 10.0 && *f < 4000.0)
        .max_by(|(_, mag1), (_, mag2)| mag1.partial_cmp(mag2).unwrap())
        .unwrap();
    content.push_str(&format!("Demodulated signal X_l(f) peak: f = {:.2} Hz, magnitude = {:.6}\n", 
        xl_peak.0, xl_peak.1));
    
    // Energy in baseband (0-4000 Hz)
    let energy_orig_baseband: f64 = original.iter()
        .filter(|(f, _)| *f < 4000.0)
        .map(|(_, m)| m * m)
        .sum();
    let energy_demod_baseband: f64 = xl.iter()
        .filter(|(f, _)| *f < 4000.0)
        .map(|(_, m)| m * m)
        .sum();
    
    content.push_str(&format!("\nEnergy analysis (0-4000 Hz band):\n"));
    content.push_str(&format!("  Original signal energy: {:.6e}\n", energy_orig_baseband));
    content.push_str(&format!("  Demodulated signal energy: {:.6e}\n", energy_demod_baseband));
    
    // Frequency shift verification
    content.push_str(&format!("\nFrequency shift verification:\n"));
    content.push_str(&format!("  Original peak at: {:.2} Hz\n", orig_peak.0));
    content.push_str(&format!("  Expected shift: {:.2} Hz (should be near f_d = {:.2} Hz)\n", 
        orig_peak.0 - f_d, f_d));
    content.push_str(&format!("  Demodulated peak at: {:.2} Hz (should be in baseband)\n", xl_peak.0));

    std::fs::write("output/Q3_results.txt", content).expect("Failed to save results");
    println!("  Saved to: output/Q3_results.txt");
}
