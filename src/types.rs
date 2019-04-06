use failure::Error;
use std::result::Result;

/// Convenience alias for result.
/// NOTE: Let's not use this anymore - Result is just
/// explicit and easier to reason with despite the
/// longer name.
pub type ResultAs<T, E = Error> = Result<T, E>;
