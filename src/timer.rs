use std::future::Future;
use std::time::{Duration, Instant};
use tokio::timer::Delay;

pub fn delay(dur: Duration) -> impl Future<Output = Result<(), tokio::timer::Error>> {
    use tokio_async_await::compat::forward::IntoAwaitable;
    Delay::new(Instant::now() + dur).into_awaitable()
}

pub fn delay_ms(ms: u64) -> impl Future<Output = Result<(), tokio::timer::Error>> {
    delay(Duration::from_millis(ms))
}
