use hound;

pub fn read_wav(filename: &str) -> Result<Vec<f64>, String> {
    let reader = hound::WavReader::open(filename)
        .map_err(|e| format!("Failed to open WAV file: {}", e))?;

    let spec = reader.spec();
    println!("  Sample rate: {} Hz", spec.sample_rate);
    println!("  Channels: {}", spec.channels);
    println!("  Bits per sample: {}", spec.bits_per_sample);

    // Read all samples and normalize to [-1.0, 1.0]
    let samples: Vec<f64> = match spec.bits_per_sample {
        16 => {
            reader
                .into_samples::<i16>()
                .map(|s| s.unwrap() as f64 / 32768.0)
                .collect()
        }
        _ => {
            return Err(format!(
                "Unsupported bits per sample: {}",
                spec.bits_per_sample
            ))
        }
    };

    Ok(samples)
}
