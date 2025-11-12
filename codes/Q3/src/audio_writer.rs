use hound;

pub fn write_wav(filename: &str, samples: &[f64], sample_rate: u32) -> Result<(), String> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(filename, spec)
        .map_err(|e| format!("Failed to create WAV file: {}", e))?;

    // Normalize samples to prevent clipping
    let max_val = samples.iter().fold(0.0f64, |max, &x| max.max(x.abs()));
    let scale = if max_val > 0.0 {
        0.95 / max_val // Leave some headroom
    } else {
        1.0
    };

    for &sample in samples {
        let normalized = sample * scale;
        let sample_i16 = (normalized * 32767.0).clamp(-32768.0, 32767.0) as i16;
        writer
            .write_sample(sample_i16)
            .map_err(|e| format!("Failed to write sample: {}", e))?;
    }

    writer
        .finalize()
        .map_err(|e| format!("Failed to finalize WAV file: {}", e))?;

    Ok(())
}
