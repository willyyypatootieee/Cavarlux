use std::collections::HashMap;

pub struct AudioOptimizer;

impl AudioOptimizer {
    pub fn get_low_latency_audio_env() -> HashMap<String, String> {
        let mut envs = HashMap::new();
        envs.insert("PULSE_LATENCY_MSEC".to_string(), "30".to_string());
        envs.insert("PIPEWIRE_LATENCY".to_string(), "256/48000".to_string());
        envs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_env() {
        let envs = AudioOptimizer::get_low_latency_audio_env();
        assert_eq!(envs.get("PULSE_LATENCY_MSEC").unwrap(), "30");
    }
}
