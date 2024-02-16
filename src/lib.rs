use tokio::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub struct Stopwatch {
    /// Time of start of the stopwatch, ['None'](Option::None) if it has never started.
    start_time: Option<Instant>,
    /// Time of last time split.
    last_split: Option<Instant>,
    /// Total time elapsed from start to stop. Is 0 if stopped.
    elapsed: Duration,
}

impl Default for Stopwatch {
    fn default() -> Self {
        Stopwatch {
            start_time: None,
            last_split: None,
            elapsed: Duration::from_secs(0),
        }
    }
}

impl std::fmt::Display for Stopwatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}ms", self.elapsed.as_millis())
    }
}

impl Stopwatch {
    /// Returns an instance of a [`Stopwatch`] with default values.
    pub fn new() -> Stopwatch {
        Default::default()
    }

    /// Begins the timing.
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.last_split = None;
        self.elapsed = Duration::from_secs(0);
    }

    /// Returnt an instance of a started [`Stopwatch`] at now.
    pub fn start_new() -> Stopwatch {
        let mut sw: Stopwatch = Default::default();
        sw.start();
        sw
    }

    /// Halts the timing.
    /// Does nothing if not started.
    pub fn stop(&mut self) -> Duration{
        match self.start_time {
            Some(t1) => {
                self.elapsed = t1.elapsed();
                self.start_time = None;
                self.last_split = None;
                self.elapsed
            },
            None => Duration::from_secs(0),
        }
    }

    /// Resets all values to default.
    pub fn reset(&mut self) {
        *self = Default::default()
    }

    /// Resets values to default and starts timing again.
    pub fn restart(&mut self) {
        *self = Default::default();
        self.start();
    }

    /// Records the elapsed time without stopping the stopwatch.
    pub fn split(&mut self) -> Option<Duration> {
        match self.start_time {
            Some(t1) => {
                self.last_split = Some(Instant::now());
                self.elapsed = t1.elapsed();
                Some(self.elapsed)
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::{time::Duration, thread::sleep};
    use super::Stopwatch;

    #[test]
    fn test_stopwatch_starts() {
        let mut sw = Stopwatch::new();
        sw.start();
        assert_ne!(sw.start_time, None);
    }

    #[test]
    fn test_start_new_sets_correct_values_and_starts() {
        let sw = Stopwatch::start_new();
        assert_ne!(sw.start_time, None);
        assert_eq!(sw.last_split, None);
        assert_eq!(sw.elapsed, Duration::from_secs(0));
    }

    #[test]
    fn test_new_sets_correct_values() {
        let sw = Stopwatch::new();
        assert_eq!(sw.start_time, None);
        assert_eq!(sw.last_split, None);
        assert_eq!(sw.elapsed, Duration::from_secs(0));
    }

    #[test]
    fn test_split_splits() {
        let mut sw = Stopwatch::start_new();
        sw.split();
        assert_ne!(sw.last_split, None);
    }
    
    #[test]
    fn test_split_dont_split_if_stopped() {
        let mut sw = Stopwatch::new();
        sw.split();
        assert_eq!(sw.last_split, None);
    }

    #[test]
    fn test_stop_resets_instants() {
        let mut sw = Stopwatch::new();
        sw.start();
        sw.stop();
        assert_eq!(sw.start_time, None);
        assert_eq!(sw.last_split, None);
    }

    #[test]
    fn test_stop_saves_elapsed_time() {
        let mut sw = Stopwatch::new();
        sw.start();
        sleep(Duration::from_millis(50));
        sw.stop();
        assert!(sw.elapsed.as_millis() >= 50);
    }

    #[test]
    fn test_reset_sets_correct_values() {
        let mut sw = Stopwatch::start_new();
        sw.split();
        sw.reset();

        assert_eq!(sw.start_time, None);
        assert_eq!(sw.last_split, None);

        sleep(Duration::from_millis(10));

        sw.stop();
        sw.reset();
        assert_eq!(sw.elapsed, Duration::from_secs(0));
    }

    #[test]
    fn test_restart_sets_correct_values_and_starts() {
        let mut sw1 = Stopwatch::start_new();
        sw1.split();

        let mut sw2 = sw1.clone();

        sw2.restart();

        assert_ne!(sw2.start_time, sw1.start_time);
        assert_eq!(sw2.last_split, None);

        // Stopwatch needs to stop in order for elapsed to be different than 0
        sleep(Duration::from_millis(10));

        sw1.stop();
        sw1.restart();
        assert_eq!(sw1.elapsed, Duration::from_secs(0));
    }
}
