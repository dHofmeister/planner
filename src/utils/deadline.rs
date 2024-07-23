use std::time::{Duration, Instant};

/// A struct to manage deadlines for operations.
pub struct Deadline {
    /// The start time of the operation.
    start: Instant,
    /// The end time (deadline) of the operation.
    end: Instant,
    /// The number of ticks (iterations) that have occurred.
    ticks: usize,
}

impl Deadline {
    /// Creates a new Deadline instance.
    ///
    /// # Arguments
    ///
    /// * `duration` - The duration of the deadline in milliseconds.
    ///
    /// # Returns
    ///
    /// A new Deadline instance.
    pub fn new(duration: f32) -> Self {
        let start = Instant::now();
        let end = start + Duration::from_secs_f32(duration / 1000.);
        Deadline {
            start,
            end,
            ticks: 0,
        }
    }

    /// Predicts if the next iteration will exceed the deadline.
    ///
    /// This is useful if you want to terminate 1 iteration before the deadline.
    ///
    /// # Returns
    ///
    /// `true` if the next iteration is predicted to exceed the deadline, `false` otherwise.
    pub fn will_exceed_deadline(&self) -> bool {
        let now = Instant::now();

        // Deadline passed
        if now >= self.end {
            log::debug!("Deadline passed");
            return true;
        }

        // Return if not even half of the duration has passed
        if (now - self.start) < ((self.end - self.start) / 2) {
            return false;
        }

        let elapsed = now - self.start;
        let total_duration = self.end - self.start;
        let avg_tick_duration = elapsed.div_f64(self.ticks as f64);
        let will_exceed = avg_tick_duration > (total_duration - elapsed);

        // WARNING: This is not guarranteed, it may still miscalculate and allow the deadline be
        // missed
        if will_exceed {
            log::debug!("No more iterations possible: elapsed: {:.2?}, total_allowed_duration: {:.2?}, tick_duration: {:.2?}, n_ticks: {:?}", 
                elapsed, total_duration, avg_tick_duration, self.ticks)
        }

        will_exceed
    }

    /// Checks if the deadline has expired.
    ///
    /// # Returns
    ///
    /// `true` if the current time is past the deadline, `false` otherwise.
    pub fn is_expired(&self) -> bool {
        Instant::now() > self.end
    }

    /// Calculates the remaining time until the deadline.
    ///
    /// # Returns
    ///
    /// A Duration representing the time left until the deadline.
    pub fn remaining(&self) -> Duration {
        self.end.saturating_duration_since(Instant::now())
    }

    /// Increments the tick count.
    ///
    /// This should be called at each iteration of the operation being timed.
    pub fn tick(&mut self) {
        self.ticks += 1;
    }
}
