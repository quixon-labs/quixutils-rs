use anyhow::Error;
use std::result::Result as StdResult;

/// Convenience alias for result with `anyhow::Error`
/// as default when E isn't provided. Fully compatible
/// with std Result when E is provided, and is a drop-in
/// replacement.
/// 
/// NOTE: This is only here for migration purposes when
/// this was based on `failure`.
///  
/// `anyhow` now provides it's own alias that can be 
/// used when needed.
pub type Result<T, E = Error> = StdResult<T, E>;
