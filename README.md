# quixutils-rs

Common helpers and utils

### Current features

- Drop in replacement alias type for `Result<T, E>` => `Result<T, E=anyhow::Error>`
- Logging initializers
- Error helpers
- Common iterators
- Future based timer helpers

#### Removed (for now) in 0.12x

- ApiError
- actix & actix web utils
- tide utils

### Cargo features

- __tokio_utils__: Futures and tokio utils
