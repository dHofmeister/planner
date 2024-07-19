use std::time::{Duration, Instant};

pub struct Deadline {
    start: Instant,
    end: Instant,
    ticks: usize,
}

impl Deadline {
    pub fn new(duration: f32) -> Self {
        let start = Instant::now();
        let end = start + Duration::from_secs_f32(duration / 1000.);
        Deadline {
            start,
            end,
            ticks: 0,
        }
    }

    // Tries to predict if the next (t+1) will cross the deadline
    // Useful if you want to terminate 1 iteration before
    pub fn will_exceed_deadline(&self) -> bool {
        let now = Instant::now();

        // INFO: Deadline passed
        if now >= self.end {
            log::debug!("Deadline passed");
            return true;
        }

        // INFO: Return if not even half of the duration has passed
        if (now - self.start) < ((self.end - self.start) / 2) {
            return false;
        }

        let elapsed = now - self.start;
        let total_duration = self.end - self.start;
        let avg_tick_duration = elapsed.div_f64(self.ticks as f64);
        let will_exceed = avg_tick_duration > (total_duration - elapsed);
        if will_exceed {
            log::debug!("No more iterations possible: elapsed: {:.2?}, total_allowed_duration: {:.2?}, tick_duration: {:.2?}, n_ticks: {:?}", 
                elapsed, total_duration, avg_tick_duration, self.ticks)
        }

        will_exceed
    }

    pub fn is_expired(&self) -> bool {
        Instant::now() > self.end
    }

    pub fn remaining(&self) -> Duration {
        self.end.saturating_duration_since(Instant::now())
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
    }
}
