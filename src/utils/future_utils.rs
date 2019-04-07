use std::future::Future;
use std::time::{Duration, Instant};
use tokio::timer::Delay;

pub fn sleep_async(dur: Duration) -> impl Future<Output = Result<(), tokio::timer::Error>> {
    use tokio_async_await::compat::forward::IntoAwaitable;
    Delay::new(Instant::now() + dur).into_awaitable()
}

pub fn sleep_ms_async(ms: u64) -> impl Future<Output = Result<(), tokio::timer::Error>> {
    sleep_async(Duration::from_millis(ms))
}
