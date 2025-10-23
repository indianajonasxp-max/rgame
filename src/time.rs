//! Time management and delta time tracking
//!
//! Provides utilities for tracking frame time, delta time, and FPS.

use std::time::{Duration, Instant};

/// Manages time-related functionality for the engine
#[derive(Debug)]
pub struct TimeManager {
    start_time: Instant,
    last_frame: Instant,
    delta_time: Duration,
    frame_count: u64,
    fps: f32,
    fps_timer: Duration,
    fps_frame_count: u32,
}

impl TimeManager {
    /// Create a new TimeManager
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_frame: now,
            delta_time: Duration::from_secs(0),
            frame_count: 0,
            fps: 0.0,
            fps_timer: Duration::from_secs(0),
            fps_frame_count: 0,
        }
    }

    /// Update the time manager (call once per frame)
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now - self.last_frame;
        self.last_frame = now;
        self.frame_count += 1;

        // Update FPS counter every second
        self.fps_timer += self.delta_time;
        self.fps_frame_count += 1;

        if self.fps_timer >= Duration::from_secs(1) {
            self.fps = self.fps_frame_count as f32 / self.fps_timer.as_secs_f32();
            self.fps_timer = Duration::from_secs(0);
            self.fps_frame_count = 0;
        }
    }

    /// Get delta time since last frame in seconds
    pub fn delta_time(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }

    /// Get delta time as a Duration
    pub fn delta_duration(&self) -> Duration {
        self.delta_time
    }

    /// Get total elapsed time since engine start
    pub fn elapsed(&self) -> Duration {
        Instant::now() - self.start_time
    }

    /// Get total elapsed time in seconds
    pub fn elapsed_secs(&self) -> f32 {
        self.elapsed().as_secs_f32()
    }

    /// Get current frames per second
    pub fn fps(&self) -> f32 {
        self.fps
    }

    /// Get total frame count
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    /// Reset the time manager
    pub fn reset(&mut self) {
        let now = Instant::now();
        self.start_time = now;
        self.last_frame = now;
        self.delta_time = Duration::from_secs(0);
        self.frame_count = 0;
        self.fps = 0.0;
        self.fps_timer = Duration::from_secs(0);
        self.fps_frame_count = 0;
    }
}

impl Default for TimeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_time_manager() {
        let mut time = TimeManager::new();
        thread::sleep(Duration::from_millis(16));
        time.update();
        assert!(time.delta_time() > 0.0);
    }
}
