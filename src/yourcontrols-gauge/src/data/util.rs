use std::time::Instant;

use yourcontrols_types::{DatumKey, DatumValue, Time};

pub struct DeltaTimeChange {
    current_time: Time,
    instant: Instant,
}

impl DeltaTimeChange {
    pub fn new(start_time: Time) -> Self {
        Self {
            current_time: start_time,
            instant: Instant::now(),
        }
    }

    pub fn step(&mut self) -> Time {
        self.current_time += self.instant.elapsed().as_secs_f64();
        self.instant = Instant::now();
        self.current_time
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;

    #[test]
    fn test_delta_change() {
        let mut delta_time = DeltaTimeChange::new(1.0);
        assert_eq!(delta_time.current_time, 1.0);

        sleep(Duration::from_millis(500));
        assert!(delta_time.step() > 1.5);
        assert!(delta_time.step() < 2.0);
    }
}