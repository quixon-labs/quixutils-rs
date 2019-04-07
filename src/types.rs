use failure::Error;
use std::result::Result;

/// Convenience alias for result with `failure::Error`
/// as default when E isn't provided.
///
/// NOTE: Based on experience, let's not use this anymore
/// Result is just explicit and easier to reason with despite
/// more key strokes.
pub type ResultAs<T, E = Error> = Result<T, E>;
