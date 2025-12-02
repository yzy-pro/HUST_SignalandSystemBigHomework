mod audio_reader;
mod iir_filter;
mod demodulator;
mod spectrum_analyzer;
mod audio_writer;

use std::f64::consts::PI;

use plotters::prelude::*;

fn main() {
    println!("Q3: Error Analysis (Incorrect Processing Order)");
    println!("================================================");

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

    // Create output directory
    std::fs::create_dir_all("output").expect("Failed to create output directory");

    // ========================================================================
    // Case 0: High-Pass -> Multiply -> Low-Pass (Correct Scheme)
    // ========================================================================
    println!("\n=== Case 0: High-Pass -> Multiply -> Low-Pass (Correct Scheme) ===");
    
    // 1. High-Pass Filter (fc = 3000 Hz)
    println!("  1. Applying High-Pass Filter...");
    let c0_step1 = iir_filter::apply_filter(&audio_samples, &hp_b, &hp_a);
    
    // 2. Multiply with Carrier
    println!("  2. Multiplying with Carrier...");
    let c0_step2 = demodulator::multiply_with_carrier(&c0_step1, f_d, f_s);
    
    // 3. Low-Pass Filter (fc = 4000 Hz)
    println!("  3. Applying Low-Pass Filter...");
    let c0_result = iir_filter::apply_filter(&c0_step2, &lp_b, &lp_a);
    
    // Save result
    println!("  Saving result to output/Q3_correct_scheme.wav");
    audio_writer::write_wav("output/Q3_correct_scheme.wav", &c0_result, f_s as u32).unwrap();

    // ========================================================================
    // Case 1: Low-Pass -> Multiply -> High-Pass (Scheme B in paper)
    // ========================================================================
    println!("\n=== Case 1: Low-Pass -> Multiply -> High-Pass (Scheme B) ===");
    
    // 1. Low-Pass Filter (fc = 4000 Hz)
    println!("  1. Applying Low-Pass Filter...");
    let c1_step1 = iir_filter::apply_filter(&audio_samples, &lp_b, &lp_a);
    
    // 2. Multiply with Carrier
    println!("  2. Multiplying with Carrier...");
    let c1_step2 = demodulator::multiply_with_carrier(&c1_step1, f_d, f_s);
    
    // 3. High-Pass Filter (fc = 3000 Hz)
    println!("  3. Applying High-Pass Filter...");
    let c1_result = iir_filter::apply_filter(&c1_step2, &hp_b, &hp_a);
    
    // Save result
    println!("  Saving result to output/Q3_error_case1.wav");
    audio_writer::write_wav("output/Q3_error_case1.wav", &c1_result, f_s as u32).unwrap();
    
    // Plot spectrum
    println!("  Plotting spectrum...");
    let c1_spectrum = spectrum_analyzer::compute_spectrum(&c1_result, f_s);
    spectrum_analyzer::plot_spectrum(&c1_spectrum, "output/Q3_error_case1_spectrum.png", "Error Case 1: LP -> Mult -> HP");

    // ========================================================================
    // Case 2: Multiply -> High-Pass -> Low-Pass (Scheme C in paper)
    // ========================================================================
    println!("\n=== Case 2: Multiply -> High-Pass -> Low-Pass (Scheme C) ===");
    
    // 1. Multiply with Carrier
    println!("  1. Multiplying with Carrier...");
    let c2_step1 = demodulator::multiply_with_carrier(&audio_samples, f_d, f_s);
    
    // 2. High-Pass Filter (fc = 3000 Hz)
    println!("  2. Applying High-Pass Filter...");
    let c2_step2 = iir_filter::apply_filter(&c2_step1, &hp_b, &hp_a);
    
    // 3. Low-Pass Filter (fc = 4000 Hz)
    println!("  3. Applying Low-Pass Filter...");
    let c2_result = iir_filter::apply_filter(&c2_step2, &lp_b, &lp_a);
    
    // Save result
    println!("  Saving result to output/Q3_error_case2.wav");
    audio_writer::write_wav("output/Q3_error_case2.wav", &c2_result, f_s as u32).unwrap();
    
    // Plot spectrum
    println!("  Plotting spectrum...");
    let c2_spectrum = spectrum_analyzer::compute_spectrum(&c2_result, f_s);
    spectrum_analyzer::plot_spectrum(&c2_spectrum, "output/Q3_error_case2_spectrum.png", "Error Case 2: Mult -> HP -> LP");

    // ========================================================================
    // Case 3: Multiply -> Low-Pass (Skip High-Pass Filter)
    // ========================================================================
    println!("\n=== Case 3: Multiply -> Low-Pass (Skip HPF) ===");
    
    // 1. Multiply with Carrier
    println!("  1. Multiplying with Carrier...");
    let c3_step1 = demodulator::multiply_with_carrier(&audio_samples, f_d, f_s);
    
    // 2. Low-Pass Filter (fc = 4000 Hz)
    println!("  2. Applying Low-Pass Filter...");
    let c3_result = iir_filter::apply_filter(&c3_step1, &lp_b, &lp_a);
    
    // Save result
    println!("  Saving result to output/Q3_error_case3.wav");
    audio_writer::write_wav("output/Q3_error_case3.wav", &c3_result, f_s as u32).unwrap();
    
    // Plot spectrum
    println!("  Plotting spectrum...");
    let c3_spectrum = spectrum_analyzer::compute_spectrum(&c3_result, f_s);
    spectrum_analyzer::plot_spectrum(&c3_spectrum, "output/Q3_error_case3_spectrum.png", "Error Case 3: Mult -> LP (No HPF)");

    // ========================================================================
    // Comparison Plot
    // ========================================================================
    println!("\n=== Generating Comparison Plot ===");
    plot_waveform_comparison(&c0_result, &c1_result, &c2_result, &c3_result, f_s, "output/Q3_error_comparison.png");

    println!("\nError analysis completed!");
}

