mod types;

pub mod logger;
pub use self::types::*;
pub mod iter;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
