use std::collections::VecDeque;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct RateLimiter {
    requests: VecDeque<Instant>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            requests: VecDeque::new(),
            max_requests,
            window,
        }
    }

    /// Check that we can make a request
    pub fn can_make_request(&mut self) -> bool {
        self.cleanup_old_requests();
        self.requests.len() < self.max_requests
    }

    /// Record a request
    pub fn record_request(&mut self) {
        self.cleanup_old_requests();
        self.requests.push_back(Instant::now());
    }

    /// Compute waiting time for the next query
    pub fn time_until_next_request(&mut self) -> Option<Duration> {
        self.cleanup_old_requests();

        if self.requests.len() < self.max_requests {
            return None;
        }

        if let Some(&oldest) = self.requests.front() {
            let elapsed = oldest.elapsed();
            if elapsed < self.window {
                Some(self.window - elapsed)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn cleanup_old_requests(&mut self) {
        let cutoff = Instant::now() - self.window;
        while let Some(&front) = self.requests.front() {
            if front < cutoff {
                self.requests.pop_front();
            } else {
                break;
            }
        }
    }
}
