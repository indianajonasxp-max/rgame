//! Utility functions and helpers
//!
//! Common utilities used throughout the engine

use std::time::{SystemTime, UNIX_EPOCH};

/// Generate a random seed based on current time
pub fn generate_seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

/// Simple pseudo-random number generator (LCG)
pub struct Random {
    state: u64,
}

impl Random {
    /// Create a new random number generator with a seed
    pub fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    /// Create with time-based seed
    pub fn from_time() -> Self {
        Self::new(generate_seed())
    }

    /// Generate next random value
    fn next(&mut self) -> u64 {
        // Linear congruential generator
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }

    /// Generate random f32 between 0.0 and 1.0
    pub fn gen_f32(&mut self) -> f32 {
        (self.next() >> 32) as f32 / u32::MAX as f32
    }

    /// Generate random f32 in range
    pub fn gen_range_f32(&mut self, min: f32, max: f32) -> f32 {
        min + self.gen_f32() * (max - min)
    }

    /// Generate random i32 in range
    pub fn gen_range_i32(&mut self, min: i32, max: i32) -> i32 {
        min + (self.next() % (max - min) as u64) as i32
    }

    /// Generate random boolean
    pub fn gen_bool(&mut self) -> bool {
        (self.next() & 1) == 1
    }
}

/// Timer for tracking elapsed time
pub struct Timer {
    duration: f32,
    elapsed: f32,
    repeating: bool,
    active: bool,
}

impl Timer {
    /// Create a new timer
    pub fn new(duration: f32, repeating: bool) -> Self {
        Self {
            duration,
            elapsed: 0.0,
            repeating,
            active: true,
        }
    }

    /// Create a one-shot timer
    pub fn once(duration: f32) -> Self {
        Self::new(duration, false)
    }

    /// Create a repeating timer
    pub fn repeating(duration: f32) -> Self {
        Self::new(duration, true)
    }

    /// Update the timer
    pub fn update(&mut self, delta: f32) -> bool {
        if !self.active {
            return false;
        }

        self.elapsed += delta;

        if self.elapsed >= self.duration {
            if self.repeating {
                self.elapsed -= self.duration;
            } else {
                self.active = false;
            }
            return true;
        }

        false
    }

    /// Reset the timer
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.active = true;
    }

    /// Get progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        (self.elapsed / self.duration).min(1.0)
    }

    /// Check if timer is finished
    pub fn is_finished(&self) -> bool {
        !self.active
    }

    /// Get remaining time
    pub fn remaining(&self) -> f32 {
        (self.duration - self.elapsed).max(0.0)
    }
}

/// Easing functions for smooth animations
pub mod easing {
    /// Linear interpolation (no easing)
    pub fn linear(t: f32) -> f32 {
        t
    }

    /// Ease in (quadratic)
    pub fn ease_in(t: f32) -> f32 {
        t * t
    }

    /// Ease out (quadratic)
    pub fn ease_out(t: f32) -> f32 {
        t * (2.0 - t)
    }

    /// Ease in-out (quadratic)
    pub fn ease_in_out(t: f32) -> f32 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            -1.0 + (4.0 - 2.0 * t) * t
        }
    }

    /// Ease in (cubic)
    pub fn ease_in_cubic(t: f32) -> f32 {
        t * t * t
    }

    /// Ease out (cubic)
    pub fn ease_out_cubic(t: f32) -> f32 {
        let t = t - 1.0;
        t * t * t + 1.0
    }

    /// Bounce effect
    pub fn bounce(t: f32) -> f32 {
        if t < 1.0 / 2.75 {
            7.5625 * t * t
        } else if t < 2.0 / 2.75 {
            let t = t - 1.5 / 2.75;
            7.5625 * t * t + 0.75
        } else if t < 2.5 / 2.75 {
            let t = t - 2.25 / 2.75;
            7.5625 * t * t + 0.9375
        } else {
            let t = t - 2.625 / 2.75;
            7.5625 * t * t + 0.984375
        }
    }

    /// Elastic effect
    pub fn elastic(t: f32) -> f32 {
        if t == 0.0 || t == 1.0 {
            t
        } else {
            let p = 0.3;
            let s = p / 4.0;
            let t = t - 1.0;
            -(2.0f32.powf(10.0 * t) * ((t - s) * (2.0 * std::f32::consts::PI) / p).sin())
        }
    }
}

/// Color utilities
pub mod color_utils {
    use crate::renderer::Color;

    /// Convert HSV to RGB color
    pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
        let h = h % 360.0;
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Color::rgb(r + m, g + m, b + m)
    }

    /// Lerp between two colors
    pub fn lerp(a: Color, b: Color, t: f32) -> Color {
        Color::new(
            a.r + (b.r - a.r) * t,
            a.g + (b.g - a.g) * t,
            a.b + (b.b - a.b) * t,
            a.a + (b.a - a.a) * t,
        )
    }

    /// Generate rainbow color from time
    pub fn rainbow(time: f32) -> Color {
        hsv_to_rgb((time * 60.0) % 360.0, 1.0, 1.0)
    }
}

/// File path utilities
pub mod path_utils {
    use std::path::{Path, PathBuf};

    /// Get asset path relative to executable
    pub fn asset_path<P: AsRef<Path>>(relative_path: P) -> PathBuf {
        let mut path = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        path.push("assets");
        path.push(relative_path);
        path
    }

    /// Check if file exists
    pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists() && path.as_ref().is_file()
    }

    /// Get file extension
    pub fn get_extension<P: AsRef<Path>>(path: P) -> Option<String> {
        path.as_ref()
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
    }
}

/// Performance profiling helpers
pub mod profiling {
    use std::time::Instant;

    /// Simple profiler for measuring code execution time
    pub struct Profiler {
        start: Instant,
        name: String,
    }

    impl Profiler {
        /// Start profiling a section
        pub fn start(name: impl Into<String>) -> Self {
            Self {
                start: Instant::now(),
                name: name.into(),
            }
        }

        /// Stop profiling and log the result
        pub fn stop(self) {
            let elapsed = self.start.elapsed();
            log::debug!("[PROFILE] {} took {:?}", self.name, elapsed);
        }

        /// Get elapsed time without stopping
        pub fn elapsed(&self) -> std::time::Duration {
            self.start.elapsed()
        }
    }

    /// Profile a function call
    pub fn profile<F, R>(name: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let profiler = Profiler::start(name);
        let result = f();
        profiler.stop();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        let mut rng = Random::new(12345);
        let val = rng.gen_f32();
        assert!(val >= 0.0 && val <= 1.0);
    }

    #[test]
    fn test_timer() {
        let mut timer = Timer::once(1.0);
        assert!(!timer.update(0.5));
        assert!(timer.update(0.6));
        assert!(timer.is_finished());
    }

    #[test]
    fn test_easing() {
        assert_eq!(easing::linear(0.5), 0.5);
        assert!(easing::ease_in(0.5) < 0.5);
        assert!(easing::ease_out(0.5) > 0.5);
    }
}
