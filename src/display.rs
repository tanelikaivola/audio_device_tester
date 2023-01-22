pub trait CPALString {
    fn to_string(&self) -> String;
}

impl CPALString for cpal::SampleFormat {
    fn to_string(&self) -> String {
        match self {
            cpal::SampleFormat::I16 => "16-bit signed integer",
            cpal::SampleFormat::U16 => "16-bit unsigned integer",
            cpal::SampleFormat::F32 => "32-bit float",
        }
        .to_string()
    }
}

impl CPALString for cpal::SupportedStreamConfigRange {
    fn to_string(&self) -> String {
        if self.min_sample_rate().0 == self.max_sample_rate().0 {
            format!(
                "{} Hz, {} channel(s), {}",
                self.min_sample_rate().0,
                self.channels(),
                self.sample_format().to_string()
            )
        } else {
            format!(
                "{} - {} Hz, {} channel(s), {}",
                self.min_sample_rate().0,
                self.max_sample_rate().0,
                self.channels(),
                self.sample_format().to_string()
            )
        }
    }
}

impl CPALString for cpal::SupportedStreamConfig {
    fn to_string(&self) -> String {
        format!(
            " {} Hz, {} channel(s), {}",
            self.sample_rate().0,
            self.channels(),
            self.sample_format().to_string()
        )
    }
}
