
extern crate crypto_abstract;
extern crate serde;

mod internal;
pub mod sym;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}