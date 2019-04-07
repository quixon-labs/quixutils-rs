use failure::Error;
use std::result::{Result as StdResult};

/// Convenience alias for result with `failure::Error`
/// as default when E isn't provided. Fully compatible
/// with std Result when E is provided, and is a drop-in
/// replacement.
pub type Result<T, E = Error> = StdResult<T, E>;
