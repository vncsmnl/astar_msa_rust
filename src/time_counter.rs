/*!
 * \author Vinícius Manoel
 * \copyright MIT License
 *
 * \brief Time counter utility for performance measurement
 */

use std::time::Instant;

pub struct TimeCounter {
    name: String,
    start: Instant,
}

impl TimeCounter {
    pub fn new(name: &str) -> Self {
        println!("{}", name);
        TimeCounter {
            name: name.to_string(),
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.start.elapsed()
    }

    pub fn elapsed_secs(&self) -> f64 {
        self.elapsed().as_secs_f64()
    }
}

impl Drop for TimeCounter {
    fn drop(&mut self) {
        let duration = self.elapsed();
        println!("{} completed in {:.3}s", self.name, duration.as_secs_f64());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_counter() {
        let tc = TimeCounter::new("Test timer");
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(tc.elapsed_secs() >= 0.01);
    }
}
