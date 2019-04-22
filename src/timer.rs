use std::future::Future;
use std::time::{Duration, Instant};
use tokio::timer::Delay;

pub fn delay(dur: Duration) -> impl Future<Output = Result<(), tokio::timer::Error>> {
    use futures::compat::Future01CompatExt;
    Delay::new(Instant::now() + dur).compat()
}

pub fn delay_ms(ms: u64) -> impl Future<Output = Result<(), tokio::timer::Error>> {
    delay(Duration::from_millis(ms))
}
