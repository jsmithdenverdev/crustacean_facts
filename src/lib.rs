pub mod database;
pub mod distribution;
pub mod errors;
pub mod facts;
pub mod subscribers;

pub type Result<T, E = errors::Error> = std::result::Result<T, E>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