fn plot_waveform_comparison(correct: &[f64], case1: &[f64], case2: &[f64], case3: &[f64], fs: f64, filename: &str) {
    let root = BitMapBackend::new(filename, (1200, 800)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Plot first 0.02 seconds (enough to see waveform details)
    let duration = 0.02;
    let samples_to_plot = (duration * fs) as usize;
    let samples_to_plot = samples_to_plot.min(correct.len()).min(case1.len()).min(case2.len()).min(case3.len());

    // Find min/max across all signals for automatic y-axis scaling
    let mut y_min = f64::INFINITY;
    let mut y_max = f64::NEG_INFINITY;
    
    for i in 0..samples_to_plot {
        let values = [correct[i], case1[i], case2[i], case3[i]];
        for &v in &values {
            if v < y_min { y_min = v; }
            if v > y_max { y_max = v; }
        }
    }
    
    // Add 10% margin
    let y_range = y_max - y_min;
    let margin = y_range * 0.1;
    y_min -= margin;
    y_max += margin;
    
    // Ensure at least some range if signal is constant
    if y_range < 1e-10 {
        y_min = -0.1;
        y_max = 0.1;
    }

    let mut chart = ChartBuilder::on(&root)
        .caption("Demodulation Schemes Comparison (Waveform)", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..duration, y_min..y_max)
        .unwrap();

    chart.configure_mesh()
        .x_desc("Time (s)")
        .y_desc("Amplitude")
        .draw()
        .unwrap();

    // Correct Scheme (Green)
    chart.draw_series(LineSeries::new(
        (0..samples_to_plot).map(|i| (i as f64 / fs, correct[i])),
        &GREEN,
    ))
    .unwrap()
    .label("Correct (HP->Mult->LP)")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    // Case 1 (Red)
    chart.draw_series(LineSeries::new(
        (0..samples_to_plot).map(|i| (i as f64 / fs, case1[i])),
        &RED,
    ))
    .unwrap()
    .label("Case 1 (LP->Mult->HP)")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // Case 2 (Blue)
    chart.draw_series(LineSeries::new(
        (0..samples_to_plot).map(|i| (i as f64 / fs, case2[i])),
        &BLUE,
    ))
    .unwrap()
    .label("Case 2 (Mult->HP->LP)")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Case 3 (Magenta)
    chart.draw_series(LineSeries::new(
        (0..samples_to_plot).map(|i| (i as f64 / fs, case3[i])),
        &MAGENTA,
    ))
    .unwrap()
    .label("Case 3 (Mult->LP)")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));

    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();
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
