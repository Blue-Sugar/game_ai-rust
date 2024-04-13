#[allow(unused)]
pub struct TimeKeeper {
    start_time: std::time::Instant,
    time_threshold: f64,
}
#[allow(unused)]
impl TimeKeeper {
    pub fn build(time_threshold: f64) -> Self {
        Self {
            start_time: std::time::Instant::now(),
            time_threshold,
        }
    }

    #[inline]
    pub fn is_time_over(&self) -> bool {
        let elapsed_time = self.start_time.elapsed().as_nanos() as f64 * 1e-9;
        #[cfg(feature = "local")]
        {
            elapsed_time * 0.85 >= self.time_threshold
        }
        #[cfg(not(feature = "local"))]
        {
            elapsed_time >= self.time_threshold
        }
    }
}